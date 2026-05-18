use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorCode {
    Ok = 0,
    Unknown = 1,
    InvalidArgument = 2,
    UnsupportedBackend = 3,
    UnsupportedFormat = 4,
    DeviceMismatch = 5,
    ResourceCreationFailed = 6,
    SharedHandleFailed = 7,
    SenderNotFound = 8,
    SenderClosed = 9,
    Timeout = 10,
    BackendError = 11,
    CommandFailed = 12,
}

impl ErrorCode {
    fn from_raw(raw: i32) -> Self {
        match raw {
            0 => ErrorCode::Ok,
            1 => ErrorCode::Unknown,
            2 => ErrorCode::InvalidArgument,
            3 => ErrorCode::UnsupportedBackend,
            4 => ErrorCode::UnsupportedFormat,
            5 => ErrorCode::DeviceMismatch,
            6 => ErrorCode::ResourceCreationFailed,
            7 => ErrorCode::SharedHandleFailed,
            8 => ErrorCode::SenderNotFound,
            9 => ErrorCode::SenderClosed,
            10 => ErrorCode::Timeout,
            11 => ErrorCode::BackendError,
            12 => ErrorCode::CommandFailed,
            _ => ErrorCode::Unknown,
        }
    }

    fn description(&self) -> &'static str {
        match self {
            ErrorCode::Ok => "no error",
            ErrorCode::Unknown => "unknown error",
            ErrorCode::InvalidArgument => "invalid argument",
            ErrorCode::UnsupportedBackend => "unsupported backend",
            ErrorCode::UnsupportedFormat => "unsupported format",
            ErrorCode::DeviceMismatch => "device mismatch",
            ErrorCode::ResourceCreationFailed => "resource creation failed",
            ErrorCode::SharedHandleFailed => "shared handle operation failed",
            ErrorCode::SenderNotFound => "sender not found",
            ErrorCode::SenderClosed => "sender closed",
            ErrorCode::Timeout => "operation timed out",
            ErrorCode::BackendError => "backend-specific error",
            ErrorCode::CommandFailed => "command execution failed",
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
}

impl Error {
    pub fn new(code: ErrorCode) -> Self {
        Error {
            code,
            message: code.description().to_owned(),
        }
    }

    pub fn with_message(code: ErrorCode, message: impl Into<String>) -> Self {
        Error {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nozzle error ({}): {}", self.code as i32, self.message)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn check(raw: u32) -> Result<()> {
    let code = ErrorCode::from_raw(raw as i32);
    if code == ErrorCode::Ok {
        Ok(())
    } else {
        Err(Error::new(code))
    }
}
