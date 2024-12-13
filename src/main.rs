use std::str::FromStr;

use anyhow::Context;
use clap::Parser;
use cli::App;
use client::ClientHelper;
use env_logger::Env;
use error::AppResult;
use raydium::RaydiumClient;
use solana_sdk::pubkey::Pubkey;

mod cli;
mod client;
mod config;
mod error;
mod raydium;

fn main() -> AppResult<()> {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let _opts = App::parse();

    let client = ClientHelper::default();

    let raydium_client = RaydiumClient::new(client.clone());

    match _opts.commands {
        Some(cli::AppCommands::Add { pool_id, amount }) => {
            let pool_id =
                Pubkey::from_str(&pool_id).context("Provided pool_id is not a valid public key")?;
            let pool_info = raydium::get_pool_info(&client.client, &pool_id)?;

            raydium_client.add_liquidity(&pool_id, &pool_info, amount)?
        }
        Some(cli::AppCommands::Remove {
            pool_id,
            amount,
            slippage_limit,
        }) => {
            let pool_id = Pubkey::from_str(&pool_id)?;
            let pool_info = raydium::get_pool_info(&client.client, &pool_id)?;

            raydium_client.remove_liquidity(&pool_id, &pool_info, amount, slippage_limit)?
        }
        _ => (),
    }

    Ok(())
}
