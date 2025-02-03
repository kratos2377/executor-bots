use solana_sdk::pubkey::Pubkey;

pub const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");

pub const TOKEN_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");


pub const SOLANA_DEVNET_URL: &str = "https://api.devnet.solana.com";


//Kafka TOpics
pub const GAME_BET_SETTLED: &str = "game_bet_settled";
pub const GAME_BET_SETTLED_ERROR: &str = "game_bet_settled_error";