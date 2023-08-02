use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::FromRequest,
    http::Request,
    response::IntoResponse,
    response::Response,
    BoxError,
};
use garde::{Unvalidated, Validate};
use serde::de::DeserializeOwned;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: Validate<Context = S> + DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        match serde_json::from_slice::<T>(&body) {
            Ok(json_body) => {
                let value = Unvalidated::new(json_body).validate(state);

                match value {
                    Ok(value) => Ok(Self(value.into_inner())),
                    Err(err) => panic!(),
                }
            }
            Err(_) => panic!(),
        }
    }
}
