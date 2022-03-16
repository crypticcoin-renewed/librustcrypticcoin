/// The prefix for a Base58Check-encoded mainnet transparent P2SH address.
pub(crate) const MAINNET: [u8; 2] = [0x13, 0xbb];

/// The prefix for a Base58Check-encoded testnet transparent P2SH address.
pub(crate) const TESTNET: [u8; 2] = [0x0e, 0xaa];

pub(crate) type Data = [u8; 20];
