use linkding::LinkDingError;

#[derive(Debug)]
pub enum ApiError {
    Uninitialized,
    Unavailable(LinkDingError),
    RequestFailed(LinkDingError),
}

impl From<LinkDingError> for ApiError {
    fn from(error: LinkDingError) -> Self {
        if let LinkDingError::SendHttpError(http_error) = &error {
            if http_error.is_connect()
                || http_error.is_timeout()
                || http_error.status().is_some_and(|status| status.is_server_error())
            {
                return Self::Unavailable(error);
            }
        }
        Self::RequestFailed(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uninitialized => write!(f, "API is uninitialized"),
            Self::Unavailable(error) => write!(f, "Server unavailable: {error}"),
            Self::RequestFailed(error) => write!(f, "Request failed: {error}"),
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Uninitialized => None,
            Self::Unavailable(error) | Self::RequestFailed(error) => Some(error),
        }
    }
}

