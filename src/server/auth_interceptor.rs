use std::sync::Arc;
use tonic::{Request, Status};

use crate::auth::Role;
use super::session::JwtSessionManager;

/// Injected into request extensions after successful token validation.
#[derive(Clone, Debug)]
pub struct AuthContext {
    pub username: String,
    pub role: Role,
}

/// Interceptor applied to the VantaDb service.
/// Validates the JWT from the "authorization" metadata key.
#[derive(Clone)]
pub struct AuthInterceptor {
    pub jwt_manager: Arc<JwtSessionManager>,
}

impl tonic::service::Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        let token = parse_bearer_token(&req)?;

        let (username, role) = self
            .jwt_manager
            .validate(&token)
            .ok_or_else(|| Status::unauthenticated("Invalid or expired token"))?;

        req.extensions_mut().insert(AuthContext { username, role });
        Ok(req)
    }
}

/// Helper to extract AuthContext from request extensions.
pub fn extract_auth<T>(request: &Request<T>) -> Result<AuthContext, Status> {
    request
        .extensions()
        .get::<AuthContext>()
        .cloned()
        .ok_or_else(|| Status::internal("Missing auth context"))
}

/// Helper to extract auth from metadata directly (for services without interceptor).
pub fn extract_auth_from_metadata<T>(
    request: &Request<T>,
    jwt_manager: &JwtSessionManager,
) -> Result<AuthContext, Status> {
    let token = parse_bearer_token(request)?;

    let (username, role) = jwt_manager
        .validate(&token)
        .ok_or_else(|| Status::unauthenticated("Invalid or expired token"))?;

    Ok(AuthContext { username, role })
}

fn parse_bearer_token<T>(request: &Request<T>) -> Result<String, Status> {
    let value = request
        .metadata()
        .get("authorization")
        .ok_or_else(|| Status::unauthenticated("Missing authorization token"))?;

    let header = value
        .to_str()
        .map_err(|_| Status::unauthenticated("Invalid authorization header encoding"))?;

    let token = header
        .strip_prefix("Bearer ")
        .ok_or_else(|| Status::unauthenticated("Authorization header must use Bearer auth"))?
        .trim();

    if token.is_empty() {
        return Err(Status::unauthenticated("Missing bearer token"));
    }

    Ok(token.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;

    #[test]
    fn test_parse_bearer_token_requires_scheme() {
        let mut request = Request::new(());
        request.metadata_mut().insert("authorization", "token-only".parse().unwrap());
        let err = parse_bearer_token(&request).unwrap_err();
        assert_eq!(err.code(), tonic::Code::Unauthenticated);
    }

    #[test]
    fn test_parse_bearer_token_accepts_valid_header() {
        let mut request = Request::new(());
        request.metadata_mut().insert("authorization", "Bearer abc123".parse().unwrap());
        assert_eq!(parse_bearer_token(&request).unwrap(), "abc123");
    }
}
