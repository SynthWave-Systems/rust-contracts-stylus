use std::process::Command;

use alloy::primitives::Address;
use alloy_primitives::U128;
use e2e::{Account, ReceiptExt};
use eyre::WrapErr;
use koba::config::{Deploy, Generate, PrivateKey};
use serde::Deserialize;

pub mod access_control;
pub mod erc1155;
pub mod erc1155_metadata_uri;
pub mod erc1155_supply;
pub mod erc20;
pub mod erc721;
pub mod merkle_proofs;
pub mod ownable;
pub mod poseidon;
pub mod poseidon_asm_sol;
pub mod poseidon_sol;
pub mod report;
pub mod vesting_wallet;

#[derive(Debug, Deserialize)]
struct ArbOtherFields {
    #[serde(rename = "gasUsedForL1")]
    gas_used_for_l1: U128,
    #[allow(dead_code)]
    #[serde(rename = "l1BlockNumber")]
    l1_block_number: String,
}

/// Optimisation options for the contract.
///
/// Cache or cache optimized WASM.
#[derive(Clone)]
pub enum Opt {
    None,
    Cache,
    CacheWasmOpt,
}

async fn deploy(
    account: &Account,
    contract_name: &str,
    args: Option<String>,
    opt: Opt,
) -> eyre::Result<Address> {
    let manifest_dir =
        std::env::current_dir().context("should get current dir from env")?;

    let contract_type = match opt {
        Opt::CacheWasmOpt => "example_opt",
        Opt::None | Opt::Cache => "example",
    };

    let wasm_path = manifest_dir
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(format!(
            "{}_{}.wasm",
            contract_name.replace('-', "_"),
            contract_type
        ));
    let sol_path = args.as_ref().map(|_| {
        manifest_dir
            .join("examples")
            .join(contract_name)
            .join("src")
            .join("constructor.sol")
    });

    let pk = account.pk();
    let config = Deploy {
        generate_config: Generate {
            wasm: wasm_path.clone(),
            sol: sol_path,
            args,
            legacy: false,
        },
        auth: PrivateKey {
            private_key_path: None,
            private_key: Some(pk),
            keystore_path: None,
            keystore_password_path: None,
        },
        endpoint: env("RPC_URL")?,
        deploy_only: false,
        quiet: true,
    };

    let address = koba::deploy(&config)
        .await
        .expect("should deploy contract")
        .address()?;

    match opt {
        Opt::Cache | Opt::CacheWasmOpt => {
            cache_contract(account, address, 0)?;
        }
        Opt::None => {}
    }

    Ok(address)
}

/// Try to cache a contract on the stylus network.
/// Already cached contracts won't be cached, and this function will not return
/// an error.
/// Output will be forwarded to the child process.
fn cache_contract(
    account: &Account,
    contract_addr: Address,
    bid: u32,
) -> eyre::Result<()> {
    // We don't need a status code.
    // Since it is not zero when the contract is already cached.
    Command::new("cargo")
        .args(["stylus", "cache", "bid"])
        .args(["-e", &env("RPC_URL")?])
        .args(["--private-key", &format!("0x{}", account.pk())])
        .arg(contract_addr.to_string())
        .arg(bid.to_string())
        .status()
        .context("failed to execute `cargo stylus cache bid` command")?;
    Ok(())
}

/// Load the `name` environment variable.
fn env(name: &str) -> eyre::Result<String> {
    std::env::var(name).wrap_err(format!("failed to load {name}"))
}
