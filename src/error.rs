use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum LoginError {
    InvalidCredentials(InvalidCredentials),
    ConnectionError(ConnectionError),
    ServerError(ServerError)
}
impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while logging into guilded.gg: {}!", self)
    }
}
impl Error for LoginError{}
impl Default for LoginError{
    fn default() -> Self {
        todo!()
    }
}
#[derive(Debug, Clone)]
pub struct InvalidCredentials;
impl Display for InvalidCredentials{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid credentials")
    }
}
impl Error for InvalidCredentials{}
#[derive(Debug, Clone)]
pub struct ConnectionError;
impl Display for ConnectionError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "connection error")
    }
}
impl Error for ConnectionError{}
#[derive(Debug, Clone)]
pub struct ServerError;
impl Display for ServerError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "server error")
    }
}
impl Error for ServerError{}


#[derive(Debug, Clone)]
pub enum HttpError {
    RateLimited(RateLimited),
    ServerError(ServerError)
}
impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while making http request: {}!", self)
    }
}
impl HttpError {
    pub fn from_code(status_code: u16) -> Self {
        match status_code {
            429 => Self::RateLimited(RateLimited{}),
            _ => Self::ServerError(ServerError{})
        }
    }
}
#[derive(Debug, Clone)]
pub struct RateLimited;
impl Display for RateLimited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rate limited")
    }
}
impl Error for RateLimited{}
