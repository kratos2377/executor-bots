
use std::borrow::Cow;
use solana_sdk::instruction::Instruction;
pub use solana_sdk::{address_lookup_table::AddressLookupTableAccount, pubkey::Pubkey};
use anchor_lang::{AccountDeserialize, InstructionData};
use crate::vortex_idl::accounts::SettleAllBets;
use crate::vortex_idl::traits::ToAccountMetas;
use crate::{constants, vortex_idl::{self, types}};

pub struct TransactionBuilder {
    /// either account authority or account delegate
    authority: Pubkey,
    /// ordered list of instructions
    ixs: Vec<Instruction>,
}

impl TransactionBuilder {
    /// Initialize a new `TransactionBuilder` for default signer
    pub fn new(
        account: Pubkey,
    ) -> Self
    {
        Self {
            authority: account,
            ixs: Vec::new(),
        }
    }



    pub fn get_settle_all_games_instruction(&mut self , game_id: [u8;16] , user_id: [u8;16],  user_betting_on_id: [u8;16] , session_id: [u8;21] , winner_id: [u8;16] ) {
        let accounts = SettleAllBets {
            user_bet: todo!(),
            game: todo!(),
            player_bet: todo!(),
            game_vault: todo!(),
            vortex_state: todo!(),
            to: todo!(),
            vortex_signer: todo!(),
            system_program: todo!(),
            token_program: todo!(),
        };


        let meta_data = accounts.to_account_metas();

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts: meta_data,
            data: InstructionData::data(&vortex_idl::instructions::SettleAllBets {
                game_id,
                user_betting_on_id: user_betting_on_id,
                session_id: session_id,
                winner_id: winner_id,
            }),
        };

        self.ixs.push(ix);

    }

}

