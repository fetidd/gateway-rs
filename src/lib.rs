pub mod bank;
pub mod merchant;
pub mod messaging_specification;
pub mod operation;
pub mod operation_field;
pub mod payment;
pub mod transaction;
pub mod currency;

#[derive(Debug, Clone, PartialEq)]
pub enum GatewayError {
    /// Error raised when attempting to create an Operation and a field's value is incorrect
    ValidationError(String),
    /// Error raised when an Operation is being encoded and the string parsed from it does not align with the bitmap specification
    EncodingError(String),

    /// Raised when a value for an Operation field is invalid
    FieldError(String),
}
type Result<T> = std::result::Result<T, GatewayError>;

macro_rules! map {
    ($($key:expr => $value:expr),+ $(,)?) => {
        ::std::collections::HashMap::from([ $(($key, $value)),* ])
    };
}
pub(crate) use map;
