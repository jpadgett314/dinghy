use gtk::glib;
use linkding::LinkDingError;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "DinghyApiError")]
#[repr(i32)]
pub enum ApiError {
    None,
    #[default]
    UnknownFailure,
    Unreachable,
    InvalidUrl,
    Rejected,
}

impl From<&LinkDingError> for ApiError {
    fn from(error: &LinkDingError) -> Self {
        match &error {
            LinkDingError::ParseUrl(_) => Self::InvalidUrl,
            LinkDingError::SendHttpError(http_error) => {
                if http_error.is_connect() || http_error.is_timeout() {
                    Self::Unreachable
                } else if http_error.status().is_some_and(|s| matches!(s.as_u16(), 401 | 403)) {
                    Self::Rejected
                } else {
                    Self::UnknownFailure
                }
            }
            _ => Self::UnknownFailure
        }
    }
}

