//! # address.rs
//! The `address` module provides general purpese tools for heandling and managing Ip and Mac
//! addresses as well as defining networks and getting local network interfaces info.
use crate::errors::coreerr::{addrerr::*, iferr::*};

use core::fmt;
use pnet::datalink::NetworkInterface;
use pnet::{datalink::interfaces, ipnetwork::IpNetwork};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

/// ## MacAddress
/// `MacAddress` - MAC address struct
#[derive(Debug, Clone, PartialEq)]
pub struct MacAddress {
    address: String,
}
impl MacAddress {
    // private function for getting the parts of the `mac address`
    fn get_parts(address: &str) -> Vec<String> {
        address.split(':').map(str::to_string).collect()
    }
    /// returns the `mac address` as a vector
    pub fn as_vector(&self) -> Vec<String> {
        MacAddress::get_parts(self.address.as_str())
    }
    pub fn as_bytes(&self) -> [u8; 6] {
        let mut parts = [0u8; 6];
        for (i, part) in self.address.as_str().split(':').enumerate() {
            parts[i] = u8::from_str_radix(part, 16).unwrap();
        }
        parts
    }

    /// Checks if a giving `mac address` is valid
    pub fn is_valid(address: &str) -> bool {
        let parts = MacAddress::get_parts(address);
        if parts.len() != 6 {
            return false;
        }
        for part in parts {
            if part.len() != 2 || i64::from_str_radix(part.as_str(), 16).is_err() {
                return false;
            }
        }
        true
    }

    pub fn new(address: &str) -> Result<MacAddress, Box<dyn Error>> {
        if MacAddress::is_valid(address) {
            return Ok(MacAddress {
                address: address.to_uppercase().to_string(),
            });
        }
        Err(Box::new(InvalidMacAddress))
    }
    pub fn address(&self) -> &String {
        &self.address
    }
}
impl Display for MacAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.address)
    }
}
impl PartialOrd for MacAddress {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_vec = self.as_vector();
        let other_vec = other.as_vector();
        let mut self_hex;
        let mut other_hex;
        let mut index = 5;
        loop {
            self_hex = self_vec.get(index).unwrap();
            other_hex = other_vec.get(index).unwrap();
            if i64::from_str_radix(self_hex, 16) != i64::from_str_radix(other_hex, 16) {
                return Some(
                    i64::from_str_radix(self_hex, 16)
                        .unwrap()
                        .cmp(&i64::from_str_radix(other_hex, 16).unwrap()),
                );
            }
            if index == 0 {
                return Some(std::cmp::Ordering::Equal);
            }
            index -= 1;
        }
    }
}

/// # IpVersion
/// `IpVersion` - Internet Protocol (IP) versions enum.
#[derive(Debug, Clone, PartialEq)]
pub enum IpVersion {
    V4,
    V6,
}

/// # IpKind
/// `IpKind` - Internet Protocols (IP) address types enum.
#[derive(Debug, Clone, PartialEq)]
pub enum IpKind {
    Public,
    Private,
    Loopback,
    Linklocal,
    Apipa,
    Uniqelocal,
    Uniqeglobal,
    Broadcast,
    Netid,
    Multicast,
    Unspecified,
}

/// # IpAddress
/// `IpAddress` - Internet Protocol (IP) address (V4/V6) struct
#[derive(Debug, Clone, PartialEq)]
pub struct IpAddress {
    address: String,
    version: IpVersion,
    kind: IpKind,
}

/// # Mask
/// `Mask` - CIDR subnet mask struct
#[derive(Debug, Clone, PartialEq)]
pub struct Mask {
    mask: String,
    prefix: u8,
    num_of_hosts: u32,
}

/// # Network
/// `Network` - computer network struct
#[derive(Debug, Clone, PartialEq)]
pub struct Network {
    id: IpAddress,
    mask: Mask,
    broadcast: IpAddress,
}

impl IpVersion {
    /// Checks if a giving Ip address is an Ip version 4
    pub fn is_v4(address: &str) -> bool {
        match IpAddr::from_str(address) {
            Ok(addr) => IpAddr::is_ipv4(&addr),
            Err(..) => false,
        }
    }
    /// Checks if a giving address is an Ip version 6 address
    pub fn is_v6(address: &str) -> bool {
        match IpAddr::from_str(address) {
            Ok(addr) => IpAddr::is_ipv6(&addr),
            Err(..) => false,
        }
    }
}

