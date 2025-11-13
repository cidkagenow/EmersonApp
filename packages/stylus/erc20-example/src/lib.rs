#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use alloc::{vec::Vec, string::String};

use openzeppelin_stylus::{
    token::erc20::{
        self,
        extensions::{capped, Capped, Erc20Metadata, ICapped, IErc20Burnable, IErc20Metadata},
        Erc20, IErc20,
    },
    utils::{introspection::erc165::IErc165, pausable, IPausable, Pausable},
};
use stylus_sdk::{
    alloy_primitives::{aliases::B32, uint, Address, U256, U8},
    prelude::*,
};

const DECIMALS: U8 = uint!(10_U8);

#[derive(SolidityError, Debug)]
enum Error {
    ExceededCap(capped::ERC20ExceededCap),
    InvalidCap(capped::ERC20InvalidCap),
    InsufficientBalance(erc20::ERC20InsufficientBalance),
    InvalidSender(erc20::ERC20InvalidSender),
    InvalidReceiver(erc20::ERC20InvalidReceiver),
    InsufficientAllowance(erc20::ERC20InsufficientAllowance),
    InvalidSpender(erc20::ERC20InvalidSpender),
    InvalidApprover(erc20::ERC20InvalidApprover),
    EnforcedPause(pausable::EnforcedPause),
    ExpectedPause(pausable::ExpectedPause),
    NotOwner, 
}

impl From<capped::Error> for Error {
    fn from(value: capped::Error) -> Self {
        match value {
            capped::Error::ExceededCap(e) => Error::ExceededCap(e),
            capped::Error::InvalidCap(e) => Error::InvalidCap(e),
        }
    }
}

impl From<erc20::Error> for Error {
    fn from(value: erc20::Error) -> Self {
        match value {
            erc20::Error::InsufficientBalance(e) => Error::InsufficientBalance(e),
            erc20::Error::InvalidSender(e) => Error::InvalidSender(e),
            erc20::Error::InvalidReceiver(e) => Error::InvalidReceiver(e),
            erc20::Error::InsufficientAllowance(e) => Error::InsufficientAllowance(e),
            erc20::Error::InvalidSpender(e) => Error::InvalidSpender(e),
            erc20::Error::InvalidApprover(e) => Error::InvalidApprover(e),
        }
    }
}

impl From<pausable::Error> for Error {
    fn from(value: pausable::Error) -> Self {
        match value {
            pausable::Error::EnforcedPause(e) => Error::EnforcedPause(e),
            pausable::Error::ExpectedPause(e) => Error::ExpectedPause(e),
        }
    }
}

#[entrypoint]
#[storage]
struct Erc20Example {
    erc20: Erc20,
    metadata: Erc20Metadata,
    capped: Capped,
    pausable: Pausable,

    owner: Address,

    levels: Vec<(Address, u64)>,

    reward_amount: U256,
}

#[public]
impl Erc20Example {
    
    #[constructor]
    pub fn constructor(
        &mut self,
        owner: Address,
        name: String,
        symbol: String,
        cap: U256,
        reward_amount: U256,
    ) -> Result<(), Error> {
        self.metadata.constructor(name, symbol);
        self.capped.constructor(cap)?;
        self.owner = owner;
        self.levels = Vec::new();
        self.reward_amount = reward_amount;
        Ok(())
    }

   
    fn ensure_owner(&self) -> Result<(), Error> {
        let caller = env::caller();
        if caller != self.owner {
            return Err(Error::NotOwner);
        }
        Ok(())
    }

 
    pub fn mint_owner(&mut self, to: Address, value: U256) -> Result<(), Error> {
        self.ensure_owner()?;
        self.pausable.when_not_paused()?;

        let max_supply = self.capped.cap();
        let supply = self
            .erc20
            .total_supply()
            .checked_add(value)
            .expect("new supply should not exceed `U256::MAX`");

        if supply > max_supply {
            return Err(capped::Error::ExceededCap(capped::ERC20ExceededCap {
                increased_supply: supply,
                cap: max_supply,
            }))?;
        }

        self.erc20._mint(to, value)?;
        Ok(())
    }

    pub fn burn_from_by_owner(&mut self, account: Address, value: U256) -> Result<(), Error> {
        self.ensure_owner()?;
        self.pausable.when_not_paused()?;
        self.erc20._burn(account, value)?;
        Ok(())
    }


    pub fn pause(&mut self) -> Result<(), Error> {
        self.ensure_owner()?;
        Ok(self.pausable.pause()?)
    }

    pub fn unpause(&mut self) -> Result<(), Error> {
        self.ensure_owner()?;
        Ok(self.pausable.unpause()?)
    }

  
    fn _get_level_index(&self, account: Address) -> Option<usize> {
        for (i, (addr, _lvl)) in self.levels.iter().enumerate() {
            if *addr == account {
                return Some(i);
            }
        }
        None
    }

    pub fn level_of(&self, account: Address) -> u64 {
        match self._get_level_index(account) {
            Some(i) => self.levels[i].1,
            None => 0u64,
        }
    }

    pub fn level_up(&mut self, account: Address) -> Result<(), Error> {
        self.ensure_owner()?;
        let idx_opt = self._get_level_index(account);
        if let Some(i) = idx_opt {
            let new_level = self.levels[i].1.checked_add(1).unwrap_or(self.levels[i].1 + 1);
            self.levels[i].1 = new_level;
            if new_level % 3 == 0 {
                self.mint_owner(account, self.reward_amount)?;
            }
        } else {
            self.levels.push((account, 1u64));
        }
        Ok(())
    }

    pub fn set_level(&mut self, account: Address, new_level: u64) -> Result<(), Error> {
        self.ensure_owner()?;
        if let Some(i) = self._get_level_index(account) {
            self.levels[i].1 = new_level;
        } else {
            self.levels.push((account, new_level));
        }
        // if new_level is multiple of 3, mint reward once
        if new_level > 0 && new_level % 3 == 0 {
            self.mint_owner(account, self.reward_amount)?;
        }
        Ok(())
    }

    
    pub fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), Error> {
        self.ensure_owner()?;
        self.owner = new_owner;
        Ok(())
    }

    pub fn owner(&self) -> Address {
        self.owner
    }


    pub fn total_supply(&self) -> U256 {
        self.erc20.total_supply()
    }

    pub fn balance_of(&self, account: Address) -> U256 {
        self.erc20.balance_of(account)
    }

    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Error> {
        self.pausable.when_not_paused()?;
        Ok(self.erc20.transfer(to, value)?)
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.erc20.allowance(owner, spender)
    }

    pub fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Error> {
        Ok(self.erc20.approve(spender, value)?)
    }

    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Error> {
        self.pausable.when_not_paused()?;
        Ok(self.erc20.transfer_from(from, to, value)?)
    }

    pub fn burn(&mut self, value: U256) -> Result<(), Error> {
        self.pausable.when_not_paused()?;
        Ok(self.erc20.burn(value)?)
    }

    pub fn burn_from(&mut self, account: Address, value: U256) -> Result<(), Error> {
        self.pausable.when_not_paused()?;
        Ok(self.erc20.burn_from(account, value)?)
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

    pub fn cap(&self) -> U256 {
        self.capped.cap()
    }

    pub fn paused(&self) -> bool {
        self.pausable.paused()
    }

    pub fn supports_interface(&self, interface_id: B32) -> bool {
        Erc20::supports_interface(&self.erc20, interface_id)
            || Erc20Metadata::supports_interface(&self.metadata, interface_id)
    }
}
