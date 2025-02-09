
use std::borrow::Cow;
use std::collections::BTreeSet;
use anchor_lang::prelude::AccountMeta;
use anchor_spl::associated_token::get_associated_token_address;
use serde::{Deserialize, Serialize};
use solana_sdk::instruction::Instruction;
use solana_sdk::message::{v0, VersionedMessage};
pub use solana_sdk::{address_lookup_table::AddressLookupTableAccount, pubkey::Pubkey};
use anchor_lang::{ InstructionData};
use crate::constants::{DEFAULT_PROGRAM_ID, PROGRAM_ID, SOL_MINT_ADDRESS, TOKEN_PROGRAM_ID};
use crate::remaining_account::RemainingAccount;
use crate::types::VortexSdkResult;
use crate::utils::{derive_vortex_signer, get_game_pubkey, get_game_vault_address, get_player_bet_pubkey, get_user_game_bet_pubkey, get_vortex_signer_account, get_vortex_state_account};
use crate::vortex_idl::accounts::SettleAllBets;
use crate::vortex_idl::traits::ToAccountMetas;
use crate::{constants, vortex_idl::{self, types}};



pub async fn get_settle_all_games_instruction(authority: Pubkey , game_id: [u8;16] , user_id: [u8;16],  user_betting_on_id: [u8;16] , session_id: &[u8;21] ,
     winner_id: [u8;16] , user_bet_wallet_key: Pubkey ) -> VortexSdkResult<VersionedMessage>{
  
        //let mut accounts_tree_set  = BTreeSet::<RemainingAccount>::new();
        let get_user_token_account_address = get_associated_token_address(&user_bet_wallet_key , &SOL_MINT_ADDRESS);
    let accounts = SettleAllBets {
        user_bet: get_user_game_bet_pubkey(game_id , user_betting_on_id , user_bet_wallet_key , session_id),
        game: get_game_pubkey(game_id, session_id),
        player_bet: get_player_bet_pubkey(game_id, user_betting_on_id, session_id),
        game_vault: get_game_vault_address(game_id, session_id),
        vortex_state: *get_vortex_state_account(),
        to: get_user_token_account_address,
        vortex_signer: authority,
        system_program: DEFAULT_PROGRAM_ID,
        token_program: TOKEN_PROGRAM_ID,
    };


    let mut account_metas = accounts.to_account_metas();

    // let remaining_accounts =  RemainingAccount {
    //     pubkey: SOL_MINT_ADDRESS,
    //     is_writable: false,
    //     is_signer: false,
    // };
    
 //   accounts_tree_set.extend( [remaining_accounts.into()].iter());
 //   account_metas.extend(accounts_tree_set.into_iter().map(Into::into));
  
    let mut meta_data = account_metas;

    meta_data.push(AccountMeta{ pubkey: SOL_MINT_ADDRESS, is_signer: false, is_writable: false });

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
