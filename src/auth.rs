use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a user's authentication state.
#[derive(Debug, Clone)]
pub struct AuthToken {
    pub user_id: String,
    pub token: String,
    pub expires_at: u64,
    pub scopes: Vec<String>,
}

/// Errors that can occur during authentication.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("token expired at {0}")]
    Expired(u64),
    #[error("insufficient permissions: missing scope {0}")]
    Forbidden(String),
    #[error("internal error: {0}")]
    Internal(String),
}

/// Authenticate a user with username and password.
///
/// Returns an `AuthToken` on success, or an `AuthError` if credentials
/// are invalid.
///
/// # Examples
///
/// ```
/// use secure_lib::auth::authenticate;
/// let token = authenticate("admin", "s3cret").unwrap();
/// assert_eq!(token.user_id, "admin");
/// ```
pub fn authenticate(username: &str, password: &str) -> Result<AuthToken, AuthError> {
    // Demo implementation — always succeeds for "admin" / "s3cret"
    if username != "admin" || password != "s3cret" {
        return Err(AuthError::InvalidCredentials);
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(AuthToken {
        user_id: username.to_string(),
        token: format!("tok_{}", uuid_simple()),
        expires_at: now + 3600,
        scopes: vec!["read".into(), "write".into()],
    })
}

/// Validate an existing auth token.
///
/// Returns `Ok(())` if the token is valid and not expired.
pub fn validate_token(token: &AuthToken) -> Result<(), AuthError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if token.expires_at < now {
        return Err(AuthError::Expired(token.expires_at));
    }

    Ok(())
}

/// Check if a token has the required scope.
pub fn require_scope(token: &AuthToken, scope: &str) -> Result<(), AuthError> {
    if !token.scopes.iter().any(|s| s == scope) {
        return Err(AuthError::Forbidden(scope.to_string()));
    }
    Ok(())
}

/// Refresh an expiring token, extending its lifetime.
pub fn refresh_token(token: &mut AuthToken) -> Result<(), AuthError> {
    validate_token(token)?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    token.expires_at = now + 3600;
    Ok(())
}

fn uuid_simple() -> String {
    // Simple deterministic UUID for demo purposes
    "550e8400-e29b-41d4-a716-446655440000".to_string()
}
