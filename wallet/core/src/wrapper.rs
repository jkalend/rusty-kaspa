use crate::*;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait WalletWrapper {
    async fn open_wallet(encrypted_wallet: &str, password: &str) -> Result<Arc<Self>>
    where
        Self: Sized;

    async fn sync(&self) -> Result<()>;

    async fn receive_address(&self) -> Result<Address>;
}
