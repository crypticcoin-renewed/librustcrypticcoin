/// The prefix for a Base58Check-encoded mainnet transparent P2PKH address.
pub(crate) const MAINNET: [u8; 2] = [0x13, 0xb6];

/// The prefix for a Base58Check-encoded testnet transparent P2PKH address.
pub(crate) const TESTNET: [u8; 2] = [0x0e, 0xa4];

pub(crate) type Data = [u8; 20];
