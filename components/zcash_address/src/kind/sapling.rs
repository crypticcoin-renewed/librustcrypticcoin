/// The HRP for a Bech32-encoded mainnet Sapling address.
///
/// Defined in the [Crypticcoin Protocol Specification section 5.6.4][saplingpaymentaddrencoding].
///
/// [saplingpaymentaddrencoding]: https://zips.z.cash/protocol/protocol.pdf#saplingpaymentaddrencoding
pub(crate) const MAINNET: &str = "cs";

/// The HRP for a Bech32-encoded testnet Sapling address.
///
/// Defined in the [Crypticcoin Protocol Specification section 5.6.4][saplingpaymentaddrencoding].
///
/// [saplingpaymentaddrencoding]: https://zips.z.cash/protocol/protocol.pdf#saplingpaymentaddrencoding
pub(crate) const TESTNET: &str = "ctestsapling";

/// The HRP for a Bech32-encoded regtest Sapling address.
///
/// It is defined in [the `crypticcoind` codebase].
///
/// [the `crypticcoind` codebase]: https://github.com/crypticcoin/crypticcoin/blob/128d863fb8be39ee294fda397c1ce3ba3b889cb2/src/chainparams.cpp#L493
pub(crate) const REGTEST: &str = "cregtestsapling";

pub(crate) type Data = [u8; 43];
