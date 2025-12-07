use crate::OutputShape;

#[derive(thiserror::Error, Debug)]
pub enum TaUtilsError {
    #[error("InvalidParameter '{0}' found")]
    InvalidParameter(String),

    /// Unallowed operation, such as trying to modify an immutable reference.
    #[error("Unallowed operation: {0}")]
    Unallowed(String),

    #[error("Unexpected error, {0}")]
    Unexpected(String),
    
    #[error("Incorrect output type, expected {expected}, got {actual}")]
    IncorrectOutputType { expected: String, actual: String },

    #[error("LangError {0}")]
    Lang(String),
    
    #[error("Cmp error, {0}")]
    Cmp(#[from] OutputError),
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum OutputError {
    #[error("Type mismatch")]
    TypeMismatch,
    #[error("Length mismatch between two arrays, array1: {0}, array2: {1}")]
    LengthMismatch(usize, usize),
    #[error("Invalid output shape {0}")]
    InvalidOutputShape(OutputShape),
}

impl PartialEq for TaUtilsError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TaUtilsError::InvalidParameter(a), TaUtilsError::InvalidParameter(b)) => a == b,
            (TaUtilsError::Unallowed(a), TaUtilsError::Unallowed(b)) => a == b,
            (TaUtilsError::Unexpected(a), TaUtilsError::Unexpected(b)) => a == b,
            (TaUtilsError::IncorrectOutputType { expected: e1, actual: a1 }, TaUtilsError::IncorrectOutputType { expected: e2, actual: a2 }) => e1 == e2 && a1 == a2,
            _ => false,
        }
    }
}

pub type TaUtilsResult<T> = Result<T, TaUtilsError>;