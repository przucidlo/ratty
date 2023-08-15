use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::FromRequest,
    http::Request,
    response::IntoResponse,
    BoxError,
};
use garde::{Errors, Unvalidated, Validate};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::errors::json_response_error::JsonResponseError;

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
    type Rejection = JsonResponseError;

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
                        let json = garde_errors_to_json(err);

                        Err(JsonResponseError::from_validation_errors(json))
                    }
                }
            }
            Err(err) => Err(JsonResponseError::from_str(500, &err.to_string())),
        }
    }
}

fn garde_errors_to_json(errors: Errors) -> Value {
    fn inner_errors_to_json(out: &mut Value, keys: Vec<String>, errors: &Errors) {
        match errors {
            Errors::Simple(errors) => {
                let value = traverse_with_create(out, keys);
                let messages = errors
                    .iter()
                    .map(|e| Value::String(e.message.to_string()))
                    .collect();

                *value = Value::Array(messages);
            }
            Errors::Nested { outer, inner } => {
                inner_errors_to_json(out, keys.clone(), inner);
                inner_errors_to_json(out, keys, outer);
            }
            Errors::List(errors) => {
                for (i, errors) in errors.iter().enumerate() {
                    let mut new_keys = keys.clone();
                    new_keys.push(i.to_string());

                    inner_errors_to_json(out, new_keys, errors)
                }
            }
            Errors::Fields(errors) => {
                for (key, errors) in errors.iter() {
                    let mut extended_keys = keys.clone();
                    extended_keys.push(key.to_string());

                    inner_errors_to_json(out, extended_keys, errors)
                }
            }
        };
    }

    fn traverse_with_create(out: &mut Value, keys: Vec<String>) -> &mut Value {
        fn get_or_create(out: &mut Value, key: String) -> &mut Value {
            match out.as_object_mut() {
                Some(value) => value.entry(key).or_insert_with(|| Value::Null),
                None => panic!(),
            }
        }

        let value = keys.iter().fold(out, |previous, next| {
            get_or_create(previous, next.to_owned())
        });

        value
    }

    let mut json = Value::Object(Map::new());

    inner_errors_to_json(&mut json, vec![], &errors);

    json
}
