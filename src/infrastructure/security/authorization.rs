use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;

use crate::domain::exception::SecurityException;
use crate::infrastructure::controller::handler::SecurityHttpError;

use super::authenticated_user::AuthenticatedUser;
use super::jwt_validator::JwtValidator;

pub(crate) async fn authenticate(
    State(validator): State<Arc<JwtValidator>>,
    mut request: Request,
    next: Next,
) -> Result<Response, SecurityHttpError> {
    let path = request.uri().path().to_owned();
    let token =
        bearer_token(&request).map_err(|exception| SecurityHttpError::new(exception, &path))?;
    let authenticated_user = validator.validate(token).await.map_err(|error| {
        SecurityHttpError::new(SecurityException::authentication_with_source(error), &path)
    })?;

    request.extensions_mut().insert(authenticated_user);

    Ok(next.run(request).await)
}

pub(crate) async fn require_administrator(
    request: Request,
    next: Next,
) -> Result<Response, SecurityHttpError> {
    let path = request.uri().path().to_owned();
    let Some(authenticated_user) = request.extensions().get::<AuthenticatedUser>() else {
        return Err(SecurityHttpError::new(
            SecurityException::authentication(),
            path,
        ));
    };

    if !authenticated_user.has_role("ADMINISTRADOR") {
        return Err(SecurityHttpError::new(
            SecurityException::authorization(),
            path,
        ));
    }

    Ok(next.run(request).await)
}

fn bearer_token(request: &Request) -> Result<&str, SecurityException> {
    let value = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(SecurityException::authentication)?
        .to_str()
        .map_err(SecurityException::authentication_with_source)?;
    let mut parts = value.split_whitespace();
    let scheme = parts.next().unwrap_or_default();
    let token = parts.next().unwrap_or_default();

    if !scheme.eq_ignore_ascii_case("Bearer") || token.is_empty() || parts.next().is_some() {
        return Err(SecurityException::authentication());
    }

    Ok(token)
}
