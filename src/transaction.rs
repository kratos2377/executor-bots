
use std::borrow::Cow;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::{v0, VersionedMessage};
pub use solana_sdk::{address_lookup_table::AddressLookupTableAccount, pubkey::Pubkey};
use anchor_lang::{AccountDeserialize, InstructionData};
use crate::constants::TOKEN_PROGRAM_ID;
use crate::types::VortexSdkResult;
use crate::utils::{derive_vortex_signer, get_game_pubkey, get_game_vault_address, get_player_bet_pubkey, get_user_game_bet_pubkey, get_vortex_state_account};
use crate::vortex_idl::accounts::SettleAllBets;
use crate::vortex_idl::traits::ToAccountMetas;
use crate::{constants, vortex_idl::{self, types}};


pub async fn get_settle_all_games_instruction(authority: Pubkey , game_id: [u8;16] , user_id: [u8;16],  user_betting_on_id: [u8;16] , session_id: &[u8;21] ,
     winner_id: [u8;16] , user_bet_wallet_key: Pubkey ) -> VortexSdkResult<VersionedMessage>{
    let accounts = SettleAllBets {
        user_bet: get_user_game_bet_pubkey(game_id , user_betting_on_id , user_bet_wallet_key , session_id),
        game: get_game_pubkey(game_id, session_id),
        player_bet: get_player_bet_pubkey(game_id, user_betting_on_id, session_id),
        game_vault: get_game_vault_address(game_id, session_id),
        vortex_state: *get_vortex_state_account(),
        to: user_bet_wallet_key,
        vortex_signer: derive_vortex_signer(),
        system_program: anchor_lang::system_program::ID,
        token_program: TOKEN_PROGRAM_ID,
    };


    let meta_data = accounts.to_account_metas();

    let ix = Instruction {
        program_id: constants::PROGRAM_ID,
        accounts: meta_data,
        data: InstructionData::data(&vortex_idl::instructions::SettleAllBets {
            game_id,
            user_betting_on_id: user_betting_on_id,
            session_id: *session_id,
            winner_id: winner_id,
        }),
    };

    let message =
    v0::Message::try_compile(&authority, &vec![ix], &[], Default::default())
        .expect("failed to compile message");

Ok(VersionedMessage::V0(message))

}
