use yrnu::address::{self, IpAddress, Mask};

// IpVersion tests

#[test]
fn is_v4_test() {

    assert_eq!(address::IpVersion::is_v4("255.255.255.128"),true);
    assert_eq!(address::IpVersion::is_v4("192.168.1"),false);
    assert_eq!(address::IpVersion::is_v4("192.168.10.1.1"),false);
    assert_eq!(address::IpVersion::is_v4("256.168.10.1"),false);
    assert_eq!(address::IpVersion::is_v4("FFEE:0::123"),false);
}

#[test]
fn is_v6_test() {

    assert_eq!(address::IpVersion::is_v6("192.168.10.1"),false);
    assert_eq!(address::IpVersion::is_v6("ffee:0000:1234:4321:feed:dead:c0ff:eeee"),true);
    assert_eq!(address::IpVersion::is_v6("ffee:0000:1234:4321:feed:dead:c0ff:rrrr"),false);
    assert_eq!(address::IpVersion::is_v6("f::1"),true);
    assert_eq!(address::IpVersion::is_v6("123::123:2"),true);
    assert_eq!(address::IpVersion::is_v6("123:123:123:123:123"),false);
}

// IpKind tests

// IpAddress tests
#[test]
fn new_ipaddress_test() {
    let addr = IpAddress::new("192.168.1.1").unwrap();
    assert_eq!(addr.to_string(),"192.168.1.1 is a version 4 private ip address");
}

// mask tests
#[test]
fn is_valid_test() {
    assert_eq!(address::Mask::is_valid("255.255.255.0"),true);
    assert_eq!(address::Mask::is_valid("230.0.0.0"),false);
    assert_eq!(address::Mask::is_valid("255.0.0.255"),false);
    assert_eq!(address::Mask::is_valid("255.255.0.1"),false);
    assert_eq!(address::Mask::is_valid("224.0.0.0"),true);
    assert_eq!(address::Mask::is_valid("255.255.255.128"),true);
}

#[test]
fn new_mask_test() {
    let mask = Mask::new("255.255.255.224").unwrap();
    assert_eq!(mask.to_string(), "255.255.255.224");
    assert_eq!(Mask::from_prefix(24).unwrap().to_string(), "255.255.255.0")
}