impl Display for IpVersion {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            IpVersion::V4 => write!(f, "version 4"),
            IpVersion::V6 => write!(f, "version 6"),
        }
    }
}

impl IpKind {
    /// Check if a giving Ip address is a private address
    pub fn is_private(address: &str) -> bool {
        if IpVersion::is_v4(address) {
            match Ipv4Addr::from_str(address) {
                Ok(addr) => Ipv4Addr::is_private(&addr),
                Err(..) => false,
            }
        } else if IpVersion::is_v6(address) {
            if address.to_lowercase().starts_with("fd00")
                || address.to_lowercase().starts_with("fc00")
            {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    /// Check if a giving Ip address is a loopback address
    pub fn is_loopback(address: &str) -> bool {
        if IpVersion::is_v4(address) {
            match Ipv4Addr::from_str(address) {
                Ok(addr) => Ipv4Addr::is_loopback(&addr),
                Err(..) => false,
            }
        } else if IpVersion::is_v6(address) {
            match Ipv6Addr::from_str(address) {
                Ok(addr) => Ipv6Addr::is_loopback(&addr),
                Err(..) => false,
            }
        } else {
            false
        }
    }
    /// Check if a giving Ip v4 address is a broadcast address
    pub fn is_broadcast(address: &str, mask: &Mask) -> bool {
        if IpVersion::is_v4(address) {
            let octats = IpAddress::get_octets_from_str(address).unwrap();
            IpKind::is_netid(
                format!(
                    "{}.{}.{}.{}",
                    octats[0],
                    octats[1],
                    octats[2],
                    octats[3] as u32 - mask.num_of_hosts() - 1
                )
                .as_str(),
                mask,
            )
        } else {
            false
        }
    }
    /// Checks if a giving Ip v4 address is a net id based on giving subnet mask
    pub fn is_netid(address: &str, mask: &Mask) -> bool {
        if !IpVersion::is_v4(address) {
            return false;
        }
        let hosts = mask.num_of_hosts();
        let octets = IpAddress::get_octets_from_str(address).unwrap();
        octets[3] as u32 % hosts == 0
    }
    /// Check is a giving Ip address is a multicast address
    pub fn is_multicast(address: &str) -> bool {
        if IpVersion::is_v4(address) {
            match Ipv4Addr::from_str(address) {
                Ok(addr) => Ipv4Addr::is_multicast(&addr),
                Err(..) => false,
            }
        } else if IpVersion::is_v6(address) {
            match Ipv6Addr::from_str(address) {
                Ok(addr) => Ipv6Addr::is_multicast(&addr),
                Err(..) => false,
            }
        } else {
            false
        }
    }
    /// Check if a giving Ip address is a linklocal address
    pub fn is_linklocal(address: &str) -> bool {
        if IpVersion::is_v6(address) {
            if address[0..4].to_lowercase() == "fe80" {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    /// Check if a giving Ip address is an apipa address
    pub fn is_apipa(address: &str) -> bool {
        if IpVersion::is_v4(address) {
            match Ipv4Addr::from_str(address) {
                Ok(addr) => Ipv4Addr::is_link_local(&addr),
                Err(..) => false,
            }
        } else {
            false
        }
    }
    /// Check if a giving Ip address is a reserved address
    pub fn is_unspecified(address: &str) -> bool {
        if IpVersion::is_v4(address) {
            match Ipv4Addr::from_str(address) {
                Ok(addr) => Ipv4Addr::is_unspecified(&addr),
                Err(..) => false,
            }
        } else if IpVersion::is_v6(address) {
            match Ipv6Addr::from_str(address) {
                Ok(addr) => Ipv6Addr::is_unspecified(&addr),
                Err(..) => false,
            }
        } else {
            false
        }
    }
    /// Check if a giving Ip address is a public address
    pub fn is_public(address: &str) -> bool {
        if IpKind::is_loopback(address)
            || IpKind::is_private(address)
            || IpKind::is_apipa(address)
            || IpKind::is_multicast(address)
            || IpKind::is_linklocal(address)
            || IpKind::is_unspecified(address)
        {
            false
        } else {
            if IpVersion::is_v4(address) || IpVersion::is_v6(address) {
                true
            } else {
                false
            }
        }
    }
    /// Returns the kind of a giving address
    pub fn get_kind(address: &str) -> Result<IpKind, Box<dyn Error>> {
        if IpKind::is_private(address) {
            if IpVersion::is_v4(address) {
                return Ok(IpKind::Private);
            }
            Ok(IpKind::Uniqelocal)
        } else if IpKind::is_loopback(address) {
            Ok(IpKind::Loopback)
        } else if IpKind::is_multicast(address) {
            Ok(IpKind::Multicast)
        } else if IpKind::is_apipa(address) {
            Ok(IpKind::Apipa)
        } else if IpKind::is_linklocal(address) {
            Ok(IpKind::Linklocal)
        } else if IpKind::is_unspecified(address) {
            Ok(IpKind::Unspecified)
        } else if IpVersion::is_v4(address) {
            Ok(IpKind::Public)
        } else if IpVersion::is_v6(address) {
            Ok(IpKind::Uniqeglobal)
        } else {
            Err(Box::new(InvalidIpAddress))
        }
    }

    pub fn get_broadcast(netid: &str, mask: &Mask) -> Result<IpAddress, Box<dyn Error>> {
        if IpKind::is_netid(netid, mask) {
            let max_hosts = mask.num_of_hosts();
            let octats = IpAddress::get_octets_from_str(netid).unwrap();
            let mut addr: String;
            if *max_hosts < Mask::MAX_CLASS_C_ADDR as u32 {
                addr = format!(
                    "{}.{}.{}.{}",
                    octats[0],
                    octats[1],
                    octats[2],
                    max_hosts + octats[3] as u32 - 1
                );
            } else if *max_hosts < Mask::MAX_CLASS_B_ADDR {
                let preportion = (max_hosts) / Mask::MAX_CLASS_C_ADDR as u32;
                addr = format!(
                    "{}.{}.{}.{}",
                    octats[0],
                    octats[1],
                    octats[2] as u32 + preportion - 1,
                    255
                );
            } else if (*max_hosts as u64) < Mask::MAX_CLASS_A_ADDR {
                let preportion = (max_hosts) / Mask::MAX_CLASS_B_ADDR;
                addr = format!(
                    "{}.{}.{}.{}",
                    octats[0],
                    octats[1] as u32 + preportion - 1,
                    255,
                    255
                );
            } else {
                let preportion = ((*max_hosts) as u64) / Mask::MAX_CLASS_A_ADDR;
                addr = format!(
                    "{}.{}.{}.{}",
                    octats[0] as u64 + preportion - 1,
                    255,
                    255,
                    255
                );
            }
            return Ok(IpAddress {
                address: addr,
                version: IpVersion::V4,
                kind: IpKind::Broadcast,
            });
        }
        Err(Box::new(InvalidIpAddress))
    }
}

impl Display for IpKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            IpKind::Private => write!(f, "private"),
            IpKind::Public => write!(f, "public"),
            IpKind::Loopback => write!(f, "loopback"),
            IpKind::Apipa => write!(f, "apipa"),
            IpKind::Broadcast => write!(f, "broadcast"),
            IpKind::Linklocal => write!(f, "linklocal"),
            IpKind::Uniqelocal => write!(f, "uniqe local"),
            IpKind::Uniqeglobal => write!(f, "uniqe global"),
            IpKind::Multicast => write!(f, "multicast"),
            IpKind::Unspecified => write!(f, "unspecified"),
            IpKind::Netid => write!(f, "netid"),
        }
    }
}

impl IpAddress {
    const MAX_CLASS_C: u16 = 256;
    const MAX_CLASS_B: u32 = 65536;
    const MAX_CLASS_A: u64 = 4294967296;
    /// checks if a giving address is a valid ip address
    pub fn is_valid(address: &str) -> bool {
        if IpVersion::is_v4(address) || IpVersion::is_v6(address) {
            true
        } else {
            false
        }
    }
    /// creates a new IpAddress instance
    pub fn new(address: &str) -> Result<IpAddress, Box<dyn Error>> {
        if IpAddress::is_valid(address) {
            let addr;
            if IpVersion::is_v6(address) {
                addr = Self::shorten(address).unwrap();
            } else {
                addr = address.to_string();
            }
            return Ok(IpAddress {
                address: addr,
                version: if IpVersion::is_v4(address) {
                    IpVersion::V4
                } else {
                    IpVersion::V6
                },
                kind: IpKind::get_kind(address)?,
            });
        } else {
            Err(Box::new(InvalidIpAddress))
        }
    }
    /// creates a new IpAddress instance from IpAddr
    pub fn from(address: &IpAddr) -> IpAddress {
        match address {
            IpAddr::V4(ipv4) => {
                let oct = ipv4.octets();
                let address = format!("{}.{}.{}.{}", oct[0], oct[1], oct[2], oct[3]);
                IpAddress {
                    version: IpVersion::V4,
                    kind: IpKind::get_kind(address.as_str()).unwrap(),
                    address,
                }
            }
            IpAddr::V6(ipv6) => {
                let segm = ipv6.segments();
                let address = format!(
                    "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
                    segm[0], segm[1], segm[2], segm[3], segm[4], segm[5], segm[6], segm[7],
                );
                IpAddress {
                    version: IpVersion::V6,
                    kind: IpKind::get_kind(address.as_str()).unwrap(),
                    address,
                }
            }
        }
    }
    /// get the octats values of an ip address as u8 vector from giving &str
    pub fn get_octets_from_str(address: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        if IpVersion::is_v6(address) {
            let address = Self::expend(address)?;
            let parts = address
                .split(":")
                .map(|hex| u16::from_str_radix(hex, 16).unwrap())
                .collect::<Vec<u16>>();
            let parts: [u16; 8] = parts.try_into().unwrap();
            Ok(Ipv6Addr::from(parts).octets().to_vec())
        } else if IpVersion::is_v4(address) {
            let octets: Vec<u8> = address
                .split('.')
                .collect::<Vec<&str>>()
                .iter()
                .map(|oct| oct.parse::<u8>())
                .collect::<Result<Vec<u8>, _>>()
                .unwrap_or(vec![]);
            Ok(octets)
        } else {
            Err(Box::new(InvalidIpAddress))
        }
    }
    /// get the octats values of an ipv4 IpAddress instance
    pub fn get_octets(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        IpAddress::get_octets_from_str(self.address().as_str())
    }
    /// get the ipv6 address as expended
    pub fn get_expended(&self) -> String {
        Self::expend(&self.address).unwrap()
    }
    // getters for the IpAddress properties
    /// a getter function for the version propertie
    pub fn version(&self) -> &IpVersion {
        &self.version
    }
    /// a getter function for the address propertie
    pub fn address(&self) -> &String {
        &self.address
    }
    /// a getter function for the kind propertie
    pub fn kind(&self) -> &IpKind {
        &self.kind
    }
    /// implementation of the EUI-64 algorithem
    pub fn eui64(mac: &MacAddress) -> IpAddress {
        let parts = mac.as_vector();
        let address = format!(
            "fe80::{:x?}{}:{}ff:fe{}:{}{}",
            (i64::from_str_radix(parts[0].as_str(), 16).unwrap_or(0) as u8) ^ 0b0000_0010,
            parts[1],
            parts[2],
            parts[3],
            parts[4],
            parts[5]
        );
        IpAddress {
            version: IpVersion::V6,
            kind: IpKind::Linklocal,
            address: address.to_lowercase(),
        }
    }
    /// expends a giving ipv6 address
    pub fn expend(address: &str) -> Result<String, Box<dyn std::error::Error>> {
        if IpVersion::is_v6(address) {
            let mut exp_addr = String::new();
            let parts = address.split(":").collect::<Vec<&str>>();
            let addr_halfs = address.split("::").collect::<Vec<&str>>();
            let mut address = String::new();
            if addr_halfs.len() == 2 {
                let mut zero_parts = String::from(":");
                for _ in 0..(8 - parts.len() + 1) {
                    zero_parts.push_str("0000:");
                }
                address = addr_halfs[0].to_owned() + zero_parts.as_str() + addr_halfs[1];
            } else {
                address = addr_halfs[0].to_string();
            }
            let parts = address.split(":").collect::<Vec<&str>>();
            let length = parts.len();
            for (i, part) in parts.into_iter().enumerate() {
                if part.len() != 4 {
                    exp_addr = exp_addr + "0".repeat(4 - part.len()).as_str();
                }
                exp_addr.push_str(part);
                if i + 1 != length {
                    exp_addr.push_str(":")
                };
            }
            Ok(exp_addr)
        } else {
            Err(Box::new(InvalidIpV6Address {}))
        }
    }
    /// shorten a giving ipv6 address
    pub fn shorten(address: &str) -> Result<String, Box<dyn std::error::Error>> {
        if IpVersion::is_v6(address) {
            let octets = Self::get_octets_from_str(&address)?;
            let octets: [u8; 16] = octets.try_into().unwrap();
            let octets = Ipv6Addr::from(octets).segments();
            let mut max_zeros: usize = 0;
            let mut max_zeros_index = 0;
            let mut curr_zeros = 0;
            let mut curr_zeros_index = 0;
            let mut is_leading = true;
            let mut shorten_addr = String::new();
            for (i, oct) in octets.clone().into_iter().enumerate() {
                if oct == 0 {
                    if is_leading {
                        is_leading = false;
                        curr_zeros_index = i;
                    }
                    curr_zeros += 1;
                } else {
                    is_leading = true;
                    if curr_zeros > max_zeros {
                        max_zeros = curr_zeros;
                        max_zeros_index = curr_zeros_index;
                    }
                    curr_zeros = 0;
                }
            }
            for i in 0..((octets.len()) as usize) {
                if i >= max_zeros_index && i < max_zeros_index + max_zeros {
                    continue;
                }
                shorten_addr = format!("{}{:x}", shorten_addr, octets[i]);
                if i != octets.len() - 1 {
                    if i == max_zeros_index - 1 {
                        shorten_addr.push_str("::");
                    } else {
                        shorten_addr.push_str(":");
                    }
                }
            }
            Ok(shorten_addr)
        } else {
            Err(Box::new(InvalidIpV6Address))
        }
    }
}

impl Display for IpAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} is a {} {} ip address",
            self.address(),
            self.version(),
            self.kind()
        )
    }
}

