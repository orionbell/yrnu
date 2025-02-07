use super::{
    FlowControlType, InterfaceKind, InterfaceState, Line, LineLoginType, LineType, ParityType,
    Service, TransportType,
};
use crate::config::Config;
use crate::core::{IpAddress, IpVersion, MacAddress, Mask, Network};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
#[derive(Debug, Clone)]
pub enum EncapsulationType {
    Dot1Q,
}
impl Display for EncapsulationType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EncapsulationType::Dot1Q => "dot1q",
            }
        )
    }
}
impl FromStr for EncapsulationType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dot1q" => Ok(EncapsulationType::Dot1Q),
            _ => Err("Invalid encapsulation".to_string()),
        }
    }
}
/// # Interface
/// `Interface` - Router Interface struct
#[derive(Debug, Clone)]
pub struct Interface {
    ipv4: Option<(IpAddress, Mask)>,
    ipv6: Option<(IpAddress, u8)>,
    mac: Option<MacAddress>,
    kind: InterfaceKind,
    state: Option<InterfaceState>,
    speed: Option<u16>,
    delay: Option<u32>,
    description: Option<String>,
    bandwidth: Option<u32>,
    mtu: Option<u16>,
    encapsulation: Option<(EncapsulationType, u16, bool)>,
}
impl Interface {
    pub fn new(kind: InterfaceKind) -> Self {
        Self {
            ipv4: None,
            ipv6: None,
            mac: None,
            kind,
            state: None,
            speed: None,
            delay: None,
            bandwidth: None,
            mtu: None,
            encapsulation: None,
            description: None,
        }
    }
    pub fn gigabit_ethernet(indexes: Vec<u8>) -> Self {
        Interface::new(InterfaceKind::GigabitEthernet(indexes, None))
    }
    pub fn fast_ethernet(indexes: Vec<u8>) -> Self {
        Interface::new(InterfaceKind::FastEthernet(indexes, None))
    }
    pub fn sub_gigabit_ethernet(indexes: Vec<u8>, ind: u32) -> Self {
        Interface::new(InterfaceKind::GigabitEthernet(indexes, Some(ind)))
    }
    pub fn sub_fast_ethernet(indexes: Vec<u8>, ind: u32) -> Self {
        Interface::new(InterfaceKind::FastEthernet(indexes, Some(ind)))
    }
    pub fn loopback(index: u8) -> Self {
        Interface::new(InterfaceKind::Lookback(index))
    }
    pub fn bandwidth(&mut self, bandwidth: Option<u32>) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(bandwidth) = bandwidth {
            if bandwidth > 0 && bandwidth <= 10_000_000 {
                self.bandwidth = Some(bandwidth);
                Ok(self)
            } else {
                todo!() //Err(Box::new())
            }
        } else {
            self.bandwidth = None;
            Ok(self)
        }
    }
    pub fn delay(&mut self, delay: Option<u32>) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(delay) = delay {
            if delay > 0 && delay <= 16_777_215 {
                self.delay = Some(delay);
                Ok(self)
            } else {
                todo!() //Err(Box::new())
            }
        } else {
            self.delay = None;
            Ok(self)
        }
    }
    pub fn mtu(&mut self, mtu: Option<u16>) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(mtu) = mtu {
            if mtu >= 64 && mtu <= 1600 {
                self.mtu = Some(mtu);
                Ok(self)
            } else {
                todo!() //Err(Box::new())
            }
        } else {
            self.mtu = None;
            Ok(self)
        }
    }
    pub fn speed(&mut self, speed: Option<u16>) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(speed) = speed {
            if speed == 0 || speed == 10 || speed == 100 || speed == 1000 {
                self.speed = Some(speed);
                Ok(self)
            } else {
                todo!() //Err(Box::new())
            }
        } else {
            self.speed = None;
            Ok(self)
        }
    }
    pub fn mac(&mut self, mac: Option<MacAddress>) -> &mut Self {
        self.mac = mac;
        self
    }
    pub fn description(&mut self, desc: Option<String>) -> &mut Self {
        self.description = desc;
        self
    }
    pub fn turn_on(&mut self) -> &mut Self {
        Self::state(self, Some(InterfaceState::Up))
    }
    pub fn shutdown(&mut self) -> &mut Self {
        Self::state(self, Some(InterfaceState::Down))
    }
    pub fn state(&mut self, state: Option<InterfaceState>) -> &mut Self {
        self.state = state;
        self
    }
    pub fn ipv4(
        &mut self,
        address: Option<IpAddress>,
        mask: Option<Mask>,
    ) -> Result<&mut Self, Box<dyn Error>> {
        if let (Some(address), Some(mask)) = (address, mask) {
            if *address.version() == IpVersion::V4 {
                self.ipv4 = Some((address, mask));
                Ok(self)
            } else {
                todo!()
            }
        } else {
            self.ipv4 = None;
            Ok(self)
        }
    }
    pub fn ipv6(
        &mut self,
        address: Option<IpAddress>,
        prefix: Option<u8>,
    ) -> Result<&mut Self, Box<dyn Error>> {
        if let (Some(address), Some(prefix)) = (address, prefix) {
            if *address.version() == IpVersion::V6 {
                self.ipv6 = Some((address, prefix));
                Ok(self)
            } else {
                todo!()
            }
        } else {
            self.ipv6 = None;
            Ok(self)
        }
    }
    pub fn encapsulation(
        &mut self,
        enc: Option<(EncapsulationType, u16, bool)>,
    ) -> Result<&mut Self, Box<dyn Error>> {
        if self.kind.is_sub() {
            self.encapsulation = enc;
            Ok(self)
        } else {
            todo!()
        }
    }
    pub fn dot1q(&mut self, vlan: u16, as_native: bool) -> &mut Self {
        self.encapsulation = Some((EncapsulationType::Dot1Q, vlan, as_native));
        self
    }
}
impl Display for Interface {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "interface {}", self.kind)
    }
}
/// # DistributeType
/// `DistributeType` - Routing Distribution type enum
#[derive(Debug, Clone, PartialEq)]
pub enum DistributeType {
    Ospf(u32),
    Eigrp(u32),
    Rip,
    Bgp,
    Static,
    Connected,
}
impl Display for DistributeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Connected => "connected".to_string(),
                Self::Static => "static".to_string(),
                Self::Rip => "rip".to_string(),
                Self::Ospf(pid) => format!("ospf {pid}"),
                Self::Eigrp(asn) => format!("eigrp {asn}"),
                Self::Bgp => "bgp".to_string(),
            }
        )
    }
}
/// # OSPF
/// `Ospf` - Ospf protocol struct
#[derive(Debug, Clone)]
pub struct Ospf {
    pid: u32,
    router_id: Option<[u8; 4]>,
    default_originate: Option<bool>,
    net: Vec<Network>,
    passive_if: Vec<InterfaceKind>,
    redistribute: Vec<DistributeType>,
    neighbors: Vec<IpAddress>,
    ref_bandwidth: Option<u32>,
}
impl Ospf {
    pub fn new(pid: u32) -> Ospf {
        Ospf {
            pid,
            router_id: None,
            default_originate: None,
            net: vec![],
            passive_if: vec![],
            redistribute: vec![],
            neighbors: vec![],
            ref_bandwidth: None,
        }
    }
    pub fn router_id(&mut self, id: Option<[u8; 4]>) -> &mut Self {
        self.router_id = id;
        self
    }
    pub fn default_originate(&mut self, def: Option<bool>) -> &mut Self {
        self.default_originate = def;
        self
    }
    pub fn reference_bandwidth(
        &mut self,
        ref_bandwidth: Option<u32>,
    ) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(ref_bw) = ref_bandwidth {
            if ref_bw > 0 && ref_bw < 4294967 {
                self.ref_bandwidth = ref_bandwidth;
                Ok(self)
            } else {
                todo!() // Create a custom error
            }
        } else {
            self.ref_bandwidth = None;
            Ok(self)
        }
    }
    pub fn add_network(&mut self, net: &Network) -> &mut Self {
        if self.net.iter().any(|n| n == net) {
            self
        } else {
            self.net.push(net.clone());
            self
        }
    }
    pub fn remove_network(&mut self, net: &Network) -> &mut Self {
        self.net.retain(|n| n != net);
        self
    }
    pub fn add_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        if self.passive_if.iter().any(|i| i == inf) {
            self
        } else {
            self.passive_if.push(inf.clone());
            self
        }
    }
    pub fn remove_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        self.passive_if.retain(|i| i != inf);
        self
    }
    pub fn add_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        if self.redistribute.iter().any(|r| r == red) {
            self
        } else {
            self.redistribute.push(red.clone());
            self
        }
    }
    pub fn remove_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        self.redistribute.retain(|r| r != red);
        self
    }
    pub fn add_neighbor(&mut self, neighbor: &IpAddress) -> &mut Self {
        if self.neighbors.iter().any(|n| n == neighbor) {
            self
        } else {
            self.neighbors.push(neighbor.clone());
            self
        }
    }
    pub fn remove_neighbor(&mut self, neighbor: &IpAddress) -> &mut Self {
        self.neighbors.retain(|n| n != neighbor);
        self
    }
}
impl Config for Ospf {
    fn config(&self) -> String {
        let mut config = String::new();
        if let Some(id) = &self.router_id {
            config = format!("router-id {}.{}.{}.{}\n", id[0], id[1], id[2], id[3]);
        }
        if let Some(ref_bandwidth) = self.ref_bandwidth {
            config = format!("{config}auto-cost reference-bandwidth {ref_bandwidth}\n");
        }
        if let Some(default_originate) = self.default_originate {
            config = format!(
                "{config}{}",
                if default_originate {
                    "default-information originate\n"
                } else {
                    "no default-information originate\n"
                }
            );
        }
        for net in &self.net {
            config = format!("{config}network {}\n", net.netid())
        }
        for inf in &self.passive_if {
            config = format!("{config}passive-interface {}\n", inf)
        }
        for rest in &self.redistribute {
            config = format!("{config}redistribute {}\n", rest);
        }
        for neighbor in &self.neighbors {
            config = format!("{config}neighbor {}\n", neighbor.address())
        }
        if config != "" {
            format!("router ospf {}\n{config}exit\n", self.pid)
        } else {
            config
        }
    }
}
/// # EIGRP
/// `Eigrp` - Eigrp protocol struct
#[derive(Debug, Clone)]
pub struct Eigrp {
    as_number: u16,
    auto_sum: Option<bool>,
    router_id: Option<[u8; 4]>,
    net: Vec<Network>,
    passive_if: Vec<InterfaceKind>,
    redistribute: Vec<DistributeType>,
    neighbors: Vec<IpAddress>,
    variance: Option<u8>,
}
impl Eigrp {
    pub fn new(as_num: u16) -> Eigrp {
        Eigrp {
            as_number: as_num,
            auto_sum: None,
            router_id: None,
            net: vec![],
            passive_if: vec![],
            redistribute: vec![],
            neighbors: vec![],
            variance: None,
        }
    }
    pub fn router_id(&mut self, id: Option<[u8; 4]>) -> &mut Self {
        self.router_id = id;
        self
    }
    pub fn auto_sum(&mut self, auto_sum: Option<bool>) -> &mut Self {
        self.auto_sum = auto_sum;
        self
    }
    pub fn variance(&mut self, var: Option<u8>) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(var) = var {
            if var > 0 && var == 128 {
                self.variance = Some(var);
                Ok(self)
            } else {
                todo!()
            }
        } else {
            self.variance = var;
            Ok(self)
        }
    }
    pub fn add_network(&mut self, net: &Network) -> &mut Self {
        if self.net.iter().any(|n| n == net) {
            self
        } else {
            self.net.push(net.clone());
            self
        }
    }
    pub fn remove_network(&mut self, net: &Network) -> &mut Self {
        self.net.retain(|n| n != net);
        self
    }
    pub fn add_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        if self.passive_if.iter().any(|i| i == inf) {
            self
        } else {
            self.passive_if.push(inf.clone());
            self
        }
    }
    pub fn remove_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        self.passive_if.retain(|i| i != inf);
        self
    }
    pub fn add_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        if self.redistribute.iter().any(|r| r == red) {
            self
        } else {
            self.redistribute.push(red.clone());
            self
        }
    }
    pub fn remove_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        self.redistribute.retain(|r| r != red);
        self
    }
    pub fn add_neighbor(&mut self, neighbor: &IpAddress) -> &mut Self {
        if self.neighbors.iter().any(|n| n == neighbor) {
            self
        } else {
            self.neighbors.push(neighbor.clone());
            self
        }
    }
    pub fn remove_neighbor(&mut self, neighbor: &IpAddress) -> &mut Self {
        self.neighbors.retain(|n| n != neighbor);
        self
    }
}
impl Config for Eigrp {
    fn config(&self) -> String {
        let mut config = String::new();
        if let Some(id) = &self.router_id {
            config = format!("eigrp router-id {}.{}.{}.{}\n", id[0], id[1], id[2], id[3]);
        }
        if let Some(variance) = self.variance {
            config = format!("{config}variance {variance}\n");
        }
        for net in &self.net {
            config = format!("{config}network {}\n", net.netid())
        }
        for inf in &self.passive_if {
            config = format!("{config}passive-interface {}\n", inf)
        }
        for rest in &self.redistribute {
            config = format!("{config}redistribute {}\n", rest);
        }
        for neighbor in &self.neighbors {
            config = format!("{config}neighbor {}\n", neighbor.address())
        }
        if config != "" {
            format!("router eigrp {}\n{config}exit\n", self.as_number)
        } else {
            config
        }
    }
}
/// # RipVersion
/// `RipVersion` - Rip protocol versions enum
#[derive(Debug, Clone)]
pub enum RipVersion {
    V1,
    V2,
}
/// # RIP
/// `Rip` - Rip protocol struct
#[derive(Debug, Clone)]
pub struct Rip {
    version: Option<RipVersion>,
    auto_sum: Option<bool>,
    default_originate: Option<bool>,
    net: Vec<Network>,
    passive_if: Vec<InterfaceKind>,
    redistribute: Vec<DistributeType>,
}
impl Rip {
    pub fn new() -> Rip {
        Rip {
            version: None,
            auto_sum: None,
            default_originate: None,
            net: vec![],
            passive_if: vec![],
            redistribute: vec![],
        }
    }
    pub fn version(&mut self, ver: Option<RipVersion>) -> &mut Self {
        self.version = ver;
        self
    }
    pub fn version_1(&mut self) -> &mut Self {
        self.version = Some(RipVersion::V1);
        self
    }
    pub fn version_2(&mut self) -> &mut Self {
        self.version = Some(RipVersion::V2);
        self
    }
    pub fn default_originate(&mut self, def: Option<bool>) -> &mut Self {
        self.default_originate = def;
        self
    }
    pub fn auto_sum(&mut self, auto_sum: Option<bool>) -> &mut Self {
        self.auto_sum = auto_sum;
        self
    }
    pub fn add_network(&mut self, net: &Network) -> &mut Self {
        if self.net.iter().any(|n| n == net) {
            self
        } else {
            self.net.push(net.clone());
            self
        }
    }
    pub fn remove_network(&mut self, net: &Network) -> &mut Self {
        self.net.retain(|n| n != net);
        self
    }
    pub fn add_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        if self.passive_if.iter().any(|i| i == inf) {
            self
        } else {
            self.passive_if.push(inf.clone());
            self
        }
    }
    pub fn remove_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        self.passive_if.retain(|i| i != inf);
        self
    }
    pub fn add_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        if self.redistribute.iter().any(|r| r == red) {
            self
        } else {
            self.redistribute.push(red.clone());
            self
        }
    }
    pub fn remove_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        self.redistribute.retain(|r| r != red);
        self
    }
}
impl Config for Rip {
    fn config(&self) -> String {
        let mut config = String::new();
        if let Some(version) = &self.version {
            config = match version {
                RipVersion::V1 => "version 1\n".to_string(),
                RipVersion::V2 => "version 2\n".to_string(),
            };
        }
        if let Some(auto_sum) = self.auto_sum {
            config = format!(
                "{config}{}",
                if auto_sum {
                    "auto-summary\n"
                } else {
                    "no auto-summary\n"
                }
            );
        }
        if let Some(default_originate) = self.default_originate {
            config = format!(
                "{config}{}",
                if default_originate {
                    "default-information originate\n"
                } else {
                    "no default-information originate\n"
                }
            );
        }
        for net in &self.net {
            config = format!("{config}network {}\n", net.netid())
        }
        for inf in &self.passive_if {
            config = format!("{config}passive-interface {}\n", inf)
        }
        for rest in &self.redistribute {
            config = format!("{config}redistribute {}\n", rest);
        }
        if config != "" {
            format!("router rip\n{config}exit\n")
        } else {
            config
        }
    }
}
/// #BGP
/// `Bgp` - Bgp protocol struct
#[derive(Debug, Clone)]
pub struct Bgp {
    as_number: u16,
    router_id: Option<[u8; 4]>,
    net: Vec<Network>,
    passive_if: Vec<InterfaceKind>,
    redistribute: Vec<DistributeType>,
    neighbors: Vec<IpAddress>,
    sync: Option<bool>,
}
impl Bgp {
    pub fn new(as_num: u16) -> Bgp {
        Bgp {
            as_number: as_num,
            router_id: None,
            sync: None,
            net: vec![],
            passive_if: vec![],
            redistribute: vec![],
            neighbors: vec![],
        }
    }
    pub fn router_id(&mut self, id: Option<[u8; 4]>) -> &mut Self {
        self.router_id = id;
        self
    }
    pub fn synchronization(&mut self, sync: Option<bool>) -> &mut Self {
        self.sync = sync;
        self
    }
    pub fn add_network(&mut self, net: &Network) -> &mut Self {
        if self.net.iter().any(|n| n == net) {
            self
        } else {
            self.net.push(net.clone());
            self
        }
    }
    pub fn remove_network(&mut self, net: &Network) -> &mut Self {
        self.net.retain(|n| n != net);
        self
    }
    pub fn add_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        if self.passive_if.iter().any(|i| i == inf) {
            self
        } else {
            self.passive_if.push(inf.clone());
            self
        }
    }
    pub fn remove_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        self.passive_if.retain(|i| i != inf);
        self
    }
    pub fn add_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        if self.redistribute.iter().any(|r| r == red) {
            self
        } else {
            self.redistribute.push(red.clone());
            self
        }
    }
    pub fn remove_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        self.redistribute.retain(|r| r != red);
        self
    }
    pub fn add_neighbor(&mut self, neighbor: &IpAddress) -> &mut Self {
        if self.neighbors.iter().any(|n| n == neighbor) {
            self
        } else {
            self.neighbors.push(neighbor.clone());
            self
        }
    }
    pub fn remove_neighbor(&mut self, neighbor: &IpAddress) -> &mut Self {
        self.neighbors.retain(|n| n != neighbor);
        self
    }
}
impl Config for Bgp {
    fn config(&self) -> String {
        let mut config = String::new();
        if let Some(id) = &self.router_id {
            config = format!("bgp router-id {}.{}.{}.{}\n", id[0], id[1], id[2], id[3]);
        }
        if let Some(sync) = self.sync {
            config = format!("{config}{}synchronization\n", if sync { "" } else { "no " });
        }
        for net in &self.net {
            config = format!("{config}network {}\n", net.netid())
        }
        for inf in &self.passive_if {
            config = format!("{config}passive-interface {}\n", inf)
        }
        for rest in &self.redistribute {
            config = format!("{config}redistribute {}\n", rest);
        }
        for neighbor in &self.neighbors {
            config = format!("{config}neighbor {}\n", neighbor.address())
        }
        if config != "" {
            format!("router bgp {}\n{config}exit\n", self.as_number)
        } else {
            config
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum StaticRoute {
    ViaAddress(IpAddress),
    ViaInterface(InterfaceKind),
}
#[derive(Debug, Clone)]
pub enum Route {
    Static(Network, StaticRoute),
    OSPF(Ospf),
    EIGRP(Eigrp),
    BGP(Bgp),
    RIP(Rip),
}
impl Route {
    pub fn default_route(via: StaticRoute) -> Route {
        Route::Static(Network::from_str("0.0.0.0/0").unwrap(), via)
    }
}
impl Config for Route {
    fn config(&self) -> String {
        match self {
            Route::Static(dst, static_route) => match static_route {
                StaticRoute::ViaAddress(via) => {
                    format!("ip route {} {} {}", dst.netid(), dst.mask(), via)
                }
                StaticRoute::ViaInterface(via) => {
                    format!("ip route {} {} {}", dst.netid(), dst.mask(), via)
                }
            },
            Route::RIP(rip) => rip.config(),
            Route::OSPF(ospf) => ospf.config(),
            Route::EIGRP(eigrp) => eigrp.config(),
            Route::BGP(bgp) => bgp.config(),
        }
    }
}

pub trait RouterConfig {
    type Interface;
    type Line;
    fn hostname(&mut self, hostname: Option<String>) -> Result<&mut Self, Box<dyn Error>>;
    fn secret(&mut self, secret: Option<String>) -> Result<&mut Self, Box<dyn Error>>;
    fn password(&mut self, passwd: Option<String>) -> Result<&mut Self, Box<dyn Error>>;
    fn add_service(&mut self, service: Service) -> Result<&mut Self, Box<dyn Error>>;
    fn banner(&mut self, banner: Option<String>) -> Result<&mut Self, Box<dyn Error>>;
    fn add_line(&mut self, line: &Self::Line) -> Result<&mut Self, Box<dyn Error>>;
    fn add_interface(&mut self, interface: &Self::Interface) -> Result<&mut Self, Box<dyn Error>>;
    fn enable_ipv6(&mut self, enable: Option<bool>) -> Result<&mut Self, Box<dyn Error>>;
    fn add_static_route(
        &mut self,
        dst: Network,
        static_route: StaticRoute,
    ) -> Result<&mut Self, Box<dyn Error>>;
}

#[derive(Default, Debug, Clone)]
pub struct Router {
    hostname: Option<String>,
    secret: Option<String>,
    password: Option<String>,
    banner: Option<String>,
    enable_ipv6: Option<bool>,
    lines: Vec<Line>,
    services: Vec<Service>,
    interfaces: Vec<Interface>,
    routes: Vec<Route>,
}
impl RouterConfig for Router {
    type Interface = Interface;
    type Line = Line;
    fn hostname(&mut self, hostname: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.hostname = hostname;
        Ok(self)
    }
    fn secret(&mut self, secret: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.secret = secret;
        Ok(self)
    }
    fn password(&mut self, passwd: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.password = passwd;
        Ok(self)
    }
    fn add_service(&mut self, service: Service) -> Result<&mut Self, Box<dyn Error>> {
        self.services.push(service);
        Ok(self)
    }
    fn banner(&mut self, banner: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.banner = banner;
        Ok(self)
    }
    fn add_line(&mut self, line: &Self::Line) -> Result<&mut Self, Box<dyn Error>> {
        if self.lines.iter().any(|l| l == line) {
            Ok(self)
        } else {
            self.lines.push(line.clone());
            Ok(self)
        }
    }
    fn add_interface(&mut self, interface: &Self::Interface) -> Result<&mut Self, Box<dyn Error>> {
        if self.interfaces.iter().any(|inf| inf.kind == interface.kind) {
            Ok(self)
        } else {
            self.interfaces.push(interface.clone());
            Ok(self)
        }
    }
    fn enable_ipv6(&mut self, enable: Option<bool>) -> Result<&mut Self, Box<dyn Error>> {
        self.enable_ipv6 = enable;
        Ok(self)
    }
    fn add_static_route(
        &mut self,
        dst: Network,
        static_route: StaticRoute,
    ) -> Result<&mut Self, Box<dyn Error>> {
        for route in &self.routes {
            if let Route::Static(net, static_r) = &route {
                if static_route == *static_r && dst == *net {
                    todo!()
                }
            }
        }
        self.routes.push(Route::Static(dst, static_route));
        Ok(self)
    }
}
impl Config for Interface {
    fn config(&self) -> String {
        let mut base_config = String::new();
        let mut config = String::new();
        let enter = self.to_string();
        let is_sub = self.kind.is_sub();
        let base_enter = if is_sub {
            format!("{}\n", enter.split('.').collect::<Vec<&str>>()[0])
        } else {
            "".to_string()
        };
        if let Some(mac) = &self.mac {
            if is_sub {
                base_config = format!("mac-address {}\n", mac.cisco_format())
            } else {
                config = format!("mac-address {}\n", mac.cisco_format())
            }
        }
        if let Some(state) = &self.state {
            if is_sub {
                base_config = match state {
                    InterfaceState::Up => format!("{base_config}no shutdown\n"),
                    InterfaceState::Down => format!("{base_config}shutdown\n"),
                }
            } else {
                config = match state {
                    InterfaceState::Up => format!("{config}no shutdown\n"),
                    InterfaceState::Down => format!("{config}shutdown\n"),
                }
            }
        }
        if let Some(speed) = &self.speed {
            if is_sub {
                if *speed == 0 {
                    base_config = format!("{base_config}speed auto\n");
                } else {
                    base_config = format!("{base_config}speed {}\n", speed);
                }
            } else {
                if *speed == 0 {
                    config = format!("{config}speed auto\n");
                } else {
                    config = format!("{config}speed {}\n", speed);
                }
            }
        }
        if let Some((enc, vlan, as_native)) = &self.encapsulation {
            config = format!(
                "{config}encapsulation {} {}{}\n",
                enc,
                vlan,
                if *as_native { " native" } else { "" }
            );
        }
        if let Some((address, mask)) = &self.ipv4 {
            config = format!("{config}ip address {} {}\n", address.address(), mask.mask());
        }
        if let Some((address, prefix)) = &self.ipv6 {
            config = format!("{config}ipv6 address {}/{}\n", address.address(), prefix);
        }
        if let Some(bandwidth) = &self.bandwidth {
            config = format!("{config}bandwidth {}\n", bandwidth);
        }
        if let Some(mtu) = &self.mtu {
            config = format!("{config}mtu {}\n", mtu);
        }
        if let Some(delay) = &self.delay {
            config = format!("{config}delay {}\n", delay);
        }
        if config != "" {
            format!(
                "{}{enter}\n{config}exit\n",
                if base_config != "" {
                    format!("{base_enter}{base_config}exit\n")
                } else {
                    "".to_string()
                }
            )
        } else {
            if base_config != "" {
                format!("{base_enter}{base_config}exit\n")
            } else {
                String::new()
            }
        }
    }
}
impl Config for Router {
    fn config(&self) -> String {
        let mut config = String::new();
        if let Some(hname) = &self.hostname {
            config = format!("{config}hostname {}\n", hname);
        }
        if let Some(sec) = &self.secret {
            config = format!("{config}enable secret {}\n", sec);
        }
        if let Some(passwd) = &self.password {
            config = format!("{config}enable password {}\n", passwd);
        }
        if let Some(banner) = &self.banner {
            config = format!("{config}banner motd {}\n", banner);
        }
        if let Some(enable_ipv6) = &self.enable_ipv6 {
            config = format!(
                "{config}{}ipv6 unicast-routing\n",
                if *enable_ipv6 { "" } else { "no " }
            );
        }
        if self.lines.len() > 0 {
            for line in &self.lines {
                config = format!("{config}{}", line.config());
            }
        }
        if self.services.len() > 0 {
            for service in &self.services {
                config = format!("{config}{}", service.config());
            }
        }
        if self.interfaces.len() > 0 {
            for interface in &self.interfaces {
                config = format!("{config}{}", interface.config());
            }
        }
        if self.routes.len() > 0 {
            for route in &self.routes {
                config = format!("{config}{}", route.config())
            }
        }
        config
    }
}
