use crate::body::Body;
use lambda_http::http::{
    self,
    header::{HeaderMap, HeaderValue},
    Response,
};
use serde::ser::{Error as SerError, SerializeMap, Serializer};
use serde_derive::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventResponse {
    pub(crate) status_code: u16,
    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        serialize_with = "serialize_headers"
    )]
    pub(crate) headers: HeaderMap<HeaderValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) body: Option<Body>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) encoding: Option<String>,
}

impl Default for EventResponse {
    fn default() -> Self {
        Self {
            status_code: 200,
            headers: Default::default(),
            body: Default::default(),
            encoding: Default::default(),
        }
    }
}

fn serialize_headers<S>(headers: &HeaderMap<HeaderValue>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(headers.keys_len()))?;
    for key in headers.keys() {
        let map_value = headers[key].to_str().map_err(S::Error::custom)?;
        map.serialize_entry(key.as_str(), map_value)?;
    }
    map.end()
}

impl<T> From<Response<T>> for EventResponse
where
    T: Into<Body>,
{
    fn from(value: Response<T>) -> Self {
        let (parts, bod) = value.into_parts();
        let (encoding, body) = match bod.into() {
            Body::Empty => (None, None),
            b @ Body::Text(_) => (None, Some(b)),
            b @ Body::Binary(_) => (Some("base64".to_string()), Some(b)),
        };
        EventResponse {
            status_code: parts.status.as_u16(),
            body,
            headers: parts.headers,
            encoding,
        }
    }
}

/// A conversion of self into a `Response`
///
/// Implementations for `Response<B> where B: Into<Body>`,
/// `B where B: Into<Body>` and `serde_json::Value` are provided
/// by default
///
/// # example
///
/// ```rust
/// use vercel_runtime::{Body, IntoResponse, Response};
///
/// assert_eq!(
///   "hello".into_response().body(),
///   Response::new(Body::from("hello")).body()
/// );
/// ```
pub(crate) trait IntoResponse {
    /// Return a translation of `self` into a `Response<Body>`
    fn into_response(self) -> Response<Body>;
}

impl<B> IntoResponse for Response<B>
where
    B: Into<Body>,
{
    fn into_response(self) -> Response<Body> {
        let (parts, body) = self.into_parts();
        Response::from_parts(parts, body.into())
    }
}

impl<B> IntoResponse for B
where
    B: Into<Body>,
{
    fn into_response(self) -> Response<Body> {
        Response::new(self.into())
    }
}

impl IntoResponse for serde_json::Value {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(
                serde_json::to_string(&self)
                    .expect("unable to serialize serde_json::Value")
                    .into(),
            )
            .expect("unable to build http::Response")
    }
}
