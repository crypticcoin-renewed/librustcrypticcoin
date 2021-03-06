use std::{error::Error, fmt};

use crate::{kind::*, AddressKind, Network, CrypticcoinAddress};

/// An address type is not supported for conversion.
#[derive(Debug)]
pub struct UnsupportedAddress(&'static str);

impl fmt::Display for UnsupportedAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Crypticcoin {} addresses are not supported", self.0)
    }
}

impl Error for UnsupportedAddress {}

/// A helper trait for converting a [`CrypticcoinAddress`] into another type.
///
/// [`CrypticcoinAddress`]: crate::CrypticcoinAddress
///
/// # Examples
///
/// ```
/// use crypticcoin_address::{FromAddress, Network, UnsupportedAddress, CrypticcoinAddress};
///
/// #[derive(Debug)]
/// struct MySapling([u8; 43]);
///
/// // Implement the FromAddress trait, overriding whichever conversion methods match your
/// // requirements for the resulting type.
/// impl FromAddress for MySapling {
///     fn from_sapling(net: Network, data: [u8; 43]) -> Result<Self, UnsupportedAddress> {
///         Ok(MySapling(data))
///     }
/// }
///
/// // For a supported address type, the conversion works.
/// let addr: CrypticcoinAddress =
///     "zs1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpq6d8g"
///         .parse()
///         .unwrap();
/// assert!(addr.convert::<MySapling>().is_ok());
///
/// // For an unsupported address type, we get an error.
/// let addr: CrypticcoinAddress = "t1Hsc1LR8yKnbbe3twRp88p6vFfC5t7DLbs".parse().unwrap();
/// assert_eq!(
///     addr.convert::<MySapling>().unwrap_err().to_string(),
///     "Crypticcoin transparent P2PKH addresses are not supported",
/// );
/// ```
pub trait FromAddress: Sized {
    fn from_sprout(net: Network, data: sprout::Data) -> Result<Self, UnsupportedAddress> {
        let _ = (net, data);
        Err(UnsupportedAddress("Sprout"))
    }

    fn from_sapling(net: Network, data: sapling::Data) -> Result<Self, UnsupportedAddress> {
        let _ = (net, data);
        Err(UnsupportedAddress("Sapling"))
    }

    fn from_unified(net: Network, data: unified::Address) -> Result<Self, UnsupportedAddress> {
        let _ = (net, data);
        Err(UnsupportedAddress("Unified"))
    }

    fn from_transparent_p2pkh(net: Network, data: p2pkh::Data) -> Result<Self, UnsupportedAddress> {
        let _ = (net, data);
        Err(UnsupportedAddress("transparent P2PKH"))
    }

    fn from_transparent_p2sh(net: Network, data: p2sh::Data) -> Result<Self, UnsupportedAddress> {
        let _ = (net, data);
        Err(UnsupportedAddress("transparent P2SH"))
    }
}

/// A helper trait for converting another type into a [`CrypticcoinAddress`].
///
/// This trait is sealed and cannot be implemented for types outside this crate. Its
/// purpose is to move these conversion functions out of the main `CrypticcoinAddress` API
/// documentation, as they are only required when creating addresses (rather than when
/// parsing addresses, which is a more common occurrence).
///
/// [`CrypticcoinAddress`]: crate::CrypticcoinAddress
///
/// # Examples
///
/// ```
/// use crypticcoin_address::{ToAddress, Network, CrypticcoinAddress};
///
/// #[derive(Debug)]
/// struct MySapling([u8; 43]);
///
/// impl MySapling {
///     /// Encodes this Sapling address for the given network.
///     fn encode(&self, net: Network) -> CrypticcoinAddress {
///         CrypticcoinAddress::from_sapling(net, self.0)
///     }
/// }
///
/// let addr = MySapling([0; 43]);
/// let encoded = addr.encode(Network::Main);
/// assert_eq!(
///     encoded.to_string(),
///     "zs1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpq6d8g",
/// );
/// ```
pub trait ToAddress: private::Sealed {
    fn from_sprout(net: Network, data: sprout::Data) -> Self;

    fn from_sapling(net: Network, data: sapling::Data) -> Self;

    fn from_unified(net: Network, data: unified::Address) -> Self;

    fn from_transparent_p2pkh(net: Network, data: p2pkh::Data) -> Self;

    fn from_transparent_p2sh(net: Network, data: p2sh::Data) -> Self;
}

impl ToAddress for CrypticcoinAddress {
    fn from_sprout(net: Network, data: sprout::Data) -> Self {
        CrypticcoinAddress {
            net: if let Network::Regtest = net {
                Network::Test
            } else {
                net
            },
            kind: AddressKind::Sprout(data),
        }
    }

    fn from_sapling(net: Network, data: sapling::Data) -> Self {
        CrypticcoinAddress {
            net,
            kind: AddressKind::Sapling(data),
        }
    }

    fn from_unified(net: Network, data: unified::Address) -> Self {
        CrypticcoinAddress {
            net,
            kind: AddressKind::Unified(data),
        }
    }

    fn from_transparent_p2pkh(net: Network, data: p2pkh::Data) -> Self {
        CrypticcoinAddress {
            net: if let Network::Regtest = net {
                Network::Test
            } else {
                net
            },
            kind: AddressKind::P2pkh(data),
        }
    }

    fn from_transparent_p2sh(net: Network, data: p2sh::Data) -> Self {
        CrypticcoinAddress {
            net: if let Network::Regtest = net {
                Network::Test
            } else {
                net
            },
            kind: AddressKind::P2sh(data),
        }
    }
}

mod private {
    use crate::CrypticcoinAddress;

    pub trait Sealed {}
    impl Sealed for CrypticcoinAddress {}
}
