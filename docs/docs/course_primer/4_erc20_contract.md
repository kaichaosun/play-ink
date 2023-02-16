---
---

# Create ERC-20 contract

## Fungible assets.

Blockchain is known to make decentralized finance in reality. You won't enjoy finance if assets is not avaliable in the blockchain island.
Fungible assets is used to represent valuable pieces which share almost same inherent features. Some typical example is money, 1 dollar is always equal to another 1 dollar. It's straightword when we are talking about money, because money is just nothing. What if it's an apple, in some context they are equal, say I eat an apple, my son eat another apple, in some context they are just not equal, say I eat a red apple, but my son eat a green one. For apple, it's not a typical fungible asset, you need offchain context to determine the concrete state.

When we talking about fungible assets, it just means the onchain assets and such assets always tell no difference.

## ERC-20

The world is changing like a magic, and it's hard to reach a consensus between different parties, as people share different background, interest, goal. If we want to improve the communication, we better share same terms and intefaces before going deep into communition. Such interfaces usually called `specification` in software development. Bitcoin and Ethereum maintains a lot specification to define the must-have features in blockchain, such as [BIP-32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki), [ERC-20](https://eips.ethereum.org/EIPS/eip-20)

ERC-20 defines a few methods and events, if a contract implements such methods and events, the contract is called a ERC-20 compatible contract. Method here means some code logic which can be called by sending a transaction. Event means the message to notify offchain system if the transaction is success or not. When a dApp deploy its ERC-20 contract, others like wallet provider Metamask, Ledger could easily support its assets, blockchain explorers can easily index your asset transactions history, other dApps like Uniwap can easily add another pool to swap your assets with another ERC-20 asset.

Method defines in ERC-20,

```solidity
function name() public view returns (string)
function symbol() public view returns (string)
function decimals() public view returns (uint8)
function totalSupply() public view returns (uint256)
function balanceOf(address _owner) public view returns (uint256 balance)
function transfer(address _to, uint256 _value) public returns (bool success)
function transferFrom(address _from, address _to, uint256 _value) public returns (bool success)
function approve(address _spender, uint256 _value) public returns (bool success)
function allowance(address _owner, address _spender) public view returns (uint256 remaining)
```

Events defined in ERC-20,

```solidity
event Transfer(address indexed _from, address indexed _to, uint256 _value)
event Approval(address indexed _owner, address indexed _spender, uint256 _value)
```

## Implementation in ink

Storage,

```rust
#[ink(storage)]
#[derive(Default)]
pub struct Erc20 {
    /// Total token supply.
    total_supply: Balance,
    /// Mapping from owner to number of owned token.
    balances: Mapping<AccountId, Balance>,
    /// Mapping of the token amount which an account is allowed to withdraw
    /// from another account.
    allowances: Mapping<(AccountId, AccountId), Balance>,
}
```

Events,

```rust
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: Option<AccountId>,
    #[ink(topic)]
    to: Option<AccountId>,
    value: Balance,
}

/// Event emitted when an approval occurs that `spender` is allowed to withdraw
/// up to the amount of `value` tokens from `owner`.
#[ink(event)]
pub struct Approval {
    #[ink(topic)]
    owner: AccountId,
    #[ink(topic)]
    spender: AccountId,
    value: Balance,
}
```

Error,

```rust
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
}

pub type Result<T> = core::result::Result<T, Error>;
```

Query and transfer methods,

