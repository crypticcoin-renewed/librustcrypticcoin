pub(crate) struct TestVector {
    pub(crate) p2pkh_bytes: Option<[u8; 20]>,
    pub(crate) p2sh_bytes: Option<[u8; 20]>,
    pub(crate) sapling_raw_addr: Option<[u8; 43]>,
    pub(crate) orchard_raw_addr: Option<[u8; 43]>,
    pub(crate) unified_addr: &'static [u8],
}

// From https://github.com/crypticcoin-hackworks/crypticcoin-test-vectors/blob/master/unified_address.py
pub(crate) const TEST_VECTORS: &[TestVector] = &[
    TestVector {
        p2pkh_bytes: None,
        p2sh_bytes: Some([
            0x7a, 0x8f, 0x73, 0x9a, 0x2d, 0x9e, 0x94, 0x5b, 0x0c, 0xe1, 0x52, 0xa8, 0x04, 0x9e,
            0x29, 0x4c, 0x4d, 0x6e, 0x66, 0xb1,
        ]),
        sapling_raw_addr: None,
        orchard_raw_addr: Some([
            0xdc, 0xb1, 0xd2, 0xa3, 0x77, 0x62, 0x14, 0x8d, 0xb4, 0xce, 0xe3, 0xbb, 0xf1, 0x9f,
            0xb1, 0xec, 0x05, 0x89, 0x18, 0x94, 0xb1, 0x38, 0x01, 0xc6, 0x22, 0xba, 0x6a, 0x90,
            0xfa, 0xf1, 0x11, 0x9f, 0x82, 0x24, 0xae, 0x39, 0x85, 0xc6, 0xab, 0xd3, 0xb7, 0xbb,
            0xae,
        ]),
        unified_addr: &[
            0x75, 0x31, 0x36, 0x75, 0x74, 0x37, 0x33, 0x6b, 0x36, 0x34, 0x6a, 0x75, 0x7a, 0x75,
            0x36, 0x6a, 0x75, 0x30, 0x77, 0x61, 0x73, 0x36, 0x71, 0x75, 0x37, 0x67, 0x64, 0x37,
            0x71, 0x7a, 0x30, 0x33, 0x72, 0x61, 0x77, 0x73, 0x68, 0x77, 0x6e, 0x74, 0x6d, 0x30,
            0x30, 0x78, 0x39, 0x37, 0x30, 0x6b, 0x32, 0x63, 0x74, 0x35, 0x6d, 0x77, 0x6a, 0x35,
            0x36, 0x72, 0x64, 0x78, 0x73, 0x78, 0x63, 0x38, 0x38, 0x71, 0x70, 0x6e, 0x35, 0x6b,
            0x76, 0x71, 0x6e, 0x35, 0x33, 0x7a, 0x63, 0x68, 0x6c, 0x6e, 0x74, 0x78, 0x6b, 0x38,
            0x34, 0x78, 0x6b, 0x6b, 0x68, 0x6e, 0x34, 0x66, 0x6a, 0x74, 0x33, 0x64, 0x66, 0x79,
            0x6e, 0x6c, 0x35, 0x76, 0x79, 0x6d, 0x64, 0x66, 0x65, 0x68, 0x30, 0x6d, 0x39, 0x78,
            0x65, 0x30, 0x77, 0x38, 0x30, 0x66, 0x76, 0x6d, 0x6a, 0x6d, 0x70, 0x78, 0x34, 0x6d,
            0x65, 0x66, 0x6a, 0x74, 0x77, 0x6a, 0x38, 0x78, 0x67, 0x6e, 0x67, 0x70, 0x68, 0x77,
            0x70,
        ],
    },
    TestVector {
        p2pkh_bytes: Some([
            0xb3, 0x53, 0x42, 0x01, 0xcf, 0xb1, 0xcd, 0x8d, 0xbf, 0x69, 0xb8, 0x25, 0x0c, 0x18,
            0xef, 0x41, 0x29, 0x4c, 0xa9, 0x79,
        ]),
        p2sh_bytes: None,
        sapling_raw_addr: Some([
            0x90, 0x2b, 0x65, 0x65, 0xa1, 0xc4, 0x4e, 0x7e, 0x7a, 0x08, 0x05, 0x71, 0xaf, 0x1d,
            0xd7, 0x74, 0x69, 0x7c, 0xc1, 0x26, 0xf1, 0xfc, 0x04, 0x35, 0xd3, 0xcd, 0xbf, 0x86,
            0x87, 0x83, 0xe9, 0xfb, 0x46, 0x20, 0xdf, 0x4b, 0xf1, 0x75, 0xcb, 0xf2, 0xc3, 0xe3,
            0x6f,
        ]),
        orchard_raw_addr: Some([
            0x05, 0xf6, 0x12, 0x73, 0xa7, 0x20, 0x12, 0x95, 0x33, 0x2f, 0xee, 0x45, 0x79, 0x47,
            0x45, 0x34, 0x80, 0x9a, 0x0a, 0xeb, 0x81, 0x7a, 0x2b, 0xc0, 0x59, 0x41, 0x66, 0xad,
            0x7a, 0x46, 0x20, 0x67, 0x71, 0x25, 0x33, 0xb6, 0xee, 0xc0, 0xfa, 0x2d, 0x1b, 0xe9,
            0x9f,
        ]),
        unified_addr: &[
            0x75, 0x31, 0x67, 0x72, 0x35, 0x33, 0x37, 0x65, 0x70, 0x6b, 0x32, 0x74, 0x78, 0x6b,
            0x78, 0x7a, 0x74, 0x61, 0x72, 0x37, 0x72, 0x72, 0x76, 0x34, 0x35, 0x70, 0x6b, 0x70,
            0x63, 0x70, 0x65, 0x6c, 0x38, 0x39, 0x6e, 0x61, 0x37, 0x32, 0x6e, 0x38, 0x67, 0x70,
            0x35, 0x72, 0x65, 0x34, 0x39, 0x61, 0x6c, 0x6d, 0x7a, 0x71, 0x34, 0x38, 0x35, 0x6e,
            0x36, 0x72, 0x37, 0x61, 0x33, 0x65, 0x61, 0x34, 0x30, 0x6a, 0x71, 0x32, 0x33, 0x32,
            0x78, 0x37, 0x39, 0x75, 0x37, 0x37, 0x65, 0x64, 0x6b, 0x6c, 0x7a, 0x6e, 0x73, 0x35,
            0x65, 0x66, 0x38, 0x36, 0x30, 0x75, 0x6e, 0x78, 0x32, 0x33, 0x71, 0x39, 0x67, 0x73,
            0x77, 0x72, 0x76, 0x33, 0x6d, 0x33, 0x7a, 0x78, 0x32, 0x6a, 0x76, 0x66, 0x64, 0x61,
            0x66, 0x36, 0x76, 0x75, 0x70, 0x35, 0x35, 0x75, 0x7a, 0x73, 0x33, 0x34, 0x7a, 0x37,
            0x61, 0x75, 0x63, 0x75, 0x38, 0x30, 0x37, 0x67, 0x30, 0x79, 0x6c, 0x6b, 0x75, 0x63,
            0x76, 0x79, 0x76, 0x77, 0x76, 0x35, 0x74, 0x63, 0x79, 0x38, 0x68, 0x34, 0x38, 0x6b,
            0x65, 0x67, 0x67, 0x70, 0x6e, 0x32, 0x72, 0x38, 0x79, 0x70, 0x35, 0x63, 0x6c, 0x70,
            0x36, 0x66, 0x30, 0x32, 0x34, 0x39, 0x36, 0x61, 0x79, 0x73, 0x38, 0x6a, 0x6c, 0x64,
            0x38, 0x6a, 0x35, 0x38, 0x75, 0x67, 0x76, 0x68, 0x65, 0x32, 0x72, 0x78, 0x63, 0x72,
            0x73, 0x77, 0x79, 0x72, 0x6a, 0x6b, 0x66, 0x35, 0x72, 0x6d, 0x37, 0x6d, 0x36, 0x74,
            0x77, 0x79, 0x73,
        ],
    },
    TestVector {
        p2pkh_bytes: None,
        p2sh_bytes: Some([
            0xe8, 0xc7, 0x20, 0x3d, 0x99, 0x6a, 0xf7, 0xd4, 0x77, 0x08, 0x37, 0x56, 0xd5, 0x9a,
            0xf8, 0x0d, 0x06, 0xa7, 0x45, 0xf4,
        ]),
        sapling_raw_addr: None,
        orchard_raw_addr: Some([
            0x4e, 0xa7, 0xd6, 0xb3, 0xdf, 0xa3, 0x38, 0x19, 0x2a, 0xf0, 0x6c, 0xbb, 0xf4, 0x7a,
            0xd4, 0x05, 0x71, 0x5b, 0xc7, 0x83, 0x2b, 0xed, 0xb1, 0x46, 0x62, 0x17, 0xdc, 0x0d,
            0x93, 0x31, 0x4d, 0xe9, 0xf3, 0xc2, 0x5e, 0xec, 0x89, 0xf9, 0xa2, 0x1b, 0xfe, 0x0e,
            0x93,
        ]),
        unified_addr: &[
            0x75, 0x31, 0x6e, 0x6b, 0x35, 0x37, 0x30, 0x61, 0x61, 0x6d, 0x79, 0x34, 0x7a, 0x6d,
            0x68, 0x65, 0x6c, 0x7a, 0x36, 0x6b, 0x61, 0x30, 0x33, 0x66, 0x7a, 0x79, 0x34, 0x73,
            0x37, 0x66, 0x39, 0x72, 0x34, 0x6b, 0x65, 0x76, 0x66, 0x6d, 0x67, 0x63, 0x65, 0x33,
            0x79, 0x35, 0x36, 0x71, 0x6e, 0x6a, 0x71, 0x7a, 0x6d, 0x68, 0x74, 0x36, 0x68, 0x79,
            0x37, 0x72, 0x33, 0x38, 0x74, 0x6b, 0x77, 0x64, 0x74, 0x67, 0x39, 0x61, 0x34, 0x63,
            0x63, 0x78, 0x65, 0x33, 0x64, 0x78, 0x74, 0x34, 0x6b, 0x74, 0x6a, 0x34, 0x61, 0x6a,
            0x6e, 0x39, 0x6b, 0x79, 0x34, 0x6c, 0x73, 0x36, 0x6a, 0x65, 0x6a, 0x63, 0x39, 0x72,
            0x67, 0x66, 0x64, 0x75, 0x6e, 0x73, 0x75, 0x67, 0x75, 0x68, 0x78, 0x64, 0x6d, 0x70,
            0x6a, 0x30, 0x35, 0x75, 0x72, 0x36, 0x64, 0x75, 0x63, 0x64, 0x77, 0x6c, 0x68, 0x6a,
            0x6d, 0x39, 0x32, 0x6a, 0x78, 0x6c, 0x6a, 0x72, 0x79, 0x37, 0x74, 0x67, 0x32, 0x64,
            0x6b,
        ],
    },
    TestVector {
        p2pkh_bytes: None,
        p2sh_bytes: None,
        sapling_raw_addr: Some([
            0x02, 0xf1, 0x53, 0x6b, 0x62, 0x2c, 0x01, 0x34, 0x67, 0x42, 0xd8, 0xf9, 0x0e, 0x9d,
            0x4f, 0xf3, 0x91, 0x37, 0xf1, 0xbe, 0xbe, 0x6e, 0x23, 0xad, 0x99, 0x71, 0x77, 0x6b,
            0x33, 0x72, 0x70, 0x24, 0x94, 0xcc, 0x08, 0x95, 0x1e, 0xef, 0x03, 0x2b, 0x35, 0x35,
            0x0f,
        ]),
        orchard_raw_addr: None,
        unified_addr: &[
            0x75, 0x31, 0x33, 0x63, 0x36, 0x6d, 0x36, 0x71, 0x6e, 0x65, 0x7a, 0x72, 0x33, 0x79,
            0x66, 0x75, 0x34, 0x68, 0x75, 0x76, 0x30, 0x35, 0x6e, 0x68, 0x79, 0x61, 0x35, 0x63,
            0x72, 0x78, 0x6e, 0x35, 0x34, 0x78, 0x61, 0x78, 0x6a, 0x78, 0x37, 0x6d, 0x6b, 0x66,
            0x74, 0x39, 0x38, 0x61, 0x79, 0x6e, 0x7a, 0x33, 0x6b, 0x68, 0x63, 0x6e, 0x61, 0x76,
            0x64, 0x79, 0x61, 0x30, 0x6c, 0x74, 0x6a, 0x79, 0x75, 0x65, 0x71, 0x7a, 0x35, 0x77,
            0x70, 0x6d, 0x30, 0x6d, 0x7a, 0x6a, 0x35, 0x7a, 0x64, 0x6c, 0x34, 0x34, 0x64, 0x32,
            0x30, 0x76, 0x65, 0x7a, 0x67, 0x68, 0x75, 0x32, 0x72, 0x74, 0x38, 0x61, 0x73, 0x76,
            0x35, 0x63, 0x6c, 0x61, 0x33, 0x74, 0x64, 0x63,
        ],
    },
    TestVector {
        p2pkh_bytes: None,
        p2sh_bytes: Some([
            0x18, 0x3e, 0x31, 0xd4, 0x9f, 0x25, 0xc9, 0xa1, 0x38, 0xf4, 0x9b, 0x1a, 0x53, 0x7e,
            0xdc, 0xf0, 0x4b, 0xe3, 0x4a, 0x98,
        ]),
        sapling_raw_addr: Some([
            0x32, 0x46, 0xb5, 0x9a, 0x5b, 0x49, 0x2d, 0xab, 0x18, 0x55, 0xcc, 0x17, 0x6b, 0xdd,
            0xfa, 0x28, 0x41, 0x8f, 0x11, 0xf9, 0x7f, 0x7b, 0x36, 0x1c, 0xc3, 0xe8, 0x83, 0x4b,
            0x2c, 0x30, 0xd2, 0xa1, 0x71, 0x7d, 0xf3, 0x23, 0xef, 0x98, 0xea, 0x7d, 0xe7, 0x1d,
            0x2e,
        ]),
        orchard_raw_addr: Some([
            0xab, 0x6d, 0x26, 0x25, 0x2c, 0x52, 0x15, 0x47, 0x04, 0x9d, 0xe2, 0x08, 0x28, 0x3d,
            0x96, 0x27, 0x8b, 0xb2, 0x21, 0xa6, 0x87, 0x4c, 0xb5, 0xa8, 0x6a, 0xf1, 0xd3, 0xf8,
            0xb3, 0xdb, 0x3f, 0xbe, 0xe3, 0xdb, 0xef, 0xed, 0xcb, 0x2c, 0x71, 0xe3, 0xca, 0x1e,
            0xad,
        ]),
        unified_addr: &[
            0x75, 0x31, 0x65, 0x6a, 0x70, 0x6e, 0x33, 0x67, 0x6e, 0x34, 0x30, 0x39, 0x73, 0x72,
            0x38, 0x33, 0x34, 0x66, 0x63, 0x77, 0x71, 0x32, 0x6b, 0x68, 0x36, 0x79, 0x34, 0x6a,
            0x61, 0x70, 0x66, 0x39, 0x68, 0x71, 0x72, 0x73, 0x36, 0x36, 0x33, 0x78, 0x6a, 0x30,
            0x74, 0x79, 0x6e, 0x78, 0x75, 0x63, 0x33, 0x64, 0x67, 0x76, 0x6b, 0x78, 0x67, 0x67,
            0x77, 0x75, 0x75, 0x30, 0x6d, 0x64, 0x6c, 0x79, 0x38, 0x38, 0x63, 0x79, 0x63, 0x38,
            0x67, 0x76, 0x74, 0x33, 0x30, 0x6b, 0x76, 0x34, 0x36, 0x35, 0x76, 0x39, 0x76, 0x65,
            0x6d, 0x6b, 0x72, 0x32, 0x32, 0x77, 0x71, 0x65, 0x78, 0x61, 0x73, 0x72, 0x77, 0x34,
            0x39, 0x76, 0x79, 0x6c, 0x34, 0x68, 0x6e, 0x61, 0x6c, 0x6c, 0x6a, 0x64, 0x63, 0x75,
            0x36, 0x32, 0x75, 0x32, 0x73, 0x61, 0x34, 0x64, 0x32, 0x61, 0x35, 0x74, 0x63, 0x65,
            0x68, 0x72, 0x66, 0x6c, 0x75, 0x79, 0x74, 0x6a, 0x7a, 0x32, 0x70, 0x7a, 0x6a, 0x39,
            0x6d, 0x61, 0x39, 0x38, 0x63, 0x78, 0x33, 0x30, 0x63, 0x6b, 0x32, 0x71, 0x30, 0x6d,
            0x6b, 0x35, 0x30, 0x6b, 0x64, 0x36, 0x6d, 0x65, 0x76, 0x70, 0x39, 0x68, 0x70, 0x79,
            0x6d, 0x6d, 0x39, 0x70, 0x37, 0x72, 0x73, 0x75, 0x79, 0x38, 0x70, 0x76, 0x30, 0x70,
            0x61, 0x66, 0x6c, 0x66, 0x6a, 0x68, 0x6c, 0x30, 0x6c, 0x70, 0x7a, 0x6e, 0x32, 0x32,
            0x38, 0x33, 0x77, 0x6b, 0x68, 0x64, 0x6c, 0x6a, 0x68, 0x71, 0x6a, 0x7a, 0x30, 0x6e,
            0x63, 0x75, 0x6b,
        ],
    },
    TestVector {
        p2pkh_bytes: None,
        p2sh_bytes: None,
        sapling_raw_addr: Some([
            0x97, 0x0d, 0xc3, 0x45, 0x0d, 0x34, 0x55, 0x41, 0x41, 0xd3, 0x56, 0xcb, 0x54, 0x80,
            0x56, 0x27, 0x9c, 0x57, 0x70, 0x8f, 0xa7, 0x3b, 0xd1, 0x6f, 0xfe, 0x9a, 0x2e, 0x24,
            0xea, 0x69, 0x48, 0x98, 0xa7, 0xb8, 0xaf, 0x1b, 0x0f, 0xf9, 0x25, 0x85, 0xd0, 0x26,
            0x23,
        ]),
        orchard_raw_addr: Some([
            0x04, 0x14, 0xbb, 0x62, 0xb8, 0x61, 0x49, 0xee, 0x73, 0x18, 0x51, 0xf2, 0x7d, 0x53,
            0x2a, 0xc0, 0x36, 0x11, 0x69, 0xda, 0x46, 0xe6, 0xd5, 0x3d, 0x19, 0xd3, 0xdf, 0xd0,
            0x7a, 0x5b, 0xae, 0x22, 0x96, 0x99, 0x22, 0xd8, 0xd0, 0xaf, 0x7d, 0xc1, 0xe1, 0x3b,
            0xae,
        ]),
        unified_addr: &[
            0x75, 0x31, 0x6a, 0x6d, 0x38, 0x6d, 0x65, 0x63, 0x32, 0x6c, 0x73, 0x72, 0x65, 0x33,
            0x66, 0x66, 0x65, 0x65, 0x70, 0x6d, 0x74, 0x74, 0x73, 0x34, 0x37, 0x6b, 0x38, 0x33,
            0x33, 0x6d, 0x33, 0x72, 0x71, 0x65, 0x30, 0x72, 0x68, 0x6d, 0x7a, 0x6a, 0x39, 0x37,
            0x78, 0x72, 0x67, 0x37, 0x37, 0x61, 0x36, 0x66, 0x6c, 0x6a, 0x7a, 0x61, 0x33, 0x36,
            0x66, 0x6a, 0x68, 0x77, 0x34, 0x64, 0x63, 0x63, 0x76, 0x6d, 0x39, 0x6c, 0x32, 0x6e,
            0x61, 0x37, 0x6c, 0x70, 0x61, 0x66, 0x75, 0x6a, 0x66, 0x61, 0x35, 0x6b, 0x61, 0x74,
            0x77, 0x38, 0x39, 0x79, 0x77, 0x36, 0x36, 0x68, 0x73, 0x30, 0x63, 0x61, 0x35, 0x74,
            0x74, 0x36, 0x66, 0x65, 0x70, 0x73, 0x6a, 0x76, 0x36, 0x70, 0x30, 0x75, 0x75, 0x39,
            0x73, 0x77, 0x64, 0x61, 0x76, 0x72, 0x63, 0x38, 0x70, 0x78, 0x6d, 0x6c, 0x34, 0x30,
            0x66, 0x77, 0x38, 0x65, 0x76, 0x6b, 0x76, 0x32, 0x30, 0x76, 0x6a, 0x61, 0x38, 0x6e,
            0x77, 0x78, 0x6e, 0x37, 0x36, 0x6e, 0x61, 0x30, 0x6d, 0x37, 0x6e, 0x67, 0x74, 0x32,
            0x6c, 0x30, 0x79, 0x73, 0x36, 0x32, 0x35, 0x37, 0x30, 0x77, 0x61, 0x75, 0x6a, 0x71,
            0x73, 0x74, 0x35, 0x71, 0x37, 0x79, 0x74, 0x35, 0x74, 0x6e,
        ],
    },
    TestVector {
        p2pkh_bytes: None,
        p2sh_bytes: Some([
            0x09, 0x8b, 0x79, 0x53, 0x5e, 0x79, 0x0f, 0xe5, 0x3e, 0x29, 0xfe, 0xf2, 0xb3, 0x76,
            0x66, 0x97, 0xac, 0x32, 0xb4, 0xf4,
        ]),
        sapling_raw_addr: Some([
            0xa8, 0xa8, 0x79, 0x7c, 0x1b, 0xa6, 0x9f, 0x78, 0x67, 0x2a, 0xff, 0xa6, 0x5b, 0x94,
            0x39, 0x75, 0x02, 0x69, 0x31, 0xea, 0x62, 0x84, 0x31, 0xf0, 0x99, 0x1e, 0x74, 0x48,
            0x72, 0xac, 0x9f, 0x36, 0x94, 0x6f, 0x5d, 0xcd, 0x68, 0x51, 0xa0, 0xb5, 0xaf, 0x29,
            0xcf,
        ]),
        orchard_raw_addr: Some([
            0x67, 0x8a, 0xb0, 0x07, 0x9b, 0xea, 0x28, 0xbf, 0x16, 0x5c, 0x1a, 0xb9, 0x76, 0xa2,
            0xa5, 0x8c, 0x18, 0xa7, 0x81, 0x1c, 0xa2, 0xad, 0x0a, 0xd6, 0x49, 0xe8, 0x76, 0x27,
            0x3d, 0x04, 0x32, 0x5d, 0xa6, 0xca, 0x53, 0xcd, 0xb8, 0x3c, 0x11, 0x1e, 0x8e, 0x43,
            0x94,
        ]),
        unified_addr: &[
            0x75, 0x31, 0x61, 0x76, 0x73, 0x63, 0x33, 0x74, 0x61, 0x38, 0x38, 0x64, 0x68, 0x63,
            0x34, 0x6a, 0x35, 0x37, 0x74, 0x64, 0x65, 0x70, 0x38, 0x6a, 0x68, 0x33, 0x66, 0x32,
            0x73, 0x67, 0x33, 0x63, 0x75, 0x6e, 0x66, 0x70, 0x73, 0x6d, 0x36, 0x76, 0x6d, 0x63,
            0x6a, 0x61, 0x61, 0x37, 0x35, 0x66, 0x30, 0x66, 0x64, 0x39, 0x37, 0x66, 0x71, 0x37,
            0x63, 0x70, 0x30, 0x79, 0x71, 0x34, 0x63, 0x6b, 0x6d, 0x63, 0x6c, 0x35, 0x76, 0x63,
            0x77, 0x78, 0x78, 0x77, 0x77, 0x33, 0x32, 0x73, 0x75, 0x73, 0x75, 0x74, 0x30, 0x76,
            0x34, 0x6c, 0x30, 0x39, 0x37, 0x33, 0x76, 0x35, 0x73, 0x76, 0x6e, 0x37, 0x37, 0x75,
            0x74, 0x30, 0x30, 0x7a, 0x61, 0x75, 0x38, 0x36, 0x6e, 0x39, 0x36, 0x79, 0x67, 0x63,
            0x77, 0x76, 0x79, 0x7a, 0x32, 0x79, 0x35, 0x74, 0x79, 0x79, 0x6c, 0x36, 0x6e, 0x64,
            0x79, 0x72, 0x36, 0x38, 0x38, 0x32, 0x36, 0x34, 0x6e, 0x72, 0x63, 0x34, 0x32, 0x73,
            0x68, 0x38, 0x33, 0x32, 0x6c, 0x6c, 0x68, 0x61, 0x70, 0x68, 0x6a, 0x39, 0x33, 0x61,
            0x6c, 0x30, 0x33, 0x6a, 0x6e, 0x64, 0x36, 0x36, 0x70, 0x37, 0x6e, 0x34, 0x34, 0x70,
            0x70, 0x37, 0x68, 0x71, 0x38, 0x66, 0x6e, 0x6b, 0x75, 0x79, 0x6d, 0x6b, 0x79, 0x6a,
            0x35, 0x36, 0x65, 0x35, 0x70, 0x39, 0x67, 0x72, 0x39, 0x78, 0x65, 0x65, 0x78, 0x34,
            0x30, 0x38, 0x65, 0x35, 0x32, 0x6e, 0x37, 0x35, 0x35, 0x65, 0x37, 0x38, 0x63, 0x64,
            0x6a, 0x6c, 0x73,
        ],
    },
    TestVector {
        p2pkh_bytes: None,
        p2sh_bytes: None,
        sapling_raw_addr: Some([
            0x35, 0x09, 0xc9, 0xe0, 0x69, 0xe8, 0x9f, 0xe5, 0x01, 0xd9, 0x76, 0x22, 0xc2, 0x83,
            0xac, 0x98, 0x92, 0x3d, 0xa2, 0xd7, 0xe6, 0xeb, 0x34, 0x6b, 0x4b, 0xaf, 0xa6, 0x78,
            0x65, 0xe1, 0xe6, 0xda, 0xe7, 0xcf, 0x21, 0x3b, 0x1e, 0xa3, 0x64, 0x8d, 0xc0, 0x9b,
            0x48,
        ]),
        orchard_raw_addr: None,
        unified_addr: &[
            0x75, 0x31, 0x35, 0x76, 0x76, 0x38, 0x38, 0x34, 0x63, 0x7a, 0x35, 0x64, 0x36, 0x34,
            0x6e, 0x67, 0x72, 0x32, 0x71, 0x76, 0x34, 0x30, 0x78, 0x35, 0x79, 0x71, 0x71, 0x67,
            0x7a, 0x36, 0x6a, 0x74, 0x33, 0x68, 0x65, 0x7a, 0x75, 0x39, 0x6d, 0x6b, 0x75, 0x66,
            0x32, 0x30, 0x64, 0x75, 0x6a, 0x61, 0x66, 0x38, 0x71, 0x76, 0x6b, 0x73, 0x6c, 0x6e,
            0x78, 0x79, 0x37, 0x38, 0x66, 0x32, 0x64, 0x70, 0x78, 0x79, 0x32, 0x34, 0x70, 0x76,
            0x76, 0x37, 0x79, 0x76, 0x66, 0x63, 0x7a, 0x6b, 0x6c, 0x30, 0x77, 0x61, 0x65, 0x34,
            0x35, 0x61, 0x30, 0x70, 0x68, 0x36, 0x64, 0x37, 0x7a, 0x37, 0x64, 0x65, 0x74, 0x33,
            0x6a, 0x67, 0x34, 0x72, 0x67, 0x67, 0x78, 0x66,
        ],
    },
    TestVector {
        p2pkh_bytes: None,
        p2sh_bytes: Some([
            0x30, 0xd0, 0x69, 0x89, 0x6c, 0xff, 0x30, 0xeb, 0x41, 0x4f, 0x72, 0x7b, 0x89, 0xe0,
            0x01, 0xaf, 0xa2, 0xfb, 0x8d, 0xc3,
        ]),
        sapling_raw_addr: Some([
            0x55, 0xbc, 0x46, 0xae, 0xa6, 0xf6, 0x0c, 0x1d, 0x61, 0x91, 0x56, 0x40, 0x02, 0x9b,
            0x2a, 0xf6, 0x33, 0x4d, 0x7d, 0x27, 0xe1, 0xc4, 0x7a, 0x24, 0x8a, 0xb4, 0x7c, 0x9f,
            0xbe, 0x5d, 0x2d, 0x7b, 0xb5, 0x81, 0x87, 0x39, 0xf0, 0x62, 0xe3, 0x71, 0x36, 0x65,
            0x4c,
        ]),
        orchard_raw_addr: None,
        unified_addr: &[
            0x75, 0x31, 0x39, 0x67, 0x67, 0x38, 0x73, 0x71, 0x70, 0x65, 0x68, 0x75, 0x6d, 0x67,
            0x6d, 0x73, 0x78, 0x7a, 0x67, 0x6a, 0x79, 0x6d, 0x6c, 0x39, 0x33, 0x36, 0x78, 0x6b,
            0x32, 0x67, 0x78, 0x6d, 0x73, 0x66, 0x65, 0x35, 0x6a, 0x65, 0x37, 0x37, 0x7a, 0x6a,
            0x61, 0x61, 0x30, 0x67, 0x67, 0x6e, 0x32, 0x72, 0x33, 0x30, 0x73, 0x32, 0x39, 0x34,
            0x32, 0x66, 0x76, 0x6b, 0x61, 0x32, 0x75, 0x63, 0x74, 0x75, 0x36, 0x39, 0x6d, 0x70,
            0x74, 0x76, 0x30, 0x63, 0x32, 0x39, 0x76, 0x68, 0x6a, 0x70, 0x34, 0x68, 0x61, 0x72,
            0x63, 0x75, 0x30, 0x72, 0x32, 0x73, 0x36, 0x6e, 0x79, 0x37, 0x30, 0x30, 0x6c, 0x79,
            0x7a, 0x78, 0x71, 0x68, 0x66, 0x38, 0x33, 0x35, 0x78, 0x6b, 0x71, 0x6a, 0x78, 0x73,
            0x77, 0x6a, 0x6a, 0x77, 0x71, 0x30, 0x32, 0x61, 0x64, 0x6b, 0x71, 0x79, 0x6a, 0x6b,
            0x6b, 0x39, 0x63, 0x77, 0x6a, 0x6e, 0x37, 0x70, 0x32, 0x73, 0x68, 0x64, 0x71, 0x33,
            0x79,
        ],
    },
    TestVector {
        p2pkh_bytes: None,
        p2sh_bytes: None,
        sapling_raw_addr: Some([
            0x5c, 0x26, 0xa8, 0x11, 0x77, 0x29, 0x33, 0x4a, 0x95, 0x7c, 0xa7, 0x94, 0x1d, 0x47,
            0xb2, 0xce, 0x70, 0x40, 0xe8, 0x44, 0xfa, 0x98, 0x82, 0xc2, 0x5b, 0xfd, 0x2f, 0xcf,
            0x51, 0xfa, 0x8a, 0xb2, 0x13, 0x76, 0xf5, 0x30, 0x0d, 0x01, 0x23, 0xf5, 0x70, 0x3e,
            0x9e,
        ]),
        orchard_raw_addr: None,
        unified_addr: &[
            0x75, 0x31, 0x39, 0x76, 0x63, 0x6e, 0x33, 0x72, 0x65, 0x64, 0x70, 0x61, 0x70, 0x68,
            0x78, 0x34, 0x32, 0x6d, 0x6e, 0x30, 0x73, 0x79, 0x63, 0x32, 0x36, 0x79, 0x38, 0x77,
            0x39, 0x77, 0x66, 0x67, 0x6c, 0x65, 0x7a, 0x39, 0x61, 0x75, 0x73, 0x6b, 0x61, 0x78,
            0x72, 0x67, 0x68, 0x6d, 0x78, 0x38, 0x30, 0x64, 0x75, 0x6e, 0x61, 0x33, 0x36, 0x63,
            0x61, 0x67, 0x63, 0x33, 0x79, 0x73, 0x37, 0x6d, 0x6e, 0x33, 0x6a, 0x37, 0x36, 0x39,
            0x76, 0x63, 0x67, 0x38, 0x72, 0x75, 0x33, 0x6b, 0x64, 0x6e, 0x61, 0x71, 0x34, 0x70,
            0x68, 0x34, 0x36, 0x30, 0x34, 0x38, 0x64, 0x68, 0x73, 0x76, 0x6c, 0x35, 0x64, 0x6d,
            0x64, 0x73, 0x67, 0x78, 0x79, 0x65, 0x38, 0x33,
        ],
    },
];
