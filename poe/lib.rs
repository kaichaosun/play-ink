#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod poe {

    use ink_storage::{traits::SpreadAllocate, Mapping};

    /// Defines the storage of your contract.
    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct Poe {
        /// Stores the proof which includes hash of content and its owner.
        proofs: Mapping<Hash, AccountId>,
    }

    /// Errors that can occur in the contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if claim exists already.
        ClaimAlreadyExist,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Event emitted when a proof is created.
    #[ink(event)]
    pub struct ClaimCreated {
        #[ink(topic)]
        claim: Hash,
        #[ink(topic)]
        owner: AccountId,
    }

    impl Poe {
        /// Initate a new contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping` of our contract.
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// Create a proof with claim being the hash of the content,
        /// and the owner is the caller of this message.
        #[ink(message)]
        pub fn create_claim(&mut self, claim: Hash) -> Result<()> {
            let caller = self.env().caller();
            if self.proofs.contains(&claim) {
                return Err(Error::ClaimAlreadyExist);
            }

            self.proofs.insert(&claim, &caller);
            self.env().emit_event(ClaimCreated {
                claim,
                owner: caller,
            });

            Ok(())
        }

        /// Get the owner of the provided claim.
        #[ink(message)]
        pub fn get_owner(&self, claim: Hash) -> Option<AccountId> {
            self.proofs.get(&claim)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let poe = Poe::default();
            assert_eq!(poe.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut poe = Poe::new(false);
            assert_eq!(poe.get(), false);
            poe.flip();
            assert_eq!(poe.get(), true);
        }
    }
}