```rust
/// Creates a new ERC-20 contract with the specified initial supply.
/// This is like preset extension from openzipplin
#[ink(constructor)]
pub fn new(total_supply: Balance) -> Self {
    let mut balances = Mapping::default();
    let caller = Self::env().caller();
    balances.insert(caller, &total_supply);
    Self::env().emit_event(Transfer {
        from: None,
        to: Some(caller),
        value: total_supply,
    });
    Self {
        total_supply,
        balances,
        allowances: Default::default(),
    }
}

/// Returns the total token supply.
#[ink(message)]
pub fn total_supply(&self) -> Balance {
    self.total_supply
}

/// Returns the account balance for the specified `owner`.
///
/// Returns `0` if the account is non-existent.
#[ink(message)]
pub fn balance_of(&self, owner: AccountId) -> Balance {
    self.balance_of_impl(&owner)
}

/// Returns the account balance for the specified `owner`.
///
/// Returns `0` if the account is non-existent.
///
/// # Note
///
/// Prefer to call this method over `balance_of` since this
/// works using references which are more efficient in Wasm.
#[inline]
fn balance_of_impl(&self, owner: &AccountId) -> Balance {
    self.balances.get(owner).unwrap_or_default()
}

/// Returns the amount which `spender` is still allowed to withdraw from `owner`.
///
/// Returns `0` if no allowance has been set.
#[ink(message)]
pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
    self.allowance_impl(&owner, &spender)
}

/// Returns the amount which `spender` is still allowed to withdraw from `owner`.
///
/// Returns `0` if no allowance has been set.
///
/// # Note
///
/// Prefer to call this method over `allowance` since this
/// works using references which are more efficient in Wasm.
#[inline]
fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
    self.allowances.get((owner, spender)).unwrap_or_default()
}

/// Transfers `value` amount of tokens from the caller's account to account `to`.
///
/// On success a `Transfer` event is emitted.
///
/// # Errors
///
/// Returns `InsufficientBalance` error if there are not enough tokens on
/// the caller's account balance.
#[ink(message)]
pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
    let from = self.env().caller();
    self.transfer_from_to(&from, &to, value)
}

/// Allows `spender` to withdraw from the caller's account multiple times, up to
/// the `value` amount.
///
/// If this function is called again it overwrites the current allowance with `value`.
///
/// An `Approval` event is emitted.
#[ink(message)]
pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
    let owner = self.env().caller();
    self.allowances.insert((&owner, &spender), &value);
    self.env().emit_event(Approval {
        owner,
        spender,
        value,
    });
    Ok(())
}

/// Transfers `value` tokens on the behalf of `from` to the account `to`.
///
/// This can be used to allow a contract to transfer tokens on ones behalf and/or
/// to charge fees in sub-currencies, for example.
///
/// On success a `Transfer` event is emitted.
///
/// # Errors
///
/// Returns `InsufficientAllowance` error if there are not enough tokens allowed
/// for the caller to withdraw from `from`.
///
/// Returns `InsufficientBalance` error if there are not enough tokens on
/// the account balance of `from`.
#[ink(message)]
pub fn transfer_from(
    &mut self,
    from: AccountId,
    to: AccountId,
    value: Balance,
) -> Result<()> {
    let caller = self.env().caller();
    let allowance = self.allowance_impl(&from, &caller);
    if allowance < value {
        return Err(Error::InsufficientAllowance)
    }
    self.transfer_from_to(&from, &to, value)?;
    self.allowances
        .insert((&from, &caller), &(allowance - value));
    Ok(())
}

/// Transfers `value` amount of tokens from the caller's account to account `to`.
///
/// On success a `Transfer` event is emitted.
///
/// # Errors
///
/// Returns `InsufficientBalance` error if there are not enough tokens on
/// the caller's account balance.
fn transfer_from_to(
    &mut self,
    from: &AccountId,
    to: &AccountId,
    value: Balance,
) -> Result<()> {
    let from_balance = self.balance_of_impl(from);
    if from_balance < value {
        return Err(Error::InsufficientBalance)
    }

    self.balances.insert(from, &(from_balance - value));
    let to_balance = self.balance_of_impl(to);
    self.balances.insert(to, &(to_balance + value));
    self.env().emit_event(Transfer {
        from: Some(*from),
        to: Some(*to),
        value,
    });
    Ok(())
}
```

## Resources

- https://ethereum.org/en/developers/docs/standards/tokens/erc-20/
