use serde::{Serialize , Deserialize};

#[derive(Clone , Serialize , Deserialize)]
pub struct GameBetSettleKafkaPayload {
    pub game_id: String,
    pub session_id: String,
    pub winner_id: String,
    pub user_id: String,
    pub user_betting_on: String,
    pub record_id: String,
    pub user_wallet_key: String,
    pub is_valid: bool,
}


#[derive(Clone , Serialize , Deserialize)]
pub struct ExecutorGameOverEvent {
    pub game_id: String,
    pub session_id: String,
    pub winner_id: String,
    pub is_game_valid: bool
}

#[derive(Clone , Serialize , Deserialize)]
pub struct ExecutorGameStakeTimeOverEvent {
    pub game_id: String,
    pub session_id: String,
}


#[derive(Clone , Serialize , Deserialize)]
pub struct EventQueueRecords {
    pub game_settle_record: Option<GameBetSettleKafkaPayload>,
    pub game_stake_time_over_record: Option<ExecutorGameStakeTimeOverEvent>
}


#[derive(Clone , Serialize , Deserialize)]
pub struct EventRecord {
    pub payload: String,
    pub event_type: String,
}


#[derive(Clone , Serialize , Deserialize)]
pub struct GameUserBetSettleEvent {
    pub game_id: String,
    pub session_id: String,
    pub user_id: String,
    pub winner_id: String,
    pub is_game_valid: bool,
    pub is_error: bool
}



#[derive(Clone , Serialize , Deserialize)]
pub struct GameStatusChangeEvent {
    pub game_id: String,
    pub session_id: String,
    pub winner_id: String,
    pub is_game_valid: bool,
    pub is_error: bool
}


#[derive(Clone , Serialize , Deserialize)]
pub struct GameStakeStatusChangeEvent {
    pub game_id: String,
    pub session_id: String,
    pub is_error: bool
}

//Event Types
pub const EXECUTOR_INDEX_ADD: &str = "executor_index_add";
pub const CONSUMER_GROUP_BET_EVENT_ADD: &str = "consumer_group_bet_event_add";
pub const GAME_OVER_EVENT: &str = "game_over_event";
pub const GAME_STAKE_TIME_OVER_EVENT: &str = "game_stake_time_over_event";