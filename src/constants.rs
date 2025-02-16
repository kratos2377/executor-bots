use solana_sdk::pubkey::Pubkey;

pub const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("7erCt6RRYepGp2TxGKRiHKH3W5hza4fcDsSfBNqbwXE5");

pub const DEFAULT_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("11111111111111111111111111111111");

pub const TOKEN_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");


pub const SOLANA_DEVNET_URL: &str = "https://api.devnet.solana.com";

pub const SOL_MINT_ADDRESS: Pubkey = solana_sdk::pubkey!("So11111111111111111111111111111111111111112");


pub const USDC_MINT_ADDRESS: Pubkey = solana_sdk::pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");


//Kafka TOpics
pub const GAME_BET_SETTLED: &str = "game_bet_settled";
pub const GAME_BET_SETTLED_ERROR: &str = "game_bet_settled_error";
pub const EXECUTOR_GAME_OVER_STATUS_SETTLED: &str = "executor_game_over_staus_settled";
pub const STAKE_TIME_OVER_RESULT: &str = "stake_time_over_result";