use color_eyre::Result;

use std::sync::Arc;
use argonautica::Hasher;
use futures::compat::Future01CompatExt;
use eyre::eyre;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>,
}

impl CryptoService {

    #[instrument(skip(self, password, ))] // <- Add this line to avoid tracing the password and self
    pub async fn hash_password(&self, password: String) -> Result<String> {

        let mut hasher = Hasher::default();
        hasher
            .with_password(password)
            .with_secret_key(&*self.key)
            .hash_non_blocking()
            .compat()
            .await
            .map_err(|e| eyre!("Error hashing password: {}", e))
    
    }
}