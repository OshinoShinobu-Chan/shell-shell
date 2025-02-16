use log::error;

pub enum ErrorType {
    UnreachableCode,
    IOError,
    PreRunError,
    PostRunError,
    OtherError,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorType::UnreachableCode => write!(f, "UnreachableCode"),
            ErrorType::IOError => write!(f, "IOError"),
            ErrorType::PreRunError => write!(f, "PreRunError"),
            ErrorType::PostRunError => write!(f, "PostRunError"),
            ErrorType::OtherError => write!(f, "OtherError"),
        }
    }
}

impl std::fmt::Debug for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorType::UnreachableCode => write!(f, "UnreachableCode"),
            ErrorType::IOError => write!(f, "IOError"),
            ErrorType::PreRunError => write!(f, "PreRunError"),
            ErrorType::PostRunError => write!(f, "PostRunError"),
            ErrorType::OtherError => write!(f, "OtherError"),
        }
    }
}

pub struct Error {
    pub message: String,
    pub error_type: ErrorType,
}

impl Error {
    pub fn new(message: String, error_type: ErrorType) -> Self {
        Error {
            message,
            error_type,
        }
    }

    pub fn print(&self) {
        error!("{}", self);
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)
    }
}
