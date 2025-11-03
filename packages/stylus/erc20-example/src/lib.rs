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
use alloc::collections::BTreeMap;
use openzeppelin_stylus::utils::pausable::{self, Pausable};



const DECIMALS: U8 = uint!(10_U8);

#[entrypoint]
#[storage]
pub struct EMtoken {
    erc20: Erc20,
    metadata: Erc20Metadata,
    capped: Capped,
    owner: Address,
    user_levels: BTreeMap<Address, U256>,
    pausable: Pausable,



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

     pub fn reward_user(&mut self, account: Address, value: U256) -> Result<(), String> {
        let current = *self.user_levels.get(&account).unwrap_or(&U256::from(0));
        let new_level = current + U256::from(1);
        self.user_levels.insert(account, new_level);
        if new_level % U256::from(3) == U256::from(0) {
            self.mint(account, value)?;
        }
        Ok(())
    }

    pub fn get_user_level(&self, account: Address) -> U256 {
        *self.user_levels.get(&account).unwrap_or(&U256::from(0))
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
