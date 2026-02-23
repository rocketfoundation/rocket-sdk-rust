use std::{sync::mpsc, thread};
use tungstenite::{connect, Message};

use crate::{
    client::error::ClientError,
    types::ws::{client_message::ClientMessage, server_message::ServerMessage},
};

pub enum Command {
    Send(ClientMessage),
    Close,
}

/// A basic WS client for subscribing to the rocket chain node data stream.
#[derive(Debug)]
pub struct WsClient {
    tx: mpsc::Sender<Command>,
}

impl WsClient {
    /// Create a new instance of the client, connect to the socket and start listening for incoming messages.
    /// Example:
    /// ```
    /// use rocket_chain_sdk::{
    ///     client::ws::WsClient,
    ///     types::ws::server_message::ServerMessage,
    /// };
    ///
    /// fn message_handler(message: ServerMessage) {
    ///     println!("Received server message: {message:?}");
    /// }
    ///
    /// let client_instance = WsClient::connect("ws://127.0.0.1:4000", message_handler);
    /// ```
    pub fn connect<F>(url: &str, handler: F) -> Result<Self, ClientError>
    where
        F: Fn(ServerMessage) + Send + 'static,
    {
        let (mut socket, _) = connect(url)?;
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop {
                // Drain outgoing message queue.
                while let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        Command::Send(msg) => {
                            let serialized_message = match serde_json::to_string(&msg) {
                                Ok(v) => v,
                                Err(err) => {
                                    eprintln!(
                                        "Failed to serialize message: {msg:?}; Error: {err:?}"
                                    );
                                    continue;
                                }
                            };
                            if let Err(err) = socket.send(Message::Text(serialized_message.into()))
                            {
                                eprintln!("Failed to send message to server: {err:?}");
                            }
                        }
                        Command::Close => {
                            let _ = socket.close(None);
                            return;
                        }
                    }
                }

                // Read incoming messages.
                match socket.read() {
                    Ok(Message::Close(frame)) => {
                        let _ = socket.close(frame);
                        println!("Server closed connection");
                        return;
                    }
                    Ok(Message::Text(text_message)) => match serde_json::from_str(&text_message) {
                        Ok(parsed_message) => handler(parsed_message),
                        Err(err) => eprintln!("Failed to parse text message from server: {err:?}"),
                    },
                    Ok(msg) => {
                        println!("Received unexpected message type from server: {msg:?}");
                    }
                    Err(err) => {
                        eprintln!("Error listening to stream: {err:?}");
                        return;
                    }
                }
            }
        });

        Ok(Self { tx })
    }

    /// Send a subscription message.
    pub fn send(&self, msg: ClientMessage) {
        let _ = self.tx.send(Command::Send(msg));
    }

    /// Clone connection and stop the client.
    pub fn close(&self) {
        let _ = self.tx.send(Command::Close);
    }
}
