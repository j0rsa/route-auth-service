extern crate jsonwebtoken as jwt;

use super::models::Claims;
use jwt::dangerous_insecure_decode;

pub fn get_claims(token: &str) -> jsonwebtoken::errors::Result<Claims> {
    dangerous_insecure_decode::<Claims>(&token).map(|d| d.claims)
}

const BEARER_LENGTH: usize = "Bearer ".len();

pub fn get_bearer_token(authorization_header: String) -> Option<String> {
    if authorization_header.starts_with("Bearer") {
        Option::Some(authorization_header[BEARER_LENGTH..].to_string())
    } else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use crate::providers::token::*;

    use crate::providers::token;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_bearer_token() {
        let auth = "Bearer <token>".to_string();
        let resp = get_bearer_token(auth);
        assert_eq!(resp, Option::Some("<token>".to_string()));
    }

    #[test]
    fn test_get_claims() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiIiLCJzdWIiOiIxMjM0NTY3ODkwIiwiYXVkIjoiIiwiZXhwIjoxNTgyODQyODUzMzEyLCJuYmYiOjE1ODAyNTA4NTMzMTIsImlhdCI6MTU4MDI1MDg1MzMxMiwianRpIjoiZDEwM2FiM2QtZmM1My00OTM2LThkZjQtM2FkNTdkNmI1YjNmIiwibmFtZSI6IjEyM3Rlc3QifQ.xa57RMHUD3sTnu561IsSedgd-j627GrrKMInQt_zATk";
        let secret = "test".to_string();
        let claims = get_claims_with_secret(&token.to_string()).unwrap();
        assert_eq!(claims.sub, "1234567890");
    }
}
