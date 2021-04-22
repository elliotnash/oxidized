use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum LoginErrorType {
    InvalidCredentials,
    ConnectionError,
    ServerError
}
impl Display for LoginErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Self::InvalidCredentials => "invalid credentials",
            Self::ConnectionError => "connection error",
            Self::ServerError => "server error"
        })
    }
}

#[derive(Debug, Clone)]
pub struct LoginError{
    pub error_type: LoginErrorType
}

impl Display for LoginError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while logging into guilded.gg: {}!", self.error_type)
    }
}
impl Error for LoginError{}
