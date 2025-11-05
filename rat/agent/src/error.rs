use std::fmt;

#[derive(Debug)]
pub enum Error {
    Internal(String),
    Api(String),
    Io(std::io::Error),
    Ssh(ssh2::Error),
    Zip(zip::result::ZipError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
            Error::Api(msg) => write!(f, "API error: {}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Ssh(err) => write!(f, "SSH error: {}", err),
            Error::Zip(err) => write!(f, "ZIP error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::convert::From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<ssh2::Error> for Error {
    fn from(err: ssh2::Error) -> Self {
        Error::Ssh(err)
    }
}

impl std::convert::From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Error::Zip(err)
    }
}
