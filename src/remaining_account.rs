use std::cmp::Ordering;

use anchor_lang::prelude::AccountMeta;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct RemainingAccount {
    pub pubkey: Pubkey,
    pub is_writable: bool,
    pub is_signer: bool
}


impl RemainingAccount {

    fn pubkey(&self) -> &Pubkey {
       &self.pubkey
    }

    fn parts(self) -> (Pubkey, bool , bool) {
        (self.pubkey, self.is_writable , self.is_signer)
    }
  
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        let ptr = <*const RemainingAccount>::from(self);
        unsafe { *ptr.cast::<u8>() }
    }
}

impl Ord for RemainingAccount {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.discriminant().cmp(&other.discriminant());
        if let Ordering::Equal = type_order {
            self.pubkey().cmp(other.pubkey())
        } else {
            type_order
        }
    }
}

impl PartialOrd for RemainingAccount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<RemainingAccount> for AccountMeta {
    fn from(value: RemainingAccount) -> Self {
        let (pubkey, is_writable , is_signer) = value.parts();
        AccountMeta {
            pubkey,
            is_writable,
            is_signer: false,
        }
    }
}