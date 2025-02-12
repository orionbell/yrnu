use super::{InterfaceKind, InterfaceState, Line, Service};
use crate::config::Config;
use crate::core::{IpAddress, IpVersion, MacAddress, Mask, Network};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
/// # EncapsulationType
/// `EncapsulationType` - Encapsulation types of router sub interface enum
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
/// `Interface` - Router Interface config struct
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
    /// Creates a new instance of router Interface
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
    /// Creates a new Interface instance already configured as GigabitEthernet interface
    pub fn gigabit_ethernet(indexes: Vec<u8>) -> Self {
        Interface::new(InterfaceKind::GigabitEthernet(indexes, None))
    }
    /// Creates a new Interface instance already configured as FastEthernet interface
    pub fn fast_ethernet(indexes: Vec<u8>) -> Self {
        Interface::new(InterfaceKind::FastEthernet(indexes, None))
    }
    /// Creates a new sub Interface instance already configured as GigabitEthernet interface
    pub fn sub_gigabit_ethernet(indexes: Vec<u8>, ind: u32) -> Self {
        Interface::new(InterfaceKind::GigabitEthernet(indexes, Some(ind)))
    }
    /// Creates a new sub Interface instance already configured as FastEthernet interface
    pub fn sub_fast_ethernet(indexes: Vec<u8>, ind: u32) -> Self {
        Interface::new(InterfaceKind::FastEthernet(indexes, Some(ind)))
    }
    /// Creates a new Interface instance already configured as Loopback interface
    pub fn loopback(index: u8) -> Self {
        Interface::new(InterfaceKind::Lookback(index))
    }
    /// Configure interface bandwidth
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
    /// Configure interface delay
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
    /// Configure interface mtu
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
    /// Configure interface speed
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
    /// Configure interface mac address
    pub fn mac(&mut self, mac: Option<MacAddress>) -> &mut Self {
        self.mac = mac;
        self
    }
    /// Configure interface description
    pub fn description(&mut self, desc: Option<String>) -> &mut Self {
        self.description = desc;
        self
    }
    /// Turn on the interface
    pub fn turn_on(&mut self) -> &mut Self {
        Self::state(self, Some(InterfaceState::Up))
    }
    /// Turn off the interface
    pub fn shutdown(&mut self) -> &mut Self {
        Self::state(self, Some(InterfaceState::Down))
    }
    /// Configure interface state
    pub fn state(&mut self, state: Option<InterfaceState>) -> &mut Self {
        self.state = state;
        self
    }
    /// Configure interface ipv4 address
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
    /// Configure interface ipv6 address (this would not enable ipv6 on the router)
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
    /// Configure sub interface encapsulation
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
    /// Configure sub interface as Dot1Q
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

