use std::sync::Arc;

use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{hash::Hash, message::VersionedMessage, signature::Signature};


use crate::{types::VortexSdkResult, utils::get_http_url, wallet::Wallet};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Context {
    name: &'static str
}


#[derive(Clone)]
#[must_use]
pub struct VortexExecutorClient {
    backend: &'static VortexExecutorClientBackend,
    wallet: Wallet,
}


impl VortexExecutorClient {

    pub async fn new(rpc_client: RpcClient, wallet: Wallet) -> VortexSdkResult<Self> {
        // check URL format here to fail early, otherwise happens at request time.
        let _ = get_http_url(&rpc_client.url())?;
        Ok(Self {
            backend: Box::leak(Box::new(
                VortexExecutorClientBackend::new( Arc::new(rpc_client)).await?,
            )),
            wallet,
        })
    }


    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }


    pub async fn sign_and_send(
        &self,
        tx: VersionedMessage,
    ) -> VortexSdkResult<Signature> {
        // will need to replace recent hash block logic
        self.backend
            .sign_and_send(self.wallet() , tx , Hash::new("random_slice".as_bytes()))
            .await
            .map_err(Into::into)
    }

    /// Sign and send a tx to the network with custom send config
    /// allows setting commitment level, retries, etc.
    ///
    /// Returns the signature on success
    pub async fn sign_and_send_with_config(
        &self,
        tx: VersionedMessage,
        recent_block_hash: Option<Hash>,
        config: RpcSendTransactionConfig,
    ) -> VortexSdkResult<Signature> {
                // will need to replace recent hash block logic
        let recent_block_hash = match recent_block_hash {
            Some(h) => h,
            None => Hash::new("random_slice".as_bytes()),
        };
        self.backend
            .sign_and_send_with_config(self.wallet(), tx, recent_block_hash, config)
            .await
            .map_err(Into::into)
    }


}




pub struct VortexExecutorClientBackend {
    rpc_client: Arc<RpcClient>,
}


impl VortexExecutorClientBackend  {
    
    async fn new(rpc_client: Arc<RpcClient> ) -> VortexSdkResult<Self> {
        Ok(
            Self { rpc_client: Arc::clone(&rpc_client) }
        )
    }

    /// Return a handle to the inner RPC client
    fn client(&self) -> Arc<RpcClient> {
        Arc::clone(&self.rpc_client)
    }


    pub async fn sign_and_send(
        &self,
        wallet: &Wallet,
        tx: VersionedMessage,
        recent_block_hash: Hash,
    ) -> VortexSdkResult<Signature> {
        let tx = wallet.sign_tx(tx, recent_block_hash)?;
        self.rpc_client
            .send_transaction(&tx)
            .await
            .map_err(Into::into)
    }

    /// Sign and send a tx to the network with custom send config
    /// allows setting commitment level, retries, etc.
    ///
    /// Returns the signature on success
    pub async fn sign_and_send_with_config(
        &self,
        wallet: &Wallet,
        tx: VersionedMessage,
        recent_block_hash: Hash,
        config: RpcSendTransactionConfig,
    ) -> VortexSdkResult<Signature> {
        let tx = wallet.sign_tx(tx, recent_block_hash)?;
        self.rpc_client
            .send_transaction_with_config(&tx, config)
            .await
            .map_err(Into::into)
    }
    
}