/// The prefix for a Base58Check-encoded mainnet Sprout address.
///
/// Defined in the [Crypticcoin Protocol Specification section 5.6.3][sproutpaymentaddrencoding].
///
/// [sproutpaymentaddrencoding]: https://zips.z.cash/protocol/protocol.pdf#sproutpaymentaddrencoding
pub(crate) const MAINNET: [u8; 2] = [0xb7, 0xa1];

/// The prefix for a Base58Check-encoded testnet Sprout address.
///
/// Defined in the [Crypticcoin Protocol Specification section 5.6.3][].
///
/// []: https://zips.z.cash/protocol/protocol.pdf#sproutpaymentaddrencoding
pub(crate) const TESTNET: [u8; 2] = [0x04, 0x96];

pub(crate) type Data = [u8; 64];