#[derive(Debug, Clone, PartialEq)]
pub enum OspfAreaType {
    Stub(u32),
    Nss(u32),
    Tstub(u32),
    Tnss(u32),
}
impl FromStr for OspfAreaType {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut slice = s.split(" ");
        let area = slice.next();
        if area.is_none() {
            return Err(todo!());
        }
        let area = area.unwrap().parse();
        if area.is_err() {
            return Err(todo!());
        }
        let area = area.unwrap();
        let typ = slice.next().unwrap_or("");
        match typ.to_lowercase().as_str() {
            "stub" => Ok(Self::Stub(area)),
            "nss" => Ok(Self::Nss(area)),
            "tnss" => Ok(Self::Tnss(area)),
            "tstub" => Ok(Self::Tstub(area)),
            _ => Err(todo!()),
        }
    }
}
impl Config for OspfAreaType {
    fn config(&self) -> String {
        match self {
            Self::Stub(area) => format!("{area} stub"),
            Self::Nss(area) => format!("{area} nssa"),
            Self::Tstub(area) => format!("{area} stub no-summary"),
            Self::Tnss(area) => format!("{area} nssa no-summary"),
        }
    }
}
/// # OSPF Configuration
/// `Ospf` - Ospf protocol config struct
#[derive(Debug, Clone)]
pub struct Ospf {
    pid: u32,
    router_id: Option<[u8; 4]>,
    default_originate: Option<bool>,
    net: Vec<(Network, u32)>,
    passive_if: Vec<InterfaceKind>,
    redistribute: Vec<DistributeType>,
    neighbors: Vec<IpAddress>,
    ref_bandwidth: Option<u32>,
    special_areas: Vec<OspfAreaType>,
}
impl Ospf {
    /// Creates a new `Ospf` config instance
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
            special_areas: vec![],
        }
    }
    /// Configure `OSPF` router id
    pub fn router_id(&mut self, id: Option<[u8; 4]>) -> &mut Self {
        self.router_id = id;
        self
    }
    /// Configure `OSPF` default originate
    pub fn default_originate(&mut self, def: Option<bool>) -> &mut Self {
        self.default_originate = def;
        self
    }
    /// Configure `OSPF` reference bandwidth
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
    /// Add network to the `OSPF` list of networks to distribute
    pub fn add_network(&mut self, net: &Network, area: u32) -> &mut Self {
        if self.net.iter().any(|(n, a)| n == net && *a == area) {
            self
        } else {
            self.net.push((net.clone(), area));
            self
        }
    }
    /// Remove network from the `OSPF` list of networks to distribute
    pub fn remove_network(&mut self, net: &Network, area: u32) -> &mut Self {
        self.net.retain(|(n, a)| n != net || *a != area);
        self
    }
    /// Add passive interface to the `OSPF` config
    pub fn add_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        if self.passive_if.iter().any(|i| i == inf) {
            self
        } else {
            self.passive_if.push(inf.clone());
            self
        }
    }
    /// Remove passive interface from the `OSPF` config
    pub fn remove_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        self.passive_if.retain(|i| i != inf);
        self
    }
    /// Add distribute type to the redistribution list of `OSPF` config
    pub fn add_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        if self.redistribute.iter().any(|r| r == red) {
            self
        } else {
            self.redistribute.push(red.clone());
            self
        }
    }
    /// Remove distribute type from the redistribution list of the `OSPF` config
    pub fn remove_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        self.redistribute.retain(|r| r != red);
        self
    }
    /// Add neighbor to the the neighbors list of the `OSPF` config
    pub fn add_neighbor(&mut self, neighbor: &IpAddress) -> &mut Self {
        if self.neighbors.iter().any(|n| n == neighbor) {
            self
        } else {
            self.neighbors.push(neighbor.clone());
            self
        }
    }
    /// Remove neighbor from the the neighbors list of the `OSPF` config
    pub fn remove_neighbor(&mut self, neighbor: &IpAddress) -> &mut Self {
        self.neighbors.retain(|n| n != neighbor);
        self
    }
    /// Add special area to the special areas list in the `OSPF` config
    pub fn add_special_area(&mut self, kind: &OspfAreaType) -> &mut Self {
        if self.special_areas.iter().any(|k| k == kind) {
            self
        } else {
            self.special_areas.push(kind.clone());
            self
        }
    }
    /// Remove special area from the special areas list in `OSPF` config
    pub fn remove_special_area(&mut self, area: &OspfAreaType) -> &mut Self {
        self.special_areas.retain(|a| a != area);
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
        for (net, area) in &self.net {
            config = format!(
                "{config}network {} {} area {}\n",
                net.netid(),
                net.mask().wildcard(),
                area
            )
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
        for area in &self.special_areas {
            config = format!("{config}area {}\n", area.config());
        }
        if config != "" {
            format!("router ospf {}\n{config}exit\n", self.pid)
        } else {
            config
        }
    }
}
/// # EIGRP Configuration
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
    /// Creates a new `Eigrp` config instance
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
    /// Configure `Eigrp` router id
    pub fn router_id(&mut self, id: Option<[u8; 4]>) -> &mut Self {
        self.router_id = id;
        self
    }
    /// Configure `Eigrp` auto summary
    pub fn auto_sum(&mut self, auto_sum: Option<bool>) -> &mut Self {
        self.auto_sum = auto_sum;
        self
    }
    /// Configure `Eigrp` variance value
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
    /// Add network to the `Eigrp` list of networks to distribute
    pub fn add_network(&mut self, net: &Network) -> &mut Self {
        if self.net.iter().any(|n| n == net) {
            self
        } else {
            self.net.push(net.clone());
            self
        }
    }
    /// Remove network from the `Eigrp` list of networks to distribute
    pub fn remove_network(&mut self, net: &Network) -> &mut Self {
        self.net.retain(|n| n != net);
        self
    }
    /// Add passive interface to the `Eigrp` config
    pub fn add_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        if self.passive_if.iter().any(|i| i == inf) {
            self
        } else {
            self.passive_if.push(inf.clone());
            self
        }
    }
    /// Remove passive interface to the `Eigrp` config
    pub fn remove_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        self.passive_if.retain(|i| i != inf);
        self
    }
    /// Add distribute type to the redistribution list of `Eigrp` config
    pub fn add_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        if self.redistribute.iter().any(|r| r == red) {
            self
        } else {
            self.redistribute.push(red.clone());
            self
        }
    }
    /// Add distribute type to the redistribution list of `Eigrp` config
    pub fn remove_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        self.redistribute.retain(|r| r != red);
        self
    }
    /// Add neighbor to the the neighbors list of the `Eigrp` config
    pub fn add_neighbor(&mut self, neighbor: &IpAddress) -> &mut Self {
        if self.neighbors.iter().any(|n| n == neighbor) {
            self
        } else {
            self.neighbors.push(neighbor.clone());
            self
        }
    }
    /// Remove neighbor to the the neighbors list of the `Eigrp` config
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
/// # RIP Configuration
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
    /// Creates a new `Rip` config instance
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
    /// Configure `RIP` version
    pub fn version(&mut self, ver: Option<RipVersion>) -> &mut Self {
        self.version = ver;
        self
    }
    /// Set the `RIP` version to version 1
    pub fn version_1(&mut self) -> &mut Self {
        self.version = Some(RipVersion::V1);
        self
    }
    /// Set the `RIP` version to version 2
    pub fn version_2(&mut self) -> &mut Self {
        self.version = Some(RipVersion::V2);
        self
    }
    /// Configure `RIP` default originate
    pub fn default_originate(&mut self, def: Option<bool>) -> &mut Self {
        self.default_originate = def;
        self
    }
    /// Configure `RIP` auto summary
    pub fn auto_sum(&mut self, auto_sum: Option<bool>) -> &mut Self {
        self.auto_sum = auto_sum;
        self
    }
    /// Add network to the `RIP` list of networks to distribute
    pub fn add_network(&mut self, net: &Network) -> &mut Self {
        if self.net.iter().any(|n| n == net) {
            self
        } else {
            self.net.push(net.clone());
            self
        }
    }
    /// Remove network to the `RIP` list of networks to distribute
    pub fn remove_network(&mut self, net: &Network) -> &mut Self {
        self.net.retain(|n| n != net);
        self
    }
    /// Add passive interface to the `RIP` config
    pub fn add_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        if self.passive_if.iter().any(|i| i == inf) {
            self
        } else {
            self.passive_if.push(inf.clone());
            self
        }
    }
    /// Remove passive interface to the `RIP` config
    pub fn remove_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        self.passive_if.retain(|i| i != inf);
        self
    }
    /// Add distribute type to the redistribution list of `RIP` config
    pub fn add_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        if self.redistribute.iter().any(|r| r == red) {
            self
        } else {
            self.redistribute.push(red.clone());
            self
        }
    }
    /// Remove distribute type to the redistribution list of `RIP` config
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
/// # BGP Configuration
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
    /// Creates a new `Bgp` config instance
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
    /// Configure `Bgp` router id
    pub fn router_id(&mut self, id: Option<[u8; 4]>) -> &mut Self {
        self.router_id = id;
        self
    }
    /// Configure `Bgp` synchronization
    pub fn synchronization(&mut self, sync: Option<bool>) -> &mut Self {
        self.sync = sync;
        self
    }
    /// Add network to the `Bgp` list of networks to distribute
    pub fn add_network(&mut self, net: &Network) -> &mut Self {
        if self.net.iter().any(|n| n == net) {
            self
        } else {
            self.net.push(net.clone());
            self
        }
    }
    /// Remove network to the `Bgp` list of networks to distribute
    pub fn remove_network(&mut self, net: &Network) -> &mut Self {
        self.net.retain(|n| n != net);
        self
    }
    /// Add passive interface to the `Bgp` config
    pub fn add_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        if self.passive_if.iter().any(|i| i == inf) {
            self
        } else {
            self.passive_if.push(inf.clone());
            self
        }
    }
    /// Remove passive interface to the `Bgp` config
    pub fn remove_passive_if(&mut self, inf: &InterfaceKind) -> &mut Self {
        self.passive_if.retain(|i| i != inf);
        self
    }
    /// Add distribute type to the redistribution list of `Bgp` config
    pub fn add_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        if self.redistribute.iter().any(|r| r == red) {
            self
        } else {
            self.redistribute.push(red.clone());
            self
        }
    }
    /// Remove distribute type to the redistribution list of `Bgp` config
    pub fn remove_redistribute_type(&mut self, red: &DistributeType) -> &mut Self {
        self.redistribute.retain(|r| r != red);
        self
    }
    /// Add neighbor to the the neighbors list of the `Bgp` config
    pub fn add_neighbor(&mut self, neighbor: &IpAddress) -> &mut Self {
        if self.neighbors.iter().any(|n| n == neighbor) {
            self
        } else {
            self.neighbors.push(neighbor.clone());
            self
        }
    }
    /// Remove neighbor to the the neighbors list of the `Bgp` config
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
/// # StaticRoute
/// `StaticRoute` - static route via options enum
#[derive(Debug, Clone, PartialEq)]
pub enum StaticRoute {
    ViaAddress(IpAddress),
    ViaInterface(InterfaceKind),
}
/// # Route Configuration
/// `Route` - Network route options enum
#[derive(Debug, Clone)]
pub enum Route {
    Static(Network, StaticRoute),
    OSPF(Ospf),
    EIGRP(Eigrp),
    BGP(Bgp),
    RIP(Rip),
}
impl Route {
    /// Create a default route
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

/// # DHCP pool configuration
#[derive(Debug, Clone, PartialEq)]
pub struct DhcpPool {
    name: String,
    net: Network,
    def: Option<IpAddress>,
    dns: Option<IpAddress>,
    domain: Option<String>,
    excluded_addresses: Option<(IpAddress, IpAddress)>,
}
impl DhcpPool {
    /// Creates a new `DhcpPool` configuration instance
    pub fn new(name: String, net: Network) -> Self {
        Self {
            name,
            net,
            def: None,
            dns: None,
            domain: None,
            excluded_addresses: None,
        }
    }
    /// Config `DhcpPool` default gateway
    pub fn default_gateway(&mut self, def: Option<IpAddress>) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(def) = def {
            if self.net.contains(&def) {
                self.def = Some(def);
                Ok(self)
            } else {
                Err(todo!())
            }
        } else {
            self.def = None;
            Ok(self)
        }
    }
    /// Config `DhcpPool` dns server
    pub fn dns(&mut self, dns: Option<IpAddress>) -> &mut Self {
        self.dns = dns;
        self
    }
    /// Config `DhcpPool` domain name
    pub fn domain(&mut self, domain: Option<String>) -> &mut Self {
        self.domain = domain;
        self
    }
    /// Config `DhcpPool` excluded addresses range
    pub fn excluded_addresses(
        &mut self,
        low: Option<&IpAddress>,
        high: Option<&IpAddress>,
    ) -> Result<&mut Self, Box<dyn Error>> {
        let low = low.unwrap_or(self.net.netid());
        let high = high.unwrap_or(self.net.broadcast());
        if self.net.netid() == low && self.net.broadcast() == high {
            self.excluded_addresses = None;
            Ok(self)
        } else if (self.net.contains(&low) || low == self.net.netid())
            && (self.net.contains(&high) || self.net.broadcast() == high)
        {
            self.excluded_addresses = Some((low.clone(), high.clone()));
            Ok(self)
        } else {
            Err(todo!())
        }
    }
}
impl Config for DhcpPool {
    fn config(&self) -> String {
        let mut config = format!(
            "ip dhcp pool {}\nnetwork {} {}\n",
            self.name,
            self.net.netid(),
            self.net.mask()
        );
        if let Some(def) = &self.def {
            config = format!("{config}default-router {}\n", def);
        }
        if let Some(dns) = &self.dns {
            config = format!("{config}dns-server {}\n", dns);
        }
        if let Some(domain) = &self.domain {
            config = format!("{config}domain-name {}\n", domain);
        }
        if let Some((low, high)) = &self.excluded_addresses {
            config = format!("{config}exit\nip dhcp excluded-address {} {}\n", low, high);
        } else {
            config = format!("{config}exit\n")
        }
        config
    }
}

