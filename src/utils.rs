use anchor_spl::associated_token::get_associated_token_address;
use axum::Json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{transaction::Transaction, transaction_context::TransactionAccounts};
use vortex_contracts::state::user::MarketType;
use vortex_sdk::{Pubkey, TransactionBuilder, Wallet};

use crate::errors::{APIResult, Error};
use serde_json::Value;


pub const TOKEN_FAUCET_PROGRAM_ID: &str = "V4v1mQiAdLz4qwckEb45WqHYceYizoib39cDBHSWfaB";


pub const JUPITER_SLIPPAGE_BPS: u64 = 100;
pub const PRIORITY_FEE_SERVER_RATE_LIMIT_PER_MIN: u64 = 100;

pub async fn get_or_create_associated_token_account(
    connection: RpcClient,
    mint: Pubkey,
    wallet: Wallet
) -> Pubkey {

    let associated_token_account = get_associated_token_address(&mint, wallet.authority());
    

    let account_info_details = connection.get_account_data(&associated_token_account);


    let account_info_details_model = account_info_details.unwrap();

    // if(account_info_details_model.is_empty() || account_info_details_model == None) {
    //     let tx  = Transaction::default();
    //     tx.ad
    // }


    
    associated_token_account
}

pub fn load_comma_delimit_to_array(
    key: String
) -> Vec<u64> {
    let result: Vec<u64> = key
        .split(',')
        .filter_map(|element| {
            let trimmed = element.trim();
            if trimmed.is_empty() {
                None 
            } else {
             
                trimmed.parse::<u64>().ok()
            }
        })
        .collect();


    if result.len() == 0 {
        return Vec::new()
    }

    return result
}

pub fn load_comma_delimit_to_string_array(
    key: String
) -> Vec<String> {
    let rsp : Vec<String> = key
    .split(',')
    .filter_map(|element| {
        let trimmed = element.trim();
        if trimmed.is_empty() {
            None 
        } else {
           Some(trimmed.to_string())
        }
    }).collect();

if rsp.len() == 0 {
    return Vec::new()
}

return rsp
}

pub fn convert_market_type(key: String) -> MarketType {
    match key.as_str() {
        "SPOT" => MarketType::Spot,
        _ => MarketType::Spot,
        
    }
}