# ERC20 Smart Contract

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
        allowances: ink_storage::collections::HashMap<(AccountId, AccountId), Balance>,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut balances = ink_storage::collections::HashMap::new();
            let caller = Self::env().caller();
            balances.insert(caller, initial_supply);
            Self {
                total_supply: initial_supply,
                balances,
                allowances: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), &'static str> {
            let caller = Self::env().caller();
            let caller_balance = self.balance_of(caller);
            if caller_balance < value {
                return Err("Insufficient balance");
            }
            self.balances.insert(caller, caller_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);
            Ok(())
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), &'static str> {
            let caller = Self::env().caller();
            self.allowances.insert((caller, spender), value);
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), &'static str> {
            let caller = Self::env().caller();
            let allowance = self.allowances.get(&(from, caller)).unwrap_or(&0);
            if *allowance < value {
                return Err("Allowance exceeded");
            }
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err("Insufficient balance");
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);
            self.allowances.insert((from, caller), allowance - value);
            Ok(())
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            *self.allowances.get(&(owner, spender)).unwrap_or(&0)
        }
    }
}
