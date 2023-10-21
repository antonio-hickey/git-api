use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Error when last() is None
    NoLastElement,
    /// Error involving a failed process command
    CommandFailed,
    /// Error involving a failed cwd change
    DirectoryChangeError,
    /// Error involving invalid UTF-8
    InvalidUtf8,
    /// Error -> std::io::Error
    StdIoError(std::io::Error),
    /// Error for std::string::FromUtf8Error
    FromUtf8(std::string::FromUtf8Error),
}
// Implement display trait for RevereError
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Think of more meaningful display messages
        write!(f, "error")
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::StdIoError(err)
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error::FromUtf8(err)
    }
}
