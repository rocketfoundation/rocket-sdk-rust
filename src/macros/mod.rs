#[macro_export]
macro_rules! impl_as_ref_mut_newtype {
    ($name:ident, $inner:ty) => {
        impl AsRef<$inner> for $name {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }

        impl AsMut<$inner> for $name {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }
    };
}
