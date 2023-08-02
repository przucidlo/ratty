use axum::{
    body::{boxed, Body, BoxBody, Bytes, HttpBody},
    http::{header::CONTENT_TYPE, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::Serialize;

#[derive(Serialize)]
struct SuccessResponse<B> {
    result: B,
}

#[derive(Serialize)]
struct FailureResponse {
    code: u16,
    message: String,
}

pub async fn response_schema_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(request).await;
    let (parts, body) = response.into_parts();

    let mut json_body: Option<String> = None;

    if parts.status.is_success() {
        let data = body.boxed_unsync().data().await;

        if data.is_some() {
            match data.unwrap() {
                Ok(data) => {
                    json_body = Some(self::build_successful_body(data));
                }
                Err(_) => todo!(),
            }
        }
    } else {
        json_body = Some(self::build_failure_body(parts.status));
    }

    let box_body: BoxBody = match json_body {
        Some(json_body) => boxed(json_body),
        None => boxed(Body::empty()),
    };

    response = Response::new(box_body);

    *response.status_mut() = parts.status;

    response
        .headers_mut()
        .insert(CONTENT_TYPE, "application/json".parse().unwrap());

    response
}

fn build_successful_body(body: Bytes) -> String {
    let body = String::from_utf8(body.to_vec());

    match body {
        Ok(body) => {
            let successful_response = SuccessResponse { result: body };

            serde_json::to_string(&successful_response).unwrap()
        }
        Err(_) => serde_json::to_string(&FailureResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: StatusCode::INTERNAL_SERVER_ERROR
                .canonical_reason()
                .unwrap()
                .to_owned(),
        })
        .unwrap(),
    }
}

fn build_failure_body(status: StatusCode) -> String {
    serde_json::to_string(&FailureResponse {
        code: status.as_u16(),
        message: status.canonical_reason().to_owned().unwrap().to_owned(),
    })
    .unwrap()
}
