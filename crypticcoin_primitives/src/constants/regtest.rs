//! # Regtest constants
//!
//! `regtest` is a `crypticcoind`-specific environment used for local testing. They mostly reuse
//! the testnet constants.
//! These constants are defined in [the `crypticcoind` codebase].
//!
//! [the `crypticcoind` codebase]: <https://github.com/crypticcoin/crypticcoin/blob/128d863fb8be39ee294fda397c1ce3ba3b889cb2/src/chainparams.cpp#L482-L496>

/// The regtest cointype reuses the testnet cointype
pub const COIN_TYPE: u32 = 1;

/// The HRP for a Bech32-encoded regtest [`ExtendedSpendingKey`].
///
/// It is defined in [the `crypticcoind` codebase].
///
/// [`ExtendedSpendingKey`]: crate::zip32::ExtendedSpendingKey
/// [the `crypticcoind` codebase]: <https://github.com/crypticcoin/crypticcoin/blob/128d863fb8be39ee294fda397c1ce3ba3b889cb2/src/chainparams.cpp#L496>
pub const HRP_SAPLING_EXTENDED_SPENDING_KEY: &str = "secret-extended-key-regtest";

/// The HRP for a Bech32-encoded regtest [`ExtendedFullViewingKey`].
///
/// It is defined in [the `crypticcoind` codebase].
///
/// [`ExtendedFullViewingKey`]: crate::zip32::ExtendedFullViewingKey
/// [the `crypticcoind` codebase]: <https://github.com/crypticcoin/crypticcoin/blob/128d863fb8be39ee294fda397c1ce3ba3b889cb2/src/chainparams.cpp#L494>
pub const HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY: &str = "cxviewregtestsapling";

/// The HRP for a Bech32-encoded regtest [`PaymentAddress`].
///
/// It is defined in [the `crypticcoind` codebase].
///
/// [`PaymentAddress`]: crate::sapling::PaymentAddress
/// [the `crypticcoind` codebase]: <https://github.com/crypticcoin/crypticcoin/blob/128d863fb8be39ee294fda397c1ce3ba3b889cb2/src/chainparams.cpp#L493>
pub const HRP_SAPLING_PAYMENT_ADDRESS: &str = "cregtestsapling";

/// The prefix for a Base58Check-encoded regtest [`TransparentAddress::PublicKey`].
/// Same as the testnet prefix.
///
/// [`TransparentAddress::PublicKey`]: crate::legacy::TransparentAddress::PublicKey
pub const B58_PUBKEY_ADDRESS_PREFIX: [u8; 2] = [0x1d, 0x25];

/// The prefix for a Base58Check-encoded regtest [`TransparentAddress::Script`].
/// Same as the testnet prefix.
///
/// [`TransparentAddress::Script`]: crate::legacy::TransparentAddress::Script
pub const B58_SCRIPT_ADDRESS_PREFIX: [u8; 2] = [0x1c, 0xba];
