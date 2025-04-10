use crate::*;

impl StdError for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TcpBindError(data) => write!(f, "Tcp bind error{}{}", COLON_SPACE, data),
            Self::HttpReadError(data) => write!(f, "Http read error{}{}", COLON_SPACE, data),
            Self::InvalidHttpRequest(data) => {
                write!(f, "Invalid http request{}{}", COLON_SPACE, data)
            }
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl StdError for RouteError {}

impl Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicatePattern(pattern) => {
                write!(f, "Route pattern already exists: {}", pattern)
            }
        }
    }
}
