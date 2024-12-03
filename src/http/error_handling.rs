use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
pub struct AppError(pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Something went wrong: {}", self.0)
//     }
// }

// impl From<&str> for AppError {
//     fn from(err: &str) -> Self {
//         Self(anyhow::Error::msg(err))
//     }
// }

// impl From<String> for AppError {
//     fn from(err: String) -> Self {
//         Self(anyhow::Error::msg(err))
//     }
// }
