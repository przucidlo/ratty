use axum::{http::Request, middleware::Next, response::Response};

pub struct UserExtension {
    pub user_id: u64,
}

pub async fn authentication_middleware<B>(mut request: Request<B>, next: Next<B>) -> Response {
    let user_extension: Option<UserExtension> = handle_internal_authentication(&request);

    if let Some(user_extension) = user_extension {
        request.extensions_mut().insert(user_extension);
    }

    next.run(request).await
}

fn handle_internal_authentication<B>(request: &Request<B>) -> Option<UserExtension> {
    let schema = request.uri().scheme();

    if schema == None {
        let header = request.headers().get("x-user-id")?;

        let user_id = header.to_str().ok()?.parse::<u64>().ok()?;

        Some(UserExtension { user_id })
    } else {
        None
    }
}