impl Mask {
    const MAX_CLASS_A_ADDR: u64 = 16777216;
    const MAX_CLASS_B_ADDR: u32 = 65536;
    const MAX_CLASS_C_ADDR: u16 = 256;

    /// Checks if a giving Subnet Mask is valid
    pub fn is_valid(mask: &str) -> bool {
        let octats_values: Vec<u8> = IpAddress::get_octets_from_str(mask).unwrap_or(vec![]);
        if octats_values.len() == 0 {
            return false;
        }
        let mask_value = (octats_values[0] as u32) << 24
            | (octats_values[1] as u32) << 16
            | (octats_values[2] as u32) << 8
            | octats_values[3] as u32;
        let ones_count = mask_value.leading_ones();
        u32::MAX << 32 - ones_count == mask_value as u32
    }
    /// returns the prefix of a giving address
    pub fn get_prefix(mask: &str) -> Result<u8, Box<dyn Error>> {
        if Mask::is_valid(mask) {
            let octets: Vec<&str> = mask.split('.').collect();
            let octets_values: Vec<u8> = octets
                .iter()
                .map(|oct| oct.parse::<u8>())
                .collect::<Result<Vec<u8>, _>>()
                .unwrap();
            let mask_value = (octets_values[0] as u32) << 24
                | (octets_values[1] as u32) << 16
                | (octets_values[2] as u32) << 8
                | octets_values[3] as u32;
            return Ok(mask_value.leading_ones() as u8);
        }
        Err(Box::new(InvalidMask))
    }
    /// creates a new Mask instance
    pub fn new(mask: &str) -> Result<Mask, Box<dyn Error>> {
        let prefix = Mask::get_prefix(mask).unwrap();
        if Mask::is_valid(mask) {
            return Ok(Mask {
                mask: mask.to_string(),
                prefix,
                num_of_hosts: (2 as u32).pow(32 - prefix as u32),
            });
        }
        Err(Box::new(InvalidMask))
    }
    /// creates new Mask instance from giving prefix
    pub fn from_prefix(prefix: u8) -> Result<Mask, Box<dyn Error>> {
        if prefix > 32 {
            return Err(Box::new(InvalidPrefix));
        }
        let full_bytes: u32 = u32::MAX;
        let mask_bytes = full_bytes >> 32 - prefix;
        let mut octats = vec![];
        octats.push(mask_bytes.to_ne_bytes()[0]);
        octats.push(mask_bytes.to_ne_bytes()[1]);
        octats.push(mask_bytes.to_ne_bytes()[2]);
        octats.push(mask_bytes.to_ne_bytes()[3]);
        Ok(Mask {
            mask: format!(
                "{}.{}.{}.{}",
                octats[0],
                octats[1],
                octats[2],
                if octats[3] == 0 {
                    octats[3]
                } else {
                    255 - octats[3]
                }
            ),
            prefix,
            num_of_hosts: 2u32.pow(32 - prefix as u32),
        })
    }

