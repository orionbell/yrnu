use yrnu::config::netdev::router::{self, Interface, Router, RouterConfig};
use yrnu::config::netdev::*;
use yrnu::config::{self, netdev::*, Config};
use yrnu::core::{IpAddress, MacAddress, Mask};

#[allow(unused)]
#[test]
fn router_test() {
    let mut rtr = Router::default()
        .hostname(Some("RTR".to_string()))
        .unwrap()
        .secret(Some("12345".to_string()))
        .unwrap()
        .password(Some("54321".to_string()))
        .unwrap()
        .enable_ipv6(Some(true))
        .unwrap()
        .banner(
            Some("^Welcome Admin
Phone: 1244121234
mail: pop@mail.com^"
                .to_string())
        )
        .unwrap()
        .add_interface(
            Interface::gigabit_ethernet([0, 0, 0].into())
                .turn_on()
                .ipv4(
                    Some(IpAddress::new("192.168.1.254").unwrap()),
                    Some(Mask::from_prefix(27).unwrap()),
                )
                .unwrap()
                .ipv6(Some(IpAddress::new("00c2::12:1").unwrap()), Some(64))
                .unwrap(),
        )
        .unwrap()
        .add_line(&Line::console(0))
        .unwrap()
        .add_line(&Line::vty(0, 15).ssh().unwrap())
        .unwrap()
        .to_owned();
    println!("{}", rtr.config());
}
