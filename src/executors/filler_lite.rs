use vortex_sdk::{order_subscriber::order_subscriber::OrderSubscriber, AccountProvider};



pub struct FillerLiteExecutor<T: AccountProvider> {
    pub order_subscriber: OrderSubscriber<T>
}

