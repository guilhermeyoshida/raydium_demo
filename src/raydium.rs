use amm_cli::AmmCommands;
use common::CommonConfig;
use raydium_amm::state::AmmInfo;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use std::str::FromStr;

use crate::{
    client::ClientHelper,
    error::{AppError, AppResult},
};

#[allow(dead_code)]
pub struct RaydiumClient {
    client: ClientHelper,
}

impl RaydiumClient {
    pub fn new(client: ClientHelper) -> Self {
        Self { client }
    }

    pub fn add_liquidity(
        &self,
        pool_id: &Pubkey,
        pool_info: &AmmInfo,
        amount: u64,
    ) -> AppResult<()> {
        log::info!("Adding liquidity! Grouped TX? ");

        match build_add_liquidity_ixs(pool_id, pool_info, amount, &self.client.config)? {
            Some(ixs) => self.client.process_transaction(&ixs, false),
            None => return Err(AppError::NoInstructions),
        };

        Ok(())
    }

    pub fn remove_liquidity(
        &self,
        pool_id: &Pubkey,
        pool_info: &AmmInfo,
        amount: u64,
        slippage_limit: f64,
    ) -> AppResult<()> {
        log::info!("Removing liquidity! Grouped TX?");
        match build_remove_liquidity_ixs(
            pool_id,
            pool_info,
            amount,
            slippage_limit,
            &self.client.config,
        )? {
            Some(ixs) => self.client.process_transaction(&ixs, false),
            None => return Err(AppError::NoInstructions),
        };
        Ok(())
    }
}

fn build_remove_liquidity_ixs(
    pool_id: &Pubkey,
    pool_info: &AmmInfo,
    amount: u64,
    slippage_limit: f64,
    config: &CommonConfig,
) -> AppResult<Option<Vec<Instruction>>> {
    if amount == 0 {
        return Err(AppError::AmountCannotBeZero);
    }

    let decimals = pool_info.sys_decimal_value;

    let amount_with_decimals = amount
        .checked_mul(decimals)
        .ok_or_else(|| panic!("Multiplication overflow"))
        .expect("not to fail");
    let cmd = AmmCommands::Withdraw {
        pool_id: *pool_id,
        withdraw_token_lp: None,
        recipient_token_coin: None,
        recipient_token_pc: None,
        input_lp_amount: amount_with_decimals,
        slippage_limit: slippage_limit > 0.0,
    };

    amm_cli::process_amm_commands(cmd, config).map_err(|e| AppError::LiquidityError(e.to_string()))
}

fn build_add_liquidity_ixs(
    pool_id: &Pubkey,
    pool_info: &AmmInfo,
    amount: u64,
    config: &CommonConfig,
) -> AppResult<Option<Vec<Instruction>>> {
    if amount == 0 {
        return Err(AppError::AmountCannotBeZero);
    }

    log::info!("Starting fn build_add_liquidity_ixs");

    let coin_ata = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    let pc_ata = Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap();
    let lp_ata = Pubkey::from_str("YOUR_LP_ATA").unwrap();

    let decimals = pool_info.sys_decimal_value;
    let amount_with_decimals = amount
        .checked_mul(decimals)
        .ok_or_else(|| panic!("Multiplication overflow"))
        .expect("not to fail");

    let cmd = AmmCommands::Deposit {
        pool_id: *pool_id,
        deposit_token_coin: Option::from(coin_ata),
        deposit_token_pc: Option::from(pc_ata),
        recipient_token_lp: None,
        amount_specified: amount_with_decimals,
        another_min_limit: false,
        base_coin: false,
    };
    log::info!("Sending cmd {:?}", cmd);

    amm_cli::process_amm_commands(cmd, config).map_err(|e| AppError::LiquidityError(e.to_string()))
}

pub(crate) fn get_pool_info(client: &RpcClient, pool_id: &Pubkey) -> AppResult<AmmInfo> {
    match common::rpc::get_account::<AmmInfo>(client, pool_id).expect("asda") {
        Some(amm_info) => Ok(amm_info),
        None => Err(AppError::CouldNotFetchAmmInfo),
    }
}
