//! Error and Result module
use derive_more::{Display, From};
use loony::http::error::{ParseError, PayloadError};
use loony::http::StatusCode;
use loony::web::error::{DefaultError, WebResponseError};

/// A set of errors that can occur during parsing multipart streams
#[derive(Debug, Display, From)]
pub enum MultipartError {
    /// Content-Type header is not found
    #[display(fmt = "No Content-type header found")]
    NoContentType,
    /// Can not parse Content-Type header
    #[display(fmt = "Can not parse Content-Type header")]
    ParseContentType,
    /// Multipart boundary is not found
    #[display(fmt = "Multipart boundary is not found")]
    Boundary,
    /// Nested multipart is not supported
    #[display(fmt = "Nested multipart is not supported")]
    Nested,
    /// Multipart stream is incomplete
    #[display(fmt = "Multipart stream is incomplete")]
    Incomplete,
    /// Error during field parsing
    #[display(fmt = "{}", _0)]
    Parse(ParseError),
    /// Payload error
    #[display(fmt = "{}", _0)]
    Payload(PayloadError),
    /// Not consumed
    #[display(fmt = "Multipart stream is not consumed")]
    NotConsumed,
}

impl std::error::Error for MultipartError {}

/// Return `BadRequest` for `MultipartError`
impl WebResponseError<DefaultError> for MultipartError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use loony::web::test::TestRequest;
    use loony::web::HttpResponse;

    #[test]
    fn test_multipart_error() {
        let req = TestRequest::default().to_http_request();
        let resp: HttpResponse = MultipartError::Boundary.error_response(&req);
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
