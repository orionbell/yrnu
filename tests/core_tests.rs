use yrnu::core::{self, IpAddress, IpKind, IpVersion, MacAddress, Mask, Network, Interface};

// IpVersion tests
#[test]
fn is_v4_test() {

    assert_eq!(IpVersion::is_v4("255.255.255.128"),true);
    assert_eq!(IpVersion::is_v4("192.168.1"),false);
    assert_eq!(IpVersion::is_v4("192.168.10.1.1"),false);
    assert_eq!(IpVersion::is_v4("256.168.10.1"),false);
    assert_eq!(IpVersion::is_v4("FFEE:0::123"),false);
}

#[test]
fn is_v6_test() {

    assert_eq!(IpVersion::is_v6("192.168.10.1"),false);
    assert_eq!(IpVersion::is_v6("ffee:0000:1234:4321:feed:dead:c0ff:eeee"),true);
    assert_eq!(IpVersion::is_v6("ffee:0000:1234:4321:feed:dead:c0ff:rrrr"),false);
    assert_eq!(IpVersion::is_v6("f::1"),true);
    assert_eq!(IpVersion::is_v6("123::123:2"),true);
    assert_eq!(IpVersion::is_v6("123:123:123:123:123"),false);
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
#[test]
fn expend_shorten_test() {
    assert_eq!(IpAddress::expend("f01:0:1:20:300::10").unwrap(),"0f01:0000:0001:0020:0300:0000:0000:0010".to_string());
    assert_eq!(IpAddress::shorten("0f01:0000:0001:0020:0300:0000:0000:0010").unwrap(),"f01:0:1:20:300::10".to_string());
    assert_eq!(IpAddress::expend("0f01:0:0001:020:0300::0010").unwrap(),"0f01:0000:0001:0020:0300:0000:0000:0010".to_string());
    assert_eq!(IpAddress::shorten("0f01:00:0001:0020:0300::0010").unwrap(),"f01:0:1:20:300::10".to_string());
}
//#[test]
//fn ipaddress_cmp_test() {
    
//}

// Mask tests
#[test]
fn is_valid_test() {
    assert_eq!(Mask::is_valid("255.255.255.0"),true);
    assert_eq!(Mask::is_valid("230.0.0.0"),false);
    assert_eq!(Mask::is_valid("255.0.0.255"),false);
    assert_eq!(Mask::is_valid("255.255.0.1"),false);
    assert_eq!(Mask::is_valid("224.0.0.0"),true);
    assert_eq!(Mask::is_valid("255.255.255.128"),true);
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
    let id2 = IpAddress::new("192.168.1.16").unwrap();
    let mask2 = Mask::new("255.255.255.240").unwrap();
    let net = Network::new(id2, mask2).unwrap();
    let net1 = Network::new(id, mask).unwrap();
    //let mask = Mask::from_prefix(27).unwrap();
    let net2 = Network::from_str("192.168.1.32/27").expect("failed");
    let big_net = Network::from_str("10.0.16.0/20").expect("failed");
    let super_big_net = Network::from_str("10.16.0.0/12").expect("failed");
    assert_eq!("192.168.1.16/28",net.to_string());
    assert_eq!("192.168.1.0/28",net1.to_string());
    assert_eq!("192.168.1.32/27",net2.to_string());
    assert_eq!("192.168.1.63",net2.broadcast().address());
    assert_eq!(big_net.to_string(), "10.0.16.0/20");
    assert_eq!("10.0.31.255",big_net.broadcast().address());
    assert_eq!(super_big_net.to_string(), "10.16.0.0/12");
    assert_eq!("10.31.255.255",super_big_net.broadcast().address())
}

#[test]
fn containes_test() {
    let net1 = Network::from_str("192.168.15.128/28").unwrap();
    let net2 = Network::from_str("10.1.12.0/24").unwrap();
    let addr = IpAddress::new("10.1.12.2").unwrap();
    assert_eq!(net1.containes(&addr), false);
    assert_eq!(net2.containes(&addr), true);
}

#[test]
fn new_mac_test() {
    let mac1 = MacAddress::new("AC:12:00:1f:ff:22");
    let mac2 = MacAddress::new("AC:R2:00:1f:ff:22");
    let mac3 = MacAddress::new("AC:12:00:1f:ff");
    let mac4 = MacAddress::new("AC12:00:1f:ff:22");
    assert_eq!(mac1.is_err(),false);
    assert_eq!(mac2.is_err(),true);
    assert_eq!(mac3.is_err(),true);
    assert_eq!(mac4.is_err(),true);
}

#[test]
fn mac_cmp_test() {
    let mac1 = MacAddress::new("AB:CD:EF:12:34:56").unwrap();
    let mac2 = MacAddress::new("AB:CD:EF:12:34:56").unwrap(); 
    let mac3 = MacAddress::new("AB:CD:EF:22:34:56").unwrap();
    assert_eq!(mac1 == mac2, true);
    assert_eq!(mac1 == mac3, false);
    assert_eq!(mac1 > mac3, false);
    assert_eq!(mac1 >= mac1, true);
    assert_eq!(mac2 <= mac3, true);
}

#[test]
fn eui64_test() {
    let mac1 = MacAddress::new("aa:bb:cc:ee:ff:11").unwrap();
    let mac2 = MacAddress::new("11:22:33:44:55:66").unwrap();
    let mac3 = MacAddress::new("12:34:56:78:9a:cd").unwrap();

    assert_eq!(IpAddress::eui64(&mac1).address(),"fe80::a8bb:ccff:feee:ff11");
    assert_eq!(IpAddress::eui64(&mac2).address(),"fe80::1322:33ff:fe44:5566");
    assert_eq!(IpAddress::eui64(&mac3).address(),"fe80::1034:56ff:fe78:9acd");
}

#[test]
fn new_if_test() {
    let inf = Interface::by_name("wlan0");
    let inf2 = Interface::by_index(2);
 //   println!("{}",inf.unwrap());
 //   println!("{}", inf2.unwrap());

    let infs = Interface::all();
    for inf in infs {
//        println!("\n\n{}",inf);
    }
}


