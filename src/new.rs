use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_storage::collections::HashMap;

    /// The ERC20 token contract.
    #[ink(storage)]
    pub struct Erc20 {
        /// The total supply of tokens.
        total_supply: Balance,
        /// The mapping of balances.
        balances: HashMap<AccountId, Balance>,
        /// The mapping of allowances.
        allowances: HashMap<(AccountId, AccountId), Balance>,
    }

    impl Erc20 {
        /// Creates a new ERC20 token contract with the given initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut balances = HashMap::new();
            let caller = Self::env().caller();
            balances.insert(caller, initial_supply);
            Self {
                total_supply: initial_supply,
                balances,
                allowances: HashMap::new(),
            }
        }

        /// Returns the total supply of tokens.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the balance of the given account.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or(&0).clone()
        }

        /// Transfers tokens from the caller's account to the given account.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Erc20Error> {
            let caller = Self::env().caller();
            self.transfer_impl(caller, to, value)
        }

        /// Approves the given account to spend tokens on behalf of the caller.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Erc20Error> {
            let caller = Self::env().caller();
            self.approve_impl(caller, spender, value)
        }

        /// Transfers tokens from the given account to the given recipient, using the allowance mechanism.
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), Erc20Error> {
            let caller = Self::env().caller();
            self.transfer_from_impl(from, to, caller, value)
        }

        /// Returns the allowance of the given account to spend tokens on behalf of the given owner.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).unwrap_or(&0).clone()
        }

        /// Internal implementation of the transfer function.
        fn transfer_impl(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), Erc20Error> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Erc20Error::InsufficientBalance);
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);
            Ok(())
        }

        /// Internal implementation of the approve function.
        fn approve_impl(&mut self, owner: AccountId, spender: AccountId, value: Balance) -> Result<(), Erc20Error> {
            self.allowances.insert((owner, spender), value);
            Ok(())
        }

        /// Internal implementation of the transfer_from function.
        fn transfer_from_impl(&mut self, from: AccountId, to: AccountId, spender: AccountId, value: Balance) -> Result<(), Erc20Error> {
            let allowance = self.allowance(from, spender);
            if allowance < value {
                return Err(Erc20Error::AllowanceExceeded);
            }
            self.transfer_impl(from, to, value)?;
            self.allowances.insert((from, spender), allowance - value);
            Ok(())
        }
    }

    /// Custom error type for the ERC20 contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Erc20Error {
        InsufficientBalance,
        AllowanceExceeded,
    }
}