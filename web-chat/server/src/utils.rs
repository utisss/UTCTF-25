use axum::response::IntoResponse;

// TODO: better error messages
// https://github.com/tokio-rs/axum/issues/1116

pub type Result<T, E = WebError> = ::core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum WebError {
    #[error("An io error occurred")]
    IoError(#[from] std::io::Error),
    #[error("An error occurred while formatting a template")]
    TemplateError(#[from] sailfish::RenderError),
    #[error("An error occurred while formatting a string")]
    FmtError(#[from] std::fmt::Error),

    #[error("An unknown error occurred")]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        error!(
            "error in handler:\n{}",
            runtime::utils::format_error_disp(&self)
        );
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "an unknown error occurred",
        )
            .into_response()
    }
}
