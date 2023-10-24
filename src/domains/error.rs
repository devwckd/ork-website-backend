use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Clone, Debug, serde::Serialize)]
pub struct ErrorResponse<T> {
    #[serde(skip_serializing)]
    code: u16,
    #[serde(rename = "code")]
    custom_code: String,
    message: T,
}

impl<T> ErrorResponse<T>
where
    T: Serialize,
{
    pub fn of<S>(code: S, message: T) -> Self
    where
        S: Into<StatusCode>,
    {
        let code = code.into();
        Self {
            code: code.as_u16(),
            custom_code: code.as_u16().to_string(),
            message,
        }
    }

    pub fn of_custom<S>(code: S, custom_code: String, message: T) -> Self
    where
        S: Into<StatusCode>,
    {
        let code = code.into();
        Self {
            code: code.as_u16(),
            custom_code,
            message,
        }
    }
}

impl<T> IntoResponse for ErrorResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status_code = StatusCode::from_u16(self.code).unwrap();
        let body = Json(self);

        (status_code, body).into_response()
    }
}
