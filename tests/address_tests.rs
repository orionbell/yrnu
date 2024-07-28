use yrnu::address::{self, IpAddress, IpKind, Mask, Network};

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
#[test]
fn get_kind_test() {

}
// IpAddress tests
#[test]
fn new_ipaddress_test() {
    let addr = IpAddress::new("192.168.1.1").unwrap();
    assert_eq!(addr.to_string(),"192.168.1.1 is a version 4 private ip address");
    let addr2 = IpAddress::new("fc00::1").unwrap();
    assert_eq!(addr2.to_string(),"fc00::1 is a version 6 uniqe local ip address");
}

// Mask tests
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

// Network tests
#[test]
fn new_network_test() {
    let id = IpAddress::new("192.168.1.0").unwrap();
    let mask = Mask::from_prefix(28).unwrap();
    let net1 = Network::new(id, mask).unwrap();
    //let mask = Mask::from_prefix(27).unwrap();
    let net2 = Network::from_str("192.168.1.32/27").expect("failed");
    let big_net = Network::from_str("10.0.16.0/20").expect("failed");
    let super_big_net = Network::from_str("10.16.0.0/12").expect("failed");
    assert_eq!("192.168.1.0/28",net1.to_string());
    assert_eq!("192.168.1.32/27",net2.to_string());
    assert_eq!("192.168.1.63",net2.broadcast().address());
    assert_eq!(big_net.to_string(), "10.0.16.0/20");
    assert_eq!("10.0.31.255",big_net.broadcast().address());
    assert_eq!(super_big_net.to_string(), "10.16.0.0/12");
    assert_eq!("10.31.255.255",super_big_net.broadcast().address())
}

//#[test]
//fn containes_test() {

//}