#[derive(Debug, Clone)]
pub struct Hsrp {
    group: Option<u16>,
    interface: InterfaceKind,
    virtual_address: IpAddress,
    priority: Option<u8>,
    preempt: Option<bool>,
    version: Option<u8>,
}
impl Hsrp {
    pub fn new(inf: InterfaceKind, addr: IpAddress) -> Self {
        Self {
            group: None,
            interface: inf,
            virtual_address: addr,
            priority: None,
            preempt: None,
            version: None,
        }
    }
    pub fn new_v1(inf: InterfaceKind, addr: IpAddress) -> Self {
        Self {
            group: None,
            interface: inf,
            virtual_address: addr,
            priority: None,
            preempt: None,
            version: Some(1),
        }
    }
    pub fn new_v2(inf: InterfaceKind, addr: IpAddress) -> Self {
        Self {
            group: None,
            interface: inf,
            virtual_address: addr,
            priority: None,
            preempt: None,
            version: Some(2),
        }
    }
    pub fn priority(&mut self, priority: Option<u8>) -> &mut Self {
        self.priority = priority;
        self
    }
    pub fn preempt(&mut self, preempt: Option<bool>) -> &mut Self {
        self.preempt = preempt;
        self
    }
    pub fn version(&mut self, ver: Option<u8>) -> Result<&mut Self, Box<dyn Error>> {
        if ver.is_some() && ver.unwrap() > 2 {
            return Err(todo!())
        }
        self.version = ver;
        Ok(self)
    }
    pub fn group(&mut self, group: Option<u16>) -> Result<&mut Self, Box<dyn Error>> {
        if group.is_some() && group.unwrap() > 4096 {
            return Err(todo!())
        }
        self.group = group;
        Ok(self)
    }
}
impl Config for Hsrp {
    fn config(&self) -> String {
        let mut config = format!("interface {}\n", self.interface);
        let group = if let Some(group) = self.group {
            format!("{} ", group)
        } else {
            String::new()
        };
        if *self.virtual_address.version() == IpVersion::V6 {
            config = format!("{config}standby {}ipv6\n", group);
        }
        config = format!(
            "{config}standby {}ip {}\n",
            group, self.virtual_address
        );
        if let Some(priority) = self.priority {
            config = format!("{config}standby {}priority {}\n", group, priority);
        }
        if self.preempt.is_some() && self.preempt.unwrap() {
            config = format!("{config}standby {}preempt\n", group);
        }
        if let Some(version) = self.version {
            config = format!("{config}standby version {}\n", version);
        }
        format!("{config}exit")
    }
}
/// # RouterConfig
/// `RouterConfig` - Router config trait
pub trait RouterConfig {
    type Interface;
    type Line;
    /// Configure Router `hostname`
    fn hostname(&mut self, hostname: Option<String>) -> Result<&mut Self, Box<dyn Error>>;
    /// Configure Router `enable secret`
    fn secret(&mut self, secret: Option<String>) -> Result<&mut Self, Box<dyn Error>>;
    /// Configure Router `enable password`
    fn password(&mut self, passwd: Option<String>) -> Result<&mut Self, Box<dyn Error>>;
    /// Add service to the Router config
    fn add_service(&mut self, service: Service) -> Result<&mut Self, Box<dyn Error>>;
    /// Configure Router `banner motd`
    fn banner(&mut self, banner: Option<String>) -> Result<&mut Self, Box<dyn Error>>;
    /// Add Line to the Router config
    fn add_line(&mut self, line: &Self::Line) -> Result<&mut Self, Box<dyn Error>>;
    /// Add Interface to the Router config
    fn add_interface(&mut self, interface: &Self::Interface) -> Result<&mut Self, Box<dyn Error>>;
    /// Enable Ipv6 routing on the router
    fn enable_ipv6(&mut self, enable: Option<bool>) -> Result<&mut Self, Box<dyn Error>>;
    /// Add static router to the Router config
    fn add_static_route(
        &mut self,
        dst: Network,
        static_route: StaticRoute,
    ) -> Result<&mut Self, Box<dyn Error>>;
    /// Add DHCP pool to the Router config
    fn add_dhcp_pool(&mut self, pool: DhcpPool) -> Result<&mut Self, Box<dyn Error>>;
}
/// # Router configuration
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
    dhcp_pools: Vec<DhcpPool>,
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
    fn add_dhcp_pool(&mut self, pool: DhcpPool) -> Result<&mut Self, Box<dyn Error>> {
        for dhcp_pool in &self.dhcp_pools {
            if pool == *dhcp_pool {
                return Err(todo!());
            }
        }
        self.dhcp_pools.push(pool);
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
        for line in &self.lines {
            config = format!("{config}{}", line.config());
        }
        for service in &self.services {
            config = format!("{config}{}", service.config());
        }
        for interface in &self.interfaces {
            config = format!("{config}{}", interface.config());
        }
        for route in &self.routes {
            config = format!("{config}{}", route.config())
        }
        for pool in &self.dhcp_pools {
            config = format!("{config}{}", pool.config())
        }
        config
    }
}
