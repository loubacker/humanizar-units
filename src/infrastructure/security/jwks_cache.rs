use std::sync::Arc;
use std::time::{Duration, Instant};

use jsonwebtoken::DecodingKey;
use jsonwebtoken::jwk::JwkSet;
use tokio::sync::{Mutex, RwLock};
use url::Url;

use crate::domain::exception::TechnicalError;

pub(crate) type JwksCacheError = TechnicalError;

pub(crate) struct JwksCache {
    client: reqwest::Client,
    jwks_url: Url,
    cache_ttl: Duration,
    minimum_refresh_interval: Duration,
    state: RwLock<CachedJwks>,
    refresh_lock: Mutex<()>,
}

impl JwksCache {
    pub(crate) async fn initialize(
        jwks_url: Url,
        cache_ttl: Duration,
        minimum_refresh_interval: Duration,
    ) -> Result<Arc<Self>, JwksCacheError> {
        let client = reqwest::Client::builder()
            .build()
            .map_err(|error| JwksCacheError::with_source("Falha ao criar cliente JWKS", error))?;
        let keys = fetch_jwks(&client, &jwks_url).await?;

        Ok(Arc::new(Self {
            client,
            jwks_url,
            cache_ttl,
            minimum_refresh_interval,
            state: RwLock::new(CachedJwks {
                keys,
                refreshed_at: Instant::now(),
                last_refresh_attempt: None,
            }),
            refresh_lock: Mutex::new(()),
        }))
    }

    pub(crate) async fn key_for(&self, key_id: &str) -> Result<DecodingKey, JwksCacheError> {
        let cached_key = self.cached_key(key_id).await?;

        if let Some(key) = cached_key.as_ref()
            && self.cache_is_fresh().await
        {
            return Ok(key.clone());
        }

        let _refresh_guard = self.refresh_lock.lock().await;
        let cached_key = self.cached_key(key_id).await?;

        if let Some(key) = cached_key.as_ref()
            && self.cache_is_fresh().await
        {
            return Ok(key.clone());
        }

        if !self.refresh_is_allowed().await {
            return cached_key.ok_or_else(|| unknown_key(key_id));
        }

        self.record_refresh_attempt().await;

        match fetch_jwks(&self.client, &self.jwks_url).await {
            Ok(keys) => {
                self.replace_keys(keys).await;
                self.cached_key(key_id)
                    .await?
                    .ok_or_else(|| unknown_key(key_id))
            }
            Err(error) => cached_key.ok_or(error),
        }
    }

    async fn cached_key(&self, key_id: &str) -> Result<Option<DecodingKey>, JwksCacheError> {
        let state = self.state.read().await;

        state
            .keys
            .find(key_id)
            .map(DecodingKey::from_jwk)
            .transpose()
            .map_err(|error| JwksCacheError::with_source("Chave JWKS inválida", error))
    }

    async fn cache_is_fresh(&self) -> bool {
        self.state.read().await.refreshed_at.elapsed() < self.cache_ttl
    }

    async fn refresh_is_allowed(&self) -> bool {
        self.state
            .read()
            .await
            .last_refresh_attempt
            .is_none_or(|last_attempt| last_attempt.elapsed() >= self.minimum_refresh_interval)
    }

    async fn record_refresh_attempt(&self) {
        self.state.write().await.last_refresh_attempt = Some(Instant::now());
    }

    async fn replace_keys(&self, keys: JwkSet) {
        let mut state = self.state.write().await;
        state.keys = keys;
        state.refreshed_at = Instant::now();
    }
}

struct CachedJwks {
    keys: JwkSet,
    refreshed_at: Instant,
    last_refresh_attempt: Option<Instant>,
}

fn unknown_key(key_id: &str) -> JwksCacheError {
    JwksCacheError::new(format!("JWKS não contém a chave {key_id}"))
}

async fn fetch_jwks(client: &reqwest::Client, jwks_url: &Url) -> Result<JwkSet, JwksCacheError> {
    let response = client
        .get(jwks_url.clone())
        .send()
        .await
        .map_err(|error| JwksCacheError::with_source("Falha ao consultar JWKS", error))?
        .error_for_status()
        .map_err(|error| JwksCacheError::with_source("JWKS respondeu com erro", error))?;
    let keys = response
        .json::<JwkSet>()
        .await
        .map_err(|error| JwksCacheError::with_source("Resposta JWKS inválida", error))?;

    if keys.keys.is_empty() {
        return Err(JwksCacheError::new("Resposta JWKS não contém chaves"));
    }

    Ok(keys)
}
