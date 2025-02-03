use std::sync::OnceLock;

use base64::Engine;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

use crate::{constants::PROGRAM_ID, types::{VortexSdkError, VortexSdkResult}};


static VORTEX_STATE_ACCOUNT: OnceLock<Pubkey> = OnceLock::new();
static VORTEX_SIGNER_ACCOUNT: OnceLock<Pubkey> = OnceLock::new();


pub fn get_user_game_bet_pubkey( game_id: [u8;16] , user_betting_on_id: [u8;16] ,   user_bet_wallet_key: Pubkey , session_id: &[u8;21]) -> Pubkey {
    return Pubkey::find_program_address(
        &[ "user_game_bet".as_bytes() , &game_id , &user_betting_on_id , user_bet_wallet_key.as_ref() , session_id ], 
        &PROGRAM_ID).0;
}



pub fn get_game_pubkey( game_id: [u8;16]  , session_id: &[u8;21]) -> Pubkey {
    return Pubkey::find_program_address(
        &[ "game".as_bytes() , &game_id  , session_id ], 
        &PROGRAM_ID).0;
}



// Player bet is different bet
//Player who is playing the game can only bet on themselves once per session
//User on other hand can bet on any one player multiple times per session.
// The reason for this is that the person winning should not be able to increase their bet when they are sure of their victory
// Since final bet is calculated by (game_total_pot/total_money_staked_on_winner_by_any_person)
//Winning player might get unfair advantage at the end
pub fn get_player_bet_pubkey( game_id: [u8;16]  , user_betting_on_id: [u8;16] , session_id: &[u8;21]) -> Pubkey {
    return Pubkey::find_program_address(
        &[ "player_bet".as_bytes() , &game_id  , &user_betting_on_id,  session_id ], 
        &PROGRAM_ID).0;
}

pub fn get_game_vault_address( game_id: [u8;16]  , session_id: &[u8;21]) -> Pubkey {
    return Pubkey::find_program_address(
        &[ "game_vault".as_bytes() , &game_id  ,  session_id ], 
        &PROGRAM_ID).0;
}



pub fn get_vortex_state_account() -> &'static Pubkey {
    VORTEX_STATE_ACCOUNT.get_or_init(|| {
        let (state_account, _seed) =
            Pubkey::find_program_address(&[&b"vortex_state"[..]], &PROGRAM_ID);
        state_account
    })
}


pub fn get_vortex_signer_account() -> &'static Pubkey {
    VORTEX_SIGNER_ACCOUNT.get_or_init(|| {
        let (signer_account, _seed) =
            Pubkey::find_program_address(&[&b"vortex_signer"[..]], &PROGRAM_ID);
        signer_account
    })
}



pub fn derive_vortex_signer() -> Pubkey {
    let (account, _seed) = Pubkey::find_program_address(&[&b"vortex_signer"[..]], &PROGRAM_ID);
    account
}

pub fn load_keypair_multi_format(path_or_key: &str) -> VortexSdkResult<Keypair> {
    if let Ok(data) = std::fs::read_to_string(path_or_key) {
        read_keypair_str_multi_format(data.as_str())
    } else {
        read_keypair_str_multi_format(path_or_key)
    }
}

pub fn read_keypair_str_multi_format(key: &str) -> VortexSdkResult<Keypair> {
    // strip out any white spaces and new line/carriage return characters
    let key = key.replace([' ', '\n', '\r', '[', ']'], "");

    // first try to decode as a byte array
    if key.contains(',') {
        // decode the numbers array into json string
        let bytes: Result<Vec<u8>, _> = key.split(',').map(|x| x.parse::<u8>()).collect();
        if let Ok(bytes) = bytes {
            return Keypair::from_bytes(&bytes).map_err(|_| VortexSdkError::InvalidSeed);
        } else {
            return Err(VortexSdkError::InvalidSeed);
        }
    }

    // try to decode as base58 string
    if let Ok(bytes) = bs58::decode(key.as_bytes()).into_vec() {
        return Keypair::from_bytes(&bytes).map_err(|_| VortexSdkError::InvalidSeed);
    }

    // try to decode as base64 string
    if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(key.as_bytes()) {
        return Keypair::from_bytes(&bytes).map_err(|_| VortexSdkError::InvalidSeed);
    }

    Err(VortexSdkError::InvalidSeed)
}




pub fn get_http_url(url: &str) -> VortexSdkResult<String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Ok(url.to_string())
    } else if url.starts_with("ws://") || url.starts_with("wss://") {
        Ok(url.replacen("ws", "http", 1))
    } else {
        Err(VortexSdkError::InvalidUrl)
    }
}