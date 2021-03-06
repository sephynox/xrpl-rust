pub struct TestCase<'a> {
    pub address: &'a str,
    pub tag: Option<u64>,
    pub main_xaddress: &'a str,
    pub test_xaddress: &'a str,
}

pub const ADDRESS_TEST_CASES: [TestCase; 22] = [
    TestCase {
        address: "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
        tag: None,
        main_xaddress: "X7AcgcsBL6XDcUb289X4mJ8djcdyKaB5hJDWMArnXr61cqZ",
        test_xaddress: "T719a5UwUCnEs54UsxG9CJYYDhwmFCqkr7wxCcNcfZ6p5GZ",
    },
    TestCase {
        address: "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
        tag: Some(1),
        main_xaddress: "X7AcgcsBL6XDcUb289X4mJ8djcdyKaGZMhc9YTE92ehJ2Fu",
        test_xaddress: "T719a5UwUCnEs54UsxG9CJYYDhwmFCvbJNZbi37gBGkRkbE",
    },
    TestCase {
        address: "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
        tag: Some(14),
        main_xaddress: "X7AcgcsBL6XDcUb289X4mJ8djcdyKaGo2K5VpXpmCqbV2gS",
        test_xaddress: "T719a5UwUCnEs54UsxG9CJYYDhwmFCvqXVCALUGJGSbNV3x",
    },
    TestCase {
        address: "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
        tag: Some(11747),
        main_xaddress: "X7AcgcsBL6XDcUb289X4mJ8djcdyKaLFuhLRuNXPrDeJd9A",
        test_xaddress: "T719a5UwUCnEs54UsxG9CJYYDhwmFCziiNHtUukubF2Mg6t",
    },
    TestCase {
        address: "rLczgQHxPhWtjkaQqn3Q6UM8AbRbbRvs5K",
        tag: None,
        main_xaddress: "XVZVpQj8YSVpNyiwXYSqvQoQqgBttTxAZwMcuJd4xteQHyt",
        test_xaddress: "TVVrSWtmQQssgVcmoMBcFQZKKf56QscyWLKnUyiuZW8ALU4",
    },
    TestCase {
        address: "rpZc4mVfWUif9CRoHRKKcmhu1nx2xktxBo",
        tag: None,
        main_xaddress: "X7YenJqxv3L66CwhBSfd3N8RzGXxYqPopMGMsCcpho79rex",
        test_xaddress: "T77wVQzA8ntj9wvCTNiQpNYLT5hmhRsFyXDoMLqYC4BzQtV",
    },
    TestCase {
        address: "rpZc4mVfWUif9CRoHRKKcmhu1nx2xktxBo",
        tag: Some(58),
        main_xaddress: "X7YenJqxv3L66CwhBSfd3N8RzGXxYqV56ZkTCa9UCzgaao1",
        test_xaddress: "T77wVQzA8ntj9wvCTNiQpNYLT5hmhR9kej6uxm4jGcQD7rZ",
    },
    TestCase {
        address: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
        tag: Some(23480),
        main_xaddress: "X7d3eHCXzwBeWrZec1yT24iZerQjYL8m8zCJ16ACxu1BrBY",
        test_xaddress: "T7YChPFWifjCAXLEtg5N74c7fSAYsvSokwcmBPBUZWhxH5P",
    },
    TestCase {
        address: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
        tag: Some(11747),
        main_xaddress: "X7d3eHCXzwBeWrZec1yT24iZerQjYLo2CJf8oVC5CMWey5m",
        test_xaddress: "T7YChPFWifjCAXLEtg5N74c7fSAYsvTcc7nEfwuEEvn5Q4w",
    },
    TestCase {
        address: "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf",
        tag: None,
        main_xaddress: "XVLhHMPHU98es4dbozjVtdWzVrDjtV5fdx1mHp98tDMoQXb",
        test_xaddress: "TVE26TYGhfLC7tQDno7G8dGtxSkYQn49b3qD26PK7FcGSKE",
    },
    TestCase {
        address: "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf",
        tag: Some(0),
        main_xaddress: "XVLhHMPHU98es4dbozjVtdWzVrDjtV8AqEL4xcZj5whKbmc",
        test_xaddress: "TVE26TYGhfLC7tQDno7G8dGtxSkYQnSy8RHqGHoGJ59spi2",
    },
    TestCase {
        address: "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf",
        tag: Some(1),
        main_xaddress: "XVLhHMPHU98es4dbozjVtdWzVrDjtV8xvjGQTYPiAx6gwDC",
        test_xaddress: "TVE26TYGhfLC7tQDno7G8dGtxSkYQnSz1uDimDdPYXzSpyw",
    },
    TestCase {
        address: "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf",
        tag: Some(2),
        main_xaddress: "XVLhHMPHU98es4dbozjVtdWzVrDjtV8zpDURx7DzBCkrQE7",
        test_xaddress: "TVE26TYGhfLC7tQDno7G8dGtxSkYQnTryP9tG9TW8GeMBmd",
    },
    TestCase {
        address: "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf",
        tag: Some(32),
        main_xaddress: "XVLhHMPHU98es4dbozjVtdWzVrDjtVoYiC9UvKfjKar4LJe",
        test_xaddress: "TVE26TYGhfLC7tQDno7G8dGtxSkYQnT2oqaCDzMEuCDAj1j",
    },
    TestCase {
        address: "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf",
        tag: Some(276),
        main_xaddress: "XVLhHMPHU98es4dbozjVtdWzVrDjtVoKj3MnFGMXEFMnvJV",
        test_xaddress: "TVE26TYGhfLC7tQDno7G8dGtxSkYQnTMgJJYfAbsiPsc6Zg",
    },
    TestCase {
        address: "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf",
        tag: Some(65591),
        main_xaddress: "XVLhHMPHU98es4dbozjVtdWzVrDjtVozpjdhPQVdt3ghaWw",
        test_xaddress: "TVE26TYGhfLC7tQDno7G8dGtxSkYQn7ryu2W6njw7mT1jmS",
    },
    TestCase {
        address: "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf",
        tag: Some(16781933),
        main_xaddress: "XVLhHMPHU98es4dbozjVtdWzVrDjtVqrDUk2vDpkTjPsY73",
        test_xaddress: "TVE26TYGhfLC7tQDno7G8dGtxSkYQnVsw45sDtGHhLi27Qa",
    },
    TestCase {
        address: "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf",
        tag: Some(4294967294),
        main_xaddress: "XVLhHMPHU98es4dbozjVtdWzVrDjtV1kAsixQTdMjbWi39u",
        test_xaddress: "TVE26TYGhfLC7tQDno7G8dGtxSkYQnX8tDFQ53itLNqs6vU",
    },
    TestCase {
        address: "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf",
        tag: Some(4294967295),
        main_xaddress: "XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHi",
        test_xaddress: "TVE26TYGhfLC7tQDno7G8dGtxSkYQnXoy6kSDh6rZzApc69",
    },
    TestCase {
        address: "rPEPPER7kfTD9w2To4CQk6UCfuHM9c6GDY",
        tag: None,
        main_xaddress: "XV5sbjUmgPpvXv4ixFWZ5ptAYZ6PD2gYsjNFQLKYW33DzBm",
        test_xaddress: "TVd2rqMkYL2AyS97NdELcpeiprNBjwLZzuUG5rZnaewsahi",
    },
    TestCase {
        address: "rPEPPER7kfTD9w2To4CQk6UCfuHM9c6GDY",
        tag: Some(0),
        main_xaddress: "XV5sbjUmgPpvXv4ixFWZ5ptAYZ6PD2m4Er6SnvjVLpMWPjR",
        test_xaddress: "TVd2rqMkYL2AyS97NdELcpeiprNBjwRQUBetPbyrvXSTuxU",
    },
    TestCase {
        address: "rPEPPER7kfTD9w2To4CQk6UCfuHM9c6GDY",
        tag: Some(13371337),
        main_xaddress: "XV5sbjUmgPpvXv4ixFWZ5ptAYZ6PD2qwGkhgc48zzcx6Gkr",
        test_xaddress: "TVd2rqMkYL2AyS97NdELcpeiprNBjwVUDvp3vhpXbNhLwJi",
    },
];

