use linkding::LinkDingError;

#[derive(Debug)]
pub enum ApiError {
    Unavailable,
    RequestFailed,
}

impl From<&LinkDingError> for ApiError {
    fn from(error: &LinkDingError) -> Self {
        if let LinkDingError::SendHttpError(http_error) = &error {
            if http_error.is_connect()
                || http_error.is_timeout()
                || http_error.status().is_some_and(|status| status.is_server_error())
            {
                return Self::Unavailable;
            }
        }
        Self::RequestFailed
    }
}

