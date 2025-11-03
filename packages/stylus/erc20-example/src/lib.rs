#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use openzeppelin_stylus::{
    token::erc20::{
        self,
        extensions::{capped, Capped, Erc20Metadata},
        Erc20,
    },
};
use stylus_sdk::{
    alloy_primitives::{uint, Address, U256, U8},
    prelude::*,
};
use alloc::string::String;

const DECIMALS: U8 = uint!(10_U8);

#[entrypoint]
#[storage]
pub struct EMtoken {
    erc20: Erc20,
    metadata: Erc20Metadata,
    capped: Capped,
    owner: Address,

}

#[public]
impl EMtoken {
    #[constructor]
    pub fn constructor(&mut self, name: String, symbol: String, cap: U256) {
        self.metadata.constructor(name, symbol);
        self.capped.constructor(cap).unwrap();
        self.owner = stylus_sdk::msg::sender();

    }

    fn only_owner(&self) -> Result<(), String> {
        if stylus_sdk::msg::sender() != self.owner {
            Err("Only owner can call this function".into())
        } else {
            Ok(())
        }
    }
    pub fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), String> {
        self.only_owner()?;
        self.owner = new_owner;
        Ok(())
    }

    pub fn mint(&mut self, account: Address, value: U256) -> Result<(), String> {
        self.only_owner()?;
        self.erc20._mint(account, value).unwrap();
        Ok(())
    }

    pub fn total_supply(&self) -> U256 {
        self.erc20.total_supply()
    }

    pub fn name(&self) -> String {
        self.metadata.name()
    }

    pub fn symbol(&self) -> String {
        self.metadata.symbol()
    }

    pub fn decimals(&self) -> U8 {
        DECIMALS
    }
}