pub const SECP256K1_ENCODED_SEED_TEST: &str = "sn259rEFXrQrWyx3Q7XneWcwV6dfL";
pub const SECP256K1_HEX_TEST: &str = "CF2DE378FBDD7E2EE87D486DFB5A7BFF";
pub const ED25519_ENCODED_SEED_TEST: &str = "sEdTM1uX8pu2do5XvTnutH6HsouMaM2";
pub const ED25519_HEX_TEST: &str = "4C3A1D213FBDFB14C7C28D609469B341";

pub const NODE_PUBLIC_KEY_TEST: &str = "n9MXXueo837zYH36DvMc13BwHcqtfAWNJY5czWVbp7uYTj7x17TH";
pub const NODE_PUBLIC_KEY_HEX_TEST: &str =
    "0388E5BA87A000CB807240DF8C848EB0B5FFA5C8E5A521BC8E105C0F0A44217828";
pub const ACCOUNT_PUBLIC_KEY_TEST: &str = "aB44YfzW24VDEJQ2UuLPV2PvqcPCSoLnL7y5M1EzhdW4LnK5xMS3";
pub const ACCOUNT_PUBLIC_KEY_HEX_TEST: &str =
    "023693F15967AE357D0327974AD46FE3C127113B1110D6044FD41E723689F81CC6";
