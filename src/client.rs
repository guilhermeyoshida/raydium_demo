use anyhow::Result;
use common::{common_utils, CommonConfig};
use log::{debug, error, info};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_response::RpcSimulateTransactionResult;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature, Signer};
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::Account as TokenAccount;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use crate::config::{get_cluster_urls, WAIT_TIME_AFTER_TRANSACTION};

#[derive(Clone)]
pub struct ClientHelper {
    pub client: Rc<RpcClient>,
    pub user_keypair: Rc<Keypair>,
    pub payer: Rc<dyn Signer>,
    pub signing_keypairs: Vec<Rc<dyn Signer>>,
    pub config: CommonConfig,
}

impl Default for ClientHelper {
    fn default() -> Self {
        let mut config = common::CommonConfig::default();
        let (cluster_url, websocket_url) = get_cluster_urls();

        config.set_cluster(&cluster_url, &websocket_url);
        let wallet_f = get_default_wallet_path();
        config.set_wallet(&wallet_f);
        let kp = match common_utils::read_keypair_file(&config.wallet()) {
            Ok(k) => k,
            Err(e) => {
                panic!("{e:?}")
            }
        };

        let client = Rc::new(RpcClient::new(config.cluster().url()));
        let user_keypair = Rc::new(kp);
        let fee_payer = Rc::clone(&user_keypair) as Rc<dyn Signer>;
        let signing_keypairs = vec![Rc::clone(&fee_payer)];
        dbg!(&user_keypair, &fee_payer, &signing_keypairs);

        Self {
            client,
            user_keypair,
            payer: fee_payer,
            signing_keypairs,
            config,
        }
    }
}

pub struct ClientHelperTxResult {
    pub simulation_result: Option<RpcSimulateTransactionResult>,
    pub signature: Option<Signature>,
}

impl ClientHelper {
    pub fn process_transaction(
        &self,
        instructions: &[Instruction],
        dryrun: bool,
    ) -> ClientHelperTxResult {
        let payer_pubkey = self.payer.pubkey();
        let signing_keypairs_refs: Vec<&dyn Signer> =
            self.signing_keypairs.iter().map(|kp| kp.as_ref()).collect();
        let txn = common::build_txn(
            &self.client,
            instructions,
            &payer_pubkey,
            &signing_keypairs_refs,
        )
        .unwrap();

        // Always simulate the transaction
        let sim_result = match common::simulate_transaction(
            &self.client,
            &txn,
            false,
            CommitmentConfig::confirmed(),
        ) {
            Ok(result) => Some(result.value),
            Err(e) => {
                error!("\nSimulation Error: {:?}\n\n", e);
                None
            }
        };
        debug!("\nSimulation Result: {:#?}\n\n", sim_result);

        // Match on the simulation result to handle success or failure
        let signature = if !dryrun {
            // Proceed to send the transaction if not in dryrun mode
            let sig = common::send_txn(&self.client, &txn, true);
            debug!("\nTransaction Result: {:#?}\n\n", sig);

            // Match on the transaction result to handle success or failure
            match sig {
                Ok(value) => Some(value),
                Err(ref error) => {
                    error!("\nTransaction Error: {:?}\n\n", error);
                    None
                }
            }
        } else {
            None
        };

        ClientHelperTxResult {
            simulation_result: sim_result,
            signature,
        }
    }

    pub fn derive_ata_and_fetch_balance(
        &self,
        wallet_address: &Pubkey,
        mint_address: &Pubkey,
    ) -> Result<u64> {
        let ata = get_associated_token_address(wallet_address, mint_address);
        let account = self.client.get_account(&ata)?;
        let token_account = TokenAccount::unpack(&account.data)?;
        debug!("Token Account: {:?}", token_account);
        Ok(token_account.amount)
    }

    pub fn fetch_token_balance(&self, mint_address: &Pubkey) -> Result<u64> {
        let account = self.client.get_account(mint_address)?;
        let token_account = TokenAccount::unpack(&account.data)?;
        debug!("Token Account: {:?}", token_account);
        Ok(token_account.amount)
    }

    // yes using timing is not ideal, will refactor later
    pub fn tests_wait_for_confirmation(&self) {
        let mut remaining_time = WAIT_TIME_AFTER_TRANSACTION;
        info!("Please wait a bit to stabilize the tests and all the accounts fetching");
        while remaining_time > 0 {
            print!("\rWaiting for {} seconds after transaction", remaining_time);
            std::io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
            remaining_time -= 1;
        }
        println!("\n");
    }
}

fn get_default_wallet_path() -> String {
    let home_dir = env::var("HOME").expect("Could not find home directory");

    let wallet_path = env::var("SOLANA_WALLET").unwrap_or_else(|_| ".config/solana/id.json".into());

    PathBuf::from(home_dir)
        .join(wallet_path)
        .to_string_lossy()
        .into_owned()
}
