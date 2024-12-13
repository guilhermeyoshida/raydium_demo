use clap::{Parser, Subcommand};

#[derive(Parser, PartialEq, Debug)]
#[command(arg_required_else_help = true)]
pub struct App {
    #[arg(long)]
    pub group_tx: bool,
    #[command(subcommand)]
    pub commands: Option<AppCommands>,
}

// _pool_id: &Pubkey,
// _pool_info: &AmmInfo,
// _amount: u64,

#[derive(Subcommand, PartialEq, Debug)]
pub enum AppCommands {
    Add {
        pool_id: String,
        amount: u64,
    },
    Remove {
        pool_id: String,
        amount: u64,
        slippage_limit: f64,
    },
}
