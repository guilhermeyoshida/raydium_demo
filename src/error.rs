use solana_sdk::pubkey::ParsePubkeyError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Could not parse wallet in path {0})")]
    InvalidPublicKey(#[from] anyhow::Error),
    #[error("Could not parse wallet public key")]
    InvalidWallet(#[from] ParsePubkeyError),
    #[error("The amount cannot be zero")]
    AmountCannotBeZero,
    #[error("Error interacting with Raydium Pool: {0}")]
    LiquidityError(String),
    #[error("Could not generate instructions")]
    NoInstructions,
    #[error("Could not fetch Amm info")]
    CouldNotFetchAmmInfo,
}