    pub fn mask(&self) -> &String {
        &self.mask
    }

    pub fn prefix(&self) -> &u8 {
        &self.prefix
    }

    pub fn num_of_hosts(&self) -> &u32 {
        &self.num_of_hosts
    }
}

impl Display for Mask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.mask())
    }
}

impl Network {
    /// Creates a new ipv4 Network instance from giving net id and subnet mask
    pub fn new(id: IpAddress, mask: Mask) -> Result<Network, Box<dyn Error>> {
        if IpKind::is_netid(id.address().as_str(), &mask) {
            let octats = IpAddress::get_octets(&id).unwrap();
            let hosts = mask.num_of_hosts() + 1;
            if IpVersion::is_v4(&id.address().as_str()) {
                return Ok(Network {
                    id,
                    mask,
                    broadcast: IpAddress::new(
                        format!(
                            "{}.{}.{}.{}",
                            octats[0],
                            octats[1],
                            octats[2],
                            octats[3] as u32 + hosts - 2
                        )
                        .as_str(),
                    )
                    .unwrap(),
                });
            }
        }
        Err(Box::new(InvalidNetwork))
    }
    /// Creates a new network from string slice in the "netid/prefix" format
    pub fn from_str(net: &str) -> Result<Network, Box<dyn Error>> {
        let networks_items = net.split('/').collect::<Vec<&str>>();
        if networks_items.len() != 2 {
            return Err(Box::new(InvalidNetwork));
        }
        let prefix = networks_items[1].parse::<u8>().unwrap_or(0);
        println!("{}", prefix);
        if prefix == 0 {
            return Err(Box::new(InvalidNetwork));
        }
        let mask = Mask::from_prefix(prefix);
        if let Ok(mask) = mask {
            if IpKind::is_netid(networks_items[0], &mask) {
                let netid = IpAddress::new(networks_items[0])?;
                return Ok(Network {
                    id: netid.clone(),
                    broadcast: IpKind::get_broadcast(netid.address().as_str(), &mask)?,
                    mask,
                });
            } else {
                return Err(Box::new(InvalidNetwork));
            }
        } else {
            return Err(Box::new(InvalidNetwork));
        }
    }
    /// Checks if a giving Ip address is in the self network
    pub fn contains(&self, address: &IpAddress) -> bool {
        if IpVersion::is_v4(address.address().as_str()) {
            let octats = address.get_octets().unwrap();
            let netid_octs = self.id.get_octets().unwrap();
            let bcast_octs = self.broadcast.get_octets().unwrap();
            let prefix = self.mask.prefix();
            if *prefix >= 24 {
                octats[3] > netid_octs[3] && octats[3] < bcast_octs[3]
            } else if *prefix >= 16 {
                octats[2] <= bcast_octs[2] && octats[3] > netid_octs[3] && octats[3] < bcast_octs[3]
            } else if *prefix >= 8 {
                octats[1] <= bcast_octs[1] && octats[3] > netid_octs[3] && octats[3] < bcast_octs[3]
            } else {
                octats[0] <= bcast_octs[0] && octats[3] > netid_octs[3] && octats[3] < bcast_octs[3]
            }
        } else {
            false
        }
    }
    /// getter for the broadcast property
    pub fn broadcast(&self) -> &IpAddress {
        &self.broadcast
    }
    /// getter for the netid property
    pub fn netid(&self) -> &IpAddress {
        &self.id
    }
    /// getter for the mask property
    pub fn mask(&self) -> &Mask {
        &self.mask
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.netid().address(), self.mask().prefix())
    }
}

