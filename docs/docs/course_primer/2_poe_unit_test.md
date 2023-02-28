---
---

# Contract Unit Test

Unit test ensure the logic expressed in code is expected, it's the fundamental way to secure your smart contract. As smart contract is deployed onchain publicly, small bug could result into a disaster for your business. Improve the test coverage as much as you can.

First we need import the definitions in our contract,

```rust
use super::*;
```

Create some test helper functions,

```rust
fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
    ink::env::test::default_accounts::<Environment>()
}

fn set_next_caller(caller: AccountId) {
    ink::env::test::set_caller::<Environment>(caller);
}
```

Test `create_claim` function,

```rust
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
```

Test `revoke_claim` function,

```rust
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
```

Test `transfer_claim` function,

```rust
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
```

TODO video needed
