use std::iter;

use crate::{
    unified::{
        self,
        address::{test_vectors::TEST_VECTORS, Receiver},
    },
    Network, ToAddress, CrypticcoinAddress,
};

#[test]
fn unified() {
    for tv in TEST_VECTORS {
        // Double-check test vectors match requirements:
        // - Only one of P2PKH and P2SH.
        assert!(tv.p2pkh_bytes.is_none() || tv.p2sh_bytes.is_none());
        // - At least one shielded receiver.
        assert!(tv.sapling_raw_addr.is_some() || tv.orchard_raw_addr.is_some());

        let addr_string = String::from_utf8(tv.unified_addr.to_vec()).unwrap();

        let receivers = iter::empty()
            .chain(tv.p2pkh_bytes.map(Receiver::P2pkh))
            .chain(tv.p2sh_bytes.map(Receiver::P2sh))
            .chain(tv.sapling_raw_addr.map(Receiver::Sapling))
            .chain(tv.orchard_raw_addr.map(Receiver::Orchard))
            .collect();

        let expected_addr = CrypticcoinAddress::from_unified(Network::Main, unified::Address(receivers));

        // Test parsing
        let addr: CrypticcoinAddress = addr_string.parse().unwrap();
        assert_eq!(addr, expected_addr);

        // Test serialization
        assert_eq!(expected_addr.to_string(), addr_string);
    }
}
