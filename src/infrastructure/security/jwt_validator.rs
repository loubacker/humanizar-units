use std::sync::Arc;

use jsonwebtoken::{Algorithm, Validation, decode, decode_header};

use crate::domain::exception::TechnicalError;

use super::authenticated_user::AuthenticatedUser;
use super::jwks_cache::{JwksCache, JwksCacheError};
use super::jwt_claims::JwtClaims;

pub(crate) type JwtValidationError = TechnicalError;

pub(crate) struct JwtValidator {
    jwks_cache: Arc<JwksCache>,
    validation: Validation,
}

impl JwtValidator {
    pub(crate) fn new(jwks_cache: Arc<JwksCache>, issuer: &str, audience: &str) -> Self {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_nbf = true;
        validation.set_issuer(&[issuer]);
        validation.set_audience(&[audience]);
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);

        Self {
            jwks_cache,
            validation,
        }
    }

    pub(crate) async fn validate(
        &self,
        token: &str,
    ) -> Result<AuthenticatedUser, JwtValidationError> {
        let header = decode_header(token).map_err(invalid_token)?;

        if header.alg != Algorithm::RS256 {
            return Err(JwtValidationError::new("Algoritmo JWT não suportado"));
        }

        let key_id = header
            .kid
            .filter(|key_id| !key_id.trim().is_empty())
            .ok_or_else(|| JwtValidationError::new("JWT não possui kid"))?;
        let decoding_key = self.jwks_cache.key_for(&key_id).await.map_err(jwks_error)?;
        let token =
            decode::<JwtClaims>(token, &decoding_key, &self.validation).map_err(invalid_token)?;

        if token.claims.subject_is_empty() {
            return Err(JwtValidationError::new("JWT possui subject vazio"));
        }

        Ok(token.claims.into_authenticated_user())
    }
}

fn invalid_token(source: jsonwebtoken::errors::Error) -> JwtValidationError {
    JwtValidationError::with_source("JWT inválido", source)
}

fn jwks_error(source: JwksCacheError) -> JwtValidationError {
    JwtValidationError::with_source("Não foi possível validar a chave do JWT", source)
}
