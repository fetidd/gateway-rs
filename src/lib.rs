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
    ValidationError(String),
    EncodingError(String),

    FieldError(String),
}
type Result<T> = std::result::Result<T, GatewayError>;

macro_rules! map {
    ($($key:expr => $value:expr),+ $(,)?) => {
        ::std::collections::HashMap::from([ $(($key, $value)),* ])
    };
}
pub(crate) use map;
