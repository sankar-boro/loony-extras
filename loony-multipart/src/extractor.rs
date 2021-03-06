//! Multipart payload support
use std::convert::Infallible;

use futures::future::{ok, Ready};
use loony::http::Payload;
use loony::web::{FromRequest, HttpRequest};

use crate::server::Multipart;

/// Get request's payload as multipart stream
///
/// Content-type: multipart/form-data;
///
/// ## Server example
///
/// ```rust
/// use futures::{Stream, StreamExt};
/// use loony::web::{self, HttpResponse, Error};
/// use loony_multipart as mp;
///
/// async fn index(mut payload: mp::Multipart) -> Result<HttpResponse, Error> {
///     // iterate over multipart stream
///     while let Some(item) = payload.next().await {
///            let mut field = item?;
///
///            // Field in turn is stream of *Bytes* object
///            while let Some(chunk) = field.next().await {
///                println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
///            }
///     }
///     Ok(HttpResponse::Ok().into())
/// }
/// # fn main() {}
/// ```
impl<Err> FromRequest<Err> for Multipart {
    type Error = Infallible;
    type Future = Ready<Result<Multipart, Infallible>>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        ok(Multipart::new(req.headers(), payload.take()))
    }
}
