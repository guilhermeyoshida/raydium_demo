use std::env;

pub const WAIT_TIME_AFTER_TRANSACTION: u64 = 15;

pub fn get_cluster_urls() -> (String, String) {
    let cluster_url = env::var("SOLANA_CLUSTER_URL")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com/".to_string());
    let websocket_url = env::var("SOLANA_WEBSOCKET_URL")
        .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com/".to_string());

    (cluster_url, websocket_url)
}
