#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod poe {

    use ink::storage::Mapping;

    /// Defines the storage of your contract.
    #[ink(storage)]
    #[derive(Default)]
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
        /// Returned if an account try to change a claim which is not hold.
        NotClaimOwner,
        /// Returned if a claim is required but not exist.
        ClaimNotExist,
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

    /// Event emitted when a proof is revoked
    #[ink(event)]
    pub struct ClaimRevoked {
        #[ink(topic)]
        claim: Hash,
        #[ink(topic)]
        owner: AccountId,
    }

    /// Event emitted when a proof is transferred to a new account
    #[ink(event)]
    pub struct ClaimTransferred {
        #[ink(topic)]
        claim: Hash,
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        to: AccountId,
    }

    impl Poe {
        /// Initate a new contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            let proofs = Mapping::default();
            
            Self {
                proofs
            }
        }

        /// Create a proof with claim being the hash of the content,
        /// and the owner is the caller of this message.
        #[ink(message)]
        pub fn create_claim(&mut self, claim: Hash) -> Result<()> {
            if self.proofs.contains(&claim) {
                return Err(Error::ClaimAlreadyExist);
            }

            let caller = self.env().caller();
            self.proofs.insert(&claim, &caller);

            self.env().emit_event(ClaimCreated {
                claim,
                owner: caller,
            });

            Ok(())
        }

        /// Revoke a proof if the sender is the owner of the claim.
        #[ink(message)]
        pub fn revoke_claim(&mut self, claim: Hash) -> Result<()> {
            let owner = self.proofs.get(&claim).ok_or(Error::ClaimNotExist)?;

            let caller = self.env().caller();
            if caller != owner {
                return Err(Error::NotClaimOwner)
            }

            self.proofs.remove(&claim);

            self.env().emit_event(ClaimRevoked {
                claim,
                owner,
            });
            
            Ok(())
        }

        /// Transfer an existing proof to a new owner.
        #[ink(message)]
        pub fn transfer_claim(&mut self, claim: Hash, to: AccountId) -> Result<()> {
            let owner = self.proofs.get(&claim).ok_or(Error::ClaimNotExist)?;

            let caller = self.env().caller();
            if caller != owner {
                return Err(Error::NotClaimOwner)
            }

            self.proofs.insert(&claim, &to);

            self.env().emit_event(ClaimTransferred {
                claim,
                owner,
                to,
            });

            Ok(())
        }

        /// Get the owner of the provided claim.
        #[ink(message)]
        pub fn get_owner(&self, claim: Hash) -> Option<AccountId> {
            self.proofs.get(&claim)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_next_caller(caller: AccountId) {
            ink::env::test::set_caller::<Environment>(caller);
        }

        #[ink::test]
        fn create_claim_works() {
            let default_accounts = default_accounts();
            let claim = Hash::from([0x99; 32]);

            set_next_caller(default_accounts.alice);
            let mut contract = Poe::new();

            assert_eq!(contract.create_claim(claim), Ok(()));
            assert_eq!(contract.create_claim(claim), Err(Error::ClaimAlreadyExist));

            // get_owner works
            assert_eq!(contract.get_owner(claim), Some(default_accounts.alice));
        }

        #[ink::test]
        fn revoke_claim_works() {
            let default_accounts = default_accounts();
            let claim = Hash::from([0x99; 32]);

            set_next_caller(default_accounts.alice);
            let mut contract = Poe::new();

            assert_eq!(contract.revoke_claim(claim), Err(Error::ClaimNotExist));
            assert_eq!(contract.create_claim(claim), Ok(()));

            set_next_caller(default_accounts.bob);
            assert_eq!(contract.revoke_claim(claim), Err(Error::NotClaimOwner));

            set_next_caller(default_accounts.alice);
            assert_eq!(contract.revoke_claim(claim), Ok(()));
        }

        #[ink::test]
        fn transfer_claim_works() {
            let default_accounts = default_accounts();
            let claim = Hash::from([0x99; 32]);

            set_next_caller(default_accounts.alice);
            let mut contract = Poe::new();

            assert_eq!(contract.transfer_claim(claim, default_accounts.bob), Err(Error::ClaimNotExist));
            assert_eq!(contract.create_claim(claim), Ok(()));

            set_next_caller(default_accounts.bob);
            assert_eq!(contract.transfer_claim(claim, default_accounts.charlie), Err(Error::NotClaimOwner));

            set_next_caller(default_accounts.alice);
            assert_eq!(contract.transfer_claim(claim, default_accounts.charlie), Ok(()));

            assert_eq!(contract.get_owner(claim), Some(default_accounts.charlie));
        }

    }
}
