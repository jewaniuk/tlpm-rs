use thiserror::Error;

#[derive(Error, Debug)]
pub enum TlpmError {
    #[error("invalid resource name: {0}")]
    InvalidResourceName(String),

    #[error("visa error {code} during {action}: {message}")]
    VisaError {
        code: i32, // sys::ViStatus is an alias for a 32-bit signed integer
        action: String,
        message: String,
    },

    #[error("failed to parse string from c string: {0}")]
    StringConversion(String),
}
