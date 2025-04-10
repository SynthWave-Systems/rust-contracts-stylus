#![cfg_attr(not(test), no_main)]
extern crate alloc;

use openzeppelin_stylus::finance::vesting_wallet::VestingWallet;
use stylus_sdk::prelude::*;

#[entrypoint]
#[storage]
struct VestingWalletExample {
    #[borrow]
    vesting_wallet: VestingWallet,
}

#[public]
#[inherit(VestingWallet)]
impl VestingWalletExample {
    #[receive]
    fn receive(&mut self) -> Result<(), Vec<u8>> {
        Ok(())
    }
}
