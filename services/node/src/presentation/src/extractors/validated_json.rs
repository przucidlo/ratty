use axum::{
    async_trait,
    body::{boxed, Bytes, HttpBody},
    extract::FromRequest,
    http::Request,
    response::IntoResponse,
    response::Response,
    BoxError,
};
use garde::{Errors, Unvalidated, Validate};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{value, Map};

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
    T: DeserializeOwned + Validate<Context = ()>,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        match serde_json::from_slice::<T>(&body) {
            Ok(json_body) => {
                let value = Unvalidated::new(json_body).validate(&());

                match value {
                    Ok(value) => Ok(Self(value.into_inner())),
                    Err(err) => {
                        println!("{err}");

                        panic!()
                        // Err(Response::new(err.flatten().to_vec))
                    }
                }
            }
            Err(_) => panic!(),
        }
    }
}

fn garde_errors_to_json(errors: Errors) -> String {
    let json = Map::new();
    // fn inner_errors_to_json(error: Error)

    match errors {
        Errors::Simple(errors) => todo!(),
        Errors::Nested { outer, inner } => todo!(),
        Errors::List(_) => todo!(),
        Errors::Fields(_) => todo!(),
    };

    serde_json::Value::Object(json).to_string();
}