/// # Interface
/// `Interface` - network interface of the local machine
#[derive(Debug, Clone, PartialEq)]
pub struct Interface {
    name: String,
    index: u32,
    description: String,
    mac: Option<MacAddress>,
    ipv4: Option<IpAddress>,
    ipv6: Option<IpAddress>,
    mask: Option<Mask>,
}

impl Interface {
    /// Returns all the network interfaces on the local machine
    pub fn all() -> Vec<Interface> {
        let mut infs = vec![];
        for inf in interfaces() {
            match Self::by_index(inf.index) {
                Ok(res) => infs.push(res),
                Err(_) => continue,
            }
        }
        infs
    }
    /// Returns local network interface by index
    pub fn by_index(index: u32) -> Result<Interface, Box<dyn Error>> {
        let mut mac: Option<MacAddress> = None;
        let mut ipv4: Option<IpAddress> = None;
        let mut ipv6: Option<IpAddress> = None;
        let mut mask: Option<Mask> = None;
        for inf in interfaces() {
            if inf.index == index {
                if inf.ips.len() > 0 {
                    if let IpNetwork::V4(addr) = inf.ips[0] {
                        ipv4 = Some(IpAddress::from(&IpAddr::V4(addr.ip())));
                        if let Ok(submask) = Mask::from_prefix(addr.prefix()) {
                            mask = Some(submask);
                        }
                    }
                }
                if inf.ips.len() > 1 {
                    if let IpNetwork::V6(addr) = inf.ips[1] {
                        ipv6 = Some(IpAddress::from(&IpAddr::V6(addr.ip())))
                    }
                }
                if let Some(mac_addr) = inf.mac {
                    let mac_addr = mac_addr.octets();
                    match MacAddress::new(
                        format!(
                            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                            mac_addr[0],
                            mac_addr[1],
                            mac_addr[2],
                            mac_addr[3],
                            mac_addr[4],
                            mac_addr[5]
                        )
                        .as_str(),
                    ) {
                        Ok(addr) => {
                            mac = Some(addr);
                        }
                        Err(_) => {
                            mac = None;
                        }
                    }
                }
                return Ok(Interface {
                    name: inf.name.to_string(),
                    index,
                    description: inf.description,
                    mac,
                    ipv4,
                    ipv6,
                    mask,
                });
            }
        }
        Err(Box::new(InterfaceNotExists))
    }
    /// Returns local network interface by name
    pub fn by_name(name: &str) -> Result<Interface, Box<dyn Error>> {
        for inf in interfaces() {
            if inf.name == name {
                return Self::by_index(inf.index);
            }
        }
        Err(Box::new(InterfaceNotExists))
    }
    /// get interface name attribute
    pub fn name(&self) -> &String {
        &self.name
    }
    /// get interface index attribute
    pub fn index(&self) -> &u32 {
        &self.index
    }
    /// get interface description attribute
    pub fn description(&self) -> &String {
        &self.description
    }
    /// get mac address attribute
    pub fn mac(&self) -> &Option<MacAddress> {
        &self.mac
    }
    /// get ipv4 address attribute
    pub fn ipv4(&self) -> &Option<IpAddress> {
        &self.ipv4
    }
    /// get ipv6 address attribute
    pub fn ipv6(&self) -> &Option<IpAddress> {
        &self.ipv6
    }
    /// get ipv4 subnet mask attribute
    pub fn mask(&self) -> &Option<Mask> {
        &self.mask
    }
    /// convert Interface instance to NetworkInterface instance
    pub fn into(&self) -> Result<NetworkInterface, Box<dyn Error>> {
        for inf in interfaces() {
            if self.name == inf.name {
                return Ok(inf);
            }
        }
        Err(Box::new(InterfaceConvertionFailed))
    }
}

impl Display for Interface {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut mac = "None";
        let mut ipv4 = "None";
        let mut ipv6 = "None";
        let mut mask = "None";
        if let Some(addr) = &self.mac {
            mac = addr.address();
        }
        if let Some(addr) = &self.ipv4 {
            ipv4 = addr.address()
        }
        if let Some(addr) = &self.ipv6 {
            ipv6 = addr.address()
        }
        if let Some(addr) = &self.mask {
            mask = addr.mask()
        }

        write!(
            f,
            "==== {} ====
index: {}
description: {}
mac: {}
ipv4: {}                
ipv6: {}
mask: {}",
            self.name, self.index, self.description, mac, ipv4, ipv6, mask
        )
    }
}
