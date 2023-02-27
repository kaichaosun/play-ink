---
---

# Create a PoE Contract

PoE stands for Proof of Existence, it's used to record the owner and time of specific thing on chain. Blockchain is a resouce constraint environment, to record every espect of the thing is unpractical. Usually we only record the hash and some valuable metadata of the content onchain to reduce the cost.

## Initialize our source file by adding compile flags

```rust
#![cfg_attr(not(feature = "std"), no_std)]
```
It means there are two options to compile this file, one is using std feature, if std feature is not given when compile, then it's no_std. When compiling no_std, it targets for WASM compatible environment, Rust standard library is not support by default.

## Create poe module

```rust
#[ink::contract]
mod poe {
    // -- snippet --
}
```

Macro `ink::contract` tells the compiler we are creating a module for ink contract, the compiler will add extra functions and types to make the module recognized by the Substrate contracts pallets, so that the contract pallet can run the contract code after deployment.

## Define storage

Import the mapping data type we need,

```rust
use ink::storage::Mapping;
```

```rust
#[ink(storage)]
#[derive(Default)]
pub struct Poe {
    /// Stores the proof which includes hash of content and its owner.
    proofs: Mapping<Hash, AccountId>,
}
```

The default implementation can be used later when constructing a new contract state.

## Contract call implementation

```rust
impl Poe {
    // -- snippet --
}
```

Initiate a new contract, a constructor, you can have many constructors as you want.

```rust
#[ink(constructor)]
pub fn new() -> Self {
    let proofs = Mapping::default();
    
    Self {
        proofs
    }
}
```

Since we derive the default implementation, we can actually simply write `Self.default()`.

Create a claim with provided hash, the owner is the caller of the message.

```rust
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
```

Here we need to define errors used,

```rust
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub enum Error {
    /// Returned if claim exists already.
    ClaimAlreadyExist,
}
```

And events,

```rust
#[ink(event)]
pub struct ClaimCreated {
    #[ink(topic)]
    claim: Hash,
    #[ink(topic)]
    owner: AccountId,
}
```

What if we want to revoke the claim,

```rust
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
```

Now try to transfer a claim

```rust
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
```

And a help readonly message to show the owner

```rust
#[ink(message)]
pub fn get_owner(&self, claim: Hash) -> Option<AccountId> {
    self.proofs.get(&claim)
}
```
