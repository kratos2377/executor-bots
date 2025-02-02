use std::sync::Arc;
use solana_sdk::{message::VersionedMessage, pubkey::Pubkey, hash::Hash , signature::{keypair_from_seed, Keypair, Signature}, signer::Signer, transaction::VersionedTransaction};

use crate::{constants::{self, PROGRAM_ID}, types::{VortexSdkError, VortexSdkResult}, utils};

#[derive(Clone, Debug)]
pub struct Wallet {
    /// The signing keypair, it could be authority or delegate
    signer: Arc<Keypair>,
    /// The drift 'authority' account
    /// user (sub)accounts are derived from this
    authority: Pubkey,
}

impl Wallet {

    /// Init wallet from a string that could be either a file path or the encoded key, uses default sub-account
    pub fn try_from_str(path_or_key: &str) -> VortexSdkResult<Self> {
        let authority = utils::load_keypair_multi_format(path_or_key)?;
        Ok(Self::new(authority))
    }
    /// Construct a read-only wallet
    pub fn read_only(authority: Pubkey) -> Self {
        Self {
            signer: Arc::new(Keypair::new()),
            authority,
        }
    }
    /// Init wallet from base58 encoded seed, uses default sub-account
    ///
    /// # panics
    /// if the key is invalid
    pub fn from_seed_bs58(seed: &str) -> Self {
        let authority = Keypair::from_base58_string(seed);
        Self::new(authority)
    }
    /// Init wallet from seed bytes, uses default sub-account
    pub fn from_seed(seed: &[u8]) -> VortexSdkResult<Self> {
        let authority = keypair_from_seed(seed).map_err(|_| VortexSdkError::InvalidSeed)?;
        Ok(Self::new(authority))
    }
    /// Init wallet with keypair
    ///
    /// * `authority` - keypair for tx signing
    pub fn new(authority: Keypair) -> Self {
        Self {
            authority: authority.pubkey(),
            signer: Arc::new(authority),
        }
    }

    /// Signs the given tx `message` returning the tx on success
    pub fn sign_tx(
        &self,
        mut message: VersionedMessage,
        recent_block_hash: Hash,
    ) -> VortexSdkResult<VersionedTransaction> {
        message.set_recent_blockhash(recent_block_hash);
        let signer: &dyn Signer = self.signer.as_ref();
        let res = VersionedTransaction::try_new(message, &[signer]);

        if res.is_err() {
            return Err(VortexSdkError::ErrorWhileMakingVersionTransaction)
        }

        return Ok(res.unwrap())
    }

    /// Sign message with the wallet's signer
    pub fn sign_message(&self, message: &[u8]) -> VortexSdkResult<Signature> {
        let signer: &dyn Signer = self.signer.as_ref();
        Ok(signer.sign_message(message))
    }
    /// Return the wallet authority address
    pub fn authority(&self) -> &Pubkey {
        &self.authority
    }
    /// Return the wallet signing address
    pub fn signer(&self) -> Pubkey {
        self.signer.pubkey()
    }


}

impl From<Keypair> for Wallet {
    fn from(value: Keypair) -> Self {
        Self::new(value)
    }
}