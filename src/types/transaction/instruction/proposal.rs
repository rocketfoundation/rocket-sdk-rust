use serde::{Deserialize, Serialize};

/// Types of proposals that can be submitted.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ProposalType {
    CreateInstrument,
    CreateValidator,
}

/// Wrapper for proposal-specific data payloads.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ProposalDataType {
    /// Data for creating an instrument.
    CreateInstrument(CreateInstrumentData),
    /// Data for creating a validator.
    CreateValidator(CreateValidatorData),
}

/// Payload for a create proposal instruction.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateProposalData {
    /// The kind of proposal being submitted.
    pub proposal_type: ProposalType,
    /// Associated data for the specified proposal type.
    pub proposal_data: ProposalDataType,
}

/// Data required to create a new instrument.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateInstrumentData {
    /// Human-readable name of the instrument.
    pub instrument_name: String,
    /// Short symbol for the instrument.
    pub instrument_symbol: String,
    /// Description explaining the instrument.
    pub instrument_description: String,
}

/// Data required to propose a new validator.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateValidatorData {
    /// Name of the validator.
    pub validator_name: String,
    /// Description of the validator.
    pub validator_description: String,
}
