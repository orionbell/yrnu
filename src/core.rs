//! # address.rs
//! The `address` module provides general purpose tools for handling and managing Ip and Mac
//! addresses as well as defining networks and getting local network interfaces info.
use crate::error::coreerr::*;
use core::fmt;
use mlua::FromLua;
use pnet::datalink::NetworkInterface;
use pnet::{datalink::interfaces, ipnetwork::IpNetwork};
use std::{
    fmt::{Display, Formatter},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, ToSocketAddrs},
    path::PathBuf,
    str::FromStr,
};

/// # MacAddress
/// `MacAddress` - MAC address struct
#[derive(Debug, Clone, PartialEq, FromLua)]
pub struct MacAddress {
    bytes: [u8; 6],
    vendor: String,
}
/// # IpVersion
/// `IpVersion` - Internet Protocol (IP) versions enum.
#[derive(Debug, Clone, PartialEq, FromLua)]
pub enum IpVersion {
    V4,
    V6,
}

/// # IpKind
/// `IpKind` - Internet Protocols (IP) address types enum.
#[derive(Debug, Clone, PartialEq, FromLua)]
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
#[derive(Debug, Clone, PartialEq, FromLua)]
pub struct IpAddress {
    address: Vec<u8>,
    version: IpVersion,
    kind: IpKind,
}

/// # Mask
/// `Mask` - CIDR subnet mask struct
#[derive(Debug, Clone, PartialEq, FromLua)]
pub struct Mask {
    prefix: u8,
    num_of_hosts: u32,
}

/// # Network
/// `Network` - computer network struct
#[derive(Debug, Clone, PartialEq, FromLua)]
pub struct Network {
    id: IpAddress,
    mask: Mask,
    broadcast: IpAddress,
}

impl MacAddress {
    /// Returns the giving mac address vendor
    fn get_vendor(address: &str) -> Result<String, InvalidMacAddress> {
        let index = rsmanuf::Index::new();
        if Self::is_valid(address) {
            Ok(match index.search(address) {
                Ok(manuf) => manuf,
                Err(_) => String::from("Unknown"),
            })
        } else {
            Err(InvalidMacAddress)
        }
    }
    /// Checks if a giving `mac address` is valid
    pub fn is_valid(address: &str) -> bool {
        Self::get_parts(address).is_ok()
    }
    /// returns the `mac address` as a vector
    pub fn as_bytes(&self) -> &[u8; 6] {
        &self.bytes
    }
    /// Converts string mac address into byte array
    fn get_parts(address: &str) -> Result<[u8; 6], InvalidMacAddress> {
        let parts: Vec<Result<u8, _>> = address
            .split(':')
            .map(|i| u8::from_str_radix(i, 16))
            .collect();
        if parts.len() != 6 {
            return Err(InvalidMacAddress);
        }
        let mut octets: [u8; 6] = [0u8; 6];
        for (i, part) in parts.iter().enumerate() {
            if let Ok(part) = part {
                octets[i] = *part;
            } else {
                return Err(InvalidMacAddress);
            }
        }
        Ok(octets)
    }
    /// Creates a new MacAddress instance
    pub fn new(bytes: [u8; 6]) -> MacAddress {
        MacAddress {
            bytes,
            vendor: Self::get_vendor(&format!(
                "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
            ))
            .unwrap(),
        }
    }
    /// Returns the address as a string
    pub fn address(&self) -> String {
        format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5],
        )
    }
    pub fn cisco_format(&self) -> String {
        format!(
            "{:02X}{:02X}.{:02X}{:02X}.{:02X}{:02X}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5],
        )
    }
    pub fn vendor(&self) -> &String {
        &self.vendor
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.address())
    }
}

impl PartialOrd for MacAddress {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_vec = self.bytes.to_vec();
        let other_vec = other.as_bytes().to_vec();
        let mut self_hex;
        let mut other_hex;
        let mut index = 5;
        loop {
            self_hex = self_vec.get(index).unwrap();
            other_hex = other_vec.get(index).unwrap();
            if self_hex != other_hex {
                return Some(self_hex.cmp(other_hex));
            }
            if index == 0 {
                return Some(std::cmp::Ordering::Equal);
            }
            index -= 1;
        }
    }
}

impl FromStr for MacAddress {
    type Err = InvalidMacAddress;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if MacAddress::is_valid(s) {
            return Ok(MacAddress {
                bytes: Self::get_parts(s).unwrap(),
                vendor: Self::get_vendor(s).unwrap(),
            });
        }
        Err(InvalidMacAddress)
    }
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
    /// Check if a giving Ipv4 address is a broadcast address
    pub fn is_broadcast(address: &str, mask: &Mask) -> bool {
        if IpVersion::is_v4(address) {
            let octats = IpAddress::octets_from_str(address).unwrap();
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
    /// Checks if a giving Ipv4 address is a net id based on giving subnet mask
    pub fn is_netid(address: &str, mask: &Mask) -> bool {
        if !IpVersion::is_v4(address) {
            return false;
        }
        let hosts = mask.num_of_hosts();
        let octets = IpAddress::octets_from_str(address).unwrap();
        if *hosts == u32::MAX {
            return true;
        }
        octets[3] as u32 % (hosts + 2) == 0
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
    pub fn get_kind(address: &str) -> Result<IpKind, InvalidIpAddress> {
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
            Err(InvalidIpAddress)
        }
    }
    pub fn get_broadcast(netid: &str, mask: &Mask) -> Result<IpAddress, InvalidIpAddress> {
        if IpKind::is_netid(netid, mask) {
            let max_hosts = mask.num_of_hosts();
            let octats = IpAddress::octets_from_str(netid).unwrap();
            let addr;
            if *max_hosts < Mask::MAX_CLASS_C_ADDR as u32 {
                addr = vec![
                    octats[0],
                    octats[1],
                    octats[2],
                    (max_hosts + octats[3] as u32 + 1) as u8,
                ];
            } else if *max_hosts < Mask::MAX_CLASS_B_ADDR {
                let preportion = (max_hosts) / Mask::MAX_CLASS_C_ADDR as u32;
                addr = vec![
                    octats[0],
                    octats[1],
                    (octats[2] as u32 + preportion) as u8,
                    255,
                ];
            } else if (*max_hosts as u64) < Mask::MAX_CLASS_A_ADDR {
                let preportion = (max_hosts) / Mask::MAX_CLASS_B_ADDR;
                addr = vec![
                    octats[0],
                    (octats[1] as u32 + preportion) as u8,
                    255,
                    255,
                ];
            } else {
                let preportion = ((*max_hosts) as u64) / Mask::MAX_CLASS_A_ADDR;
                addr = vec![(octats[0] as u64 + preportion) as u8, 255, 255, 255];
            }
            return Ok(IpAddress {
                address: addr,
                version: IpVersion::V4,
                kind: IpKind::Broadcast,
            });
        }
        Err(InvalidIpAddress)
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
    pub const MAX_CLASS_C: u16 = 256;
    pub const MAX_CLASS_B: u32 = 65536;
    pub const MAX_CLASS_A: u64 = 4294967296;
    /// Checks if a giving address is a valid ip address
    pub fn is_valid(address: &str) -> bool {
        IpVersion::is_v4(address) || IpVersion::is_v6(address)
    }
    /// Creates a new IpAddress instance
    pub fn new(octets: &Vec<u8>) -> Result<IpAddress, InvalidIpAddress> {
        if octets.len() != 6 && octets.len() != 16 {
            return Err(InvalidIpAddress);
        }
        Ok(IpAddress {
            address: octets.clone(),
            version: if octets.len() == 4 {
                IpVersion::V4
            } else {
                IpVersion::V6
            },
            kind: if octets.len() == 4 {
                IpKind::get_kind(
                    &octets
                        .iter()
                        .map(|oct| oct.to_string())
                        .collect::<Vec<String>>()
                        .join("."),
                )
                .unwrap()
            } else {
                let mut address = String::new();
                let mut i = 0;
                while i < octets.len() - 1 {
                    address = format!("{address}{}{}:", octets[i], octets[i + 1]);
                    i += 2;
                }
                address.pop();
                IpKind::get_kind(&address).unwrap()
            },
        })
    }
    /// Creates a new IpAddress instance from IpAddr
    pub fn from(address: &IpAddr) -> IpAddress {
        match address {
            IpAddr::V4(ipv4) => {
                let addr = ipv4
                    .octets()
                    .iter()
                    .map(|oct| oct.to_string())
                    .collect::<Vec<String>>()
                    .join(".");
                IpAddress {
                    version: IpVersion::V4,
                    kind: IpKind::get_kind(&addr).unwrap(),
                    address: ipv4.octets().to_vec(),
                }
            }
            IpAddr::V6(ipv6) => {
                let address = ipv6
                    .segments()
                    .iter()
                    .map(|seg| format!("{:x}", seg))
                    .collect::<Vec<String>>()
                    .join(":");
                IpAddress {
                    version: IpVersion::V6,
                    kind: IpKind::get_kind(&address).unwrap(),
                    address: ipv6.octets().to_vec(),
                }
            }
        }
    }
    /// Creates a net IpAddress instance for each address in a giving domain
    pub fn from_domain(domain: &str) -> Vec<IpAddress> {
        (domain, 0)
            .to_socket_addrs()
            .unwrap_or_default()
            .map(|v| Self::from(&v.ip()))
            .collect::<Vec<IpAddress>>()
    }
    /// Get the octats values of an ip address as u8 vector from giving &str
    pub fn octets_from_str(address: &str) -> Result<Vec<u8>, InvalidIpAddress> {
        if IpVersion::is_v6(address) {
            let addr: Ipv6Addr = address.parse().unwrap();
            Ok(addr.octets().to_vec())
        } else if IpVersion::is_v4(address) {
            let octets: Ipv4Addr = address.parse().unwrap();
            Ok(octets.octets().to_vec())
        } else {
            Err(InvalidIpAddress)
        }
    }
    /// Get the octets values of an ipv4 IpAddress instance
    pub fn octets(&self) -> &Vec<u8> {
        &self.address
    }
    /// Get the ipv6 address as expended
    pub fn get_expended(&self) -> Result<String, InvalidIpV6Address> {
        Self::expend(&self.address())
    }
    // Getters for the IpAddress properties
    /// Getter function for the version property
    pub fn version(&self) -> &IpVersion {
        &self.version
    }
    /// Getter function for the address property
    pub fn address(&self) -> String {
        self.to_string()
    }
    /// Getter function for the kind property
    pub fn kind(&self) -> &IpKind {
        &self.kind
    }
    /// Implementation of the EUI-64 algorithm
    pub fn eui64(mac: &MacAddress) -> IpAddress {
        let parts = mac.bytes;
        let address = vec![
            0xfe,
            0x80,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            parts[0] ^ 0b0000_0010,
            parts[1],
            parts[2],
            0xff,
            0xfe,
            parts[3],
            parts[4],
            parts[5],
        ];
        IpAddress {
            version: IpVersion::V6,
            kind: IpKind::Linklocal,
            address,
        }
    }
    /// Expends a giving ipv6 address
    pub fn expend(address: &str) -> Result<String, InvalidIpV6Address> {
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
            Err(InvalidIpV6Address)
        }
    }
    /// Shorten a giving ipv6 address
    pub fn shorten(address: &str) -> Result<String, InvalidIpV6Address> {
        if IpVersion::is_v6(address) {
            let octets = address.parse::<Ipv6Addr>().unwrap().segments();
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
            Err(InvalidIpV6Address)
        }
    }
}

impl Display for IpAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let address = match self.version {
            IpVersion::V4 => format!(
                "{}",
                self.address
                    .iter()
                    .map(|oct| oct.to_string())
                    .collect::<Vec<String>>()
                    .join(".")
            ),
            IpVersion::V6 => format!(
                "{}",
                IpAddress::shorten(
                    &self
                        .address
                        .chunks(2)
                        .map(|chunk| (format!(
                            "{:x}",
                            (chunk[1] as u16) | ((chunk[0] as u16) << 8)
                        )))
                        .collect::<Vec<String>>()
                        .join(":")
                )
                .unwrap()
            ),
        };
        write!(f, "{address}",)
    }
}

impl FromStr for IpAddress {
    type Err = InvalidIpAddress;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if IpAddress::is_valid(s) {
            let addr = IpAddress::octets_from_str(s)?;
            return Ok(IpAddress {
                address: addr,
                version: if IpVersion::is_v4(s) {
                    IpVersion::V4
                } else {
                    IpVersion::V6
                },
                kind: IpKind::get_kind(s)?,
            });
        } else {
            Err(InvalidIpAddress)
        }
    }
}

impl Mask {
    const MAX_CLASS_A_ADDR: u64 = 16777216;
    const MAX_CLASS_B_ADDR: u32 = 65536;
    const MAX_CLASS_C_ADDR: u16 = 256;

    /// Checks if a giving Subnet Mask is valid
    pub fn is_valid(mask: &str) -> bool {
        let octats_values: Vec<u8> = IpAddress::octets_from_str(mask).unwrap_or(vec![]);
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
    /// Returns the prefix of a giving address
    pub fn get_prefix(octets_values: &Vec<u8>) -> u8 {
        let mask_value = (octets_values[0] as u32) << 24
            | (octets_values[1] as u32) << 16
            | (octets_values[2] as u32) << 8
            | octets_values[3] as u32;
        mask_value.leading_ones() as u8
    }
    /// Creates a new Mask instance
    pub fn new(bytes: &Vec<u8>) -> Result<Mask, InvalidMask> {
        if bytes.len() == 4 || bytes.len() == 16 {
            let prefix = Mask::get_prefix(bytes);
            Ok(Mask {
                prefix,
                num_of_hosts: (2 as u32).pow(32 - prefix as u32) - 2,
            })
        } else {
            Err(InvalidMask)
        }
    }
    /// Creates new Mask instance from giving prefix
    pub fn from_prefix(prefix: u8) -> Result<Mask, InvalidPrefix> {
        if prefix > 32 {
            return Err(InvalidPrefix);
        }
        Ok(Mask {
            prefix,
            num_of_hosts: if prefix == 0 {
                u32::MAX
            } else {
                2u32.pow(32 - prefix as u32) - 2
            },
        })
    }

    pub fn mask(&self) -> String {
        let full_bytes: u32 = u32::MAX;
        let mask_bytes = if self.prefix == 0 {
            0
        } else {
            full_bytes << 32 - self.prefix
        };
        let octats = mask_bytes.to_ne_bytes();
        format!("{}.{}.{}.{}", octats[3], octats[2], octats[1], octats[0])
    }

    pub fn wildcard(&self) -> String {
        let full_bytes: u32 = u32::MAX;
        let mask_bytes = if self.prefix == 0 {
            0
        } else {
            full_bytes << 32 - self.prefix
        };
        let octats = mask_bytes.to_ne_bytes();
        format!(
            "{}.{}.{}.{}",
            255 - octats[3],
            255 - octats[2],
            255 - octats[1],
            255 - octats[0]
        )
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

impl FromStr for Mask {
    type Err = InvalidMask;
    fn from_str(mask: &str) -> Result<Self, Self::Err> {
        if Mask::is_valid(mask) {
            let prefix = Mask::get_prefix(&IpAddress::octets_from_str(mask).unwrap());
            return Ok(Mask {
                prefix,
                num_of_hosts: (2 as u32).pow(32 - prefix as u32) - 2,
            });
        }
        Err(InvalidMask)
    }
}

impl Network {
    /// Creates a new ipv4 Network instance from giving net id and subnet mask
    pub fn new(id: IpAddress, mask: Mask) -> Result<Network, InvalidNetwork> {
        if IpKind::is_netid(id.address().as_str(), &mask) {
            let octets = id.octets();
            let hosts = mask.num_of_hosts() + 1;
            if IpVersion::is_v4(&id.address().as_str()) {
                return Ok(Network {
                    mask,
                    broadcast: IpAddress::from_str(
                        format!(
                            "{}.{}.{}.{}",
                            octets[0],
                            octets[1],
                            octets[2],
                            octets[3] as u32 + hosts - 2
                        )
                        .as_str(),
                    )
                    .unwrap(),
                    id,
                });
            }
        }
        Err(InvalidNetwork)
    }
    /// Checks if a giving Ip address is in the self network
    pub fn contains(&self, address: &IpAddress) -> bool {
        if IpVersion::is_v4(address.address().as_str()) {
            let octats = address.octets();
            let netid_octs = self.id.octets();
            let bcast_octs = self.broadcast.octets();
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

impl FromStr for Network {
    type Err = InvalidNetwork;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let network_items = s.split('/').collect::<Vec<&str>>();
        if network_items.len() != 2 {
            return Err(InvalidNetwork);
        }
        let prefix = network_items[1].parse::<u8>().unwrap_or(255);
        if prefix == 255 {
            return Err(InvalidNetwork);
        }
        let mask = Mask::from_prefix(prefix);
        println!("{prefix}");
        if let Ok(mask) = mask {
            if IpKind::is_netid(network_items[0], &mask) {
                let netid = IpAddress::from_str(network_items[0]).unwrap();
                return Ok(Network {
                    id: netid.clone(),
                    broadcast: IpKind::get_broadcast(netid.address().as_str(), &mask).unwrap(),
                    mask,
                });
            } else {
                return Err(InvalidNetwork);
            }
        } else {
            return Err(InvalidNetwork);
        }
    }
}

/// # Interface
/// `Interface` - network interface of the local machine
#[derive(Debug, Clone, PartialEq, FromLua)]
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
    pub fn by_index(index: u32) -> Result<Interface, InterfaceNotExists> {
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
                    mac = Some(MacAddress::new(mac_addr.octets()));
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
        Err(InterfaceNotExists)
    }
    /// Returns local network interface by name
    pub fn by_name(name: &str) -> Result<Interface, InterfaceNotExists> {
        for inf in interfaces() {
            if inf.name == name {
                return Self::by_index(inf.index);
            }
        }
        Err(InterfaceNotExists)
    }
    /// Get interface name attribute
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Get interface index attribute
    pub fn index(&self) -> &u32 {
        &self.index
    }
    /// Get interface description attribute
    pub fn description(&self) -> &String {
        &self.description
    }
    /// Get mac address attribute
    pub fn mac(&self) -> &Option<MacAddress> {
        &self.mac
    }
    /// Get ipv4 address attribute
    pub fn ipv4(&self) -> &Option<IpAddress> {
        &self.ipv4
    }
    /// Get ipv6 address attribute
    pub fn ipv6(&self) -> &Option<IpAddress> {
        &self.ipv6
    }
    /// Get ipv4 subnet mask attribute
    pub fn mask(&self) -> &Option<Mask> {
        &self.mask
    }
    /// Convert Interface instance to NetworkInterface instance
    pub fn into(&self) -> Result<NetworkInterface, InterfaceConvertionFailed> {
        for inf in interfaces() {
            if self.name == inf.name {
                return Ok(inf);
            }
        }
        Err(InterfaceConvertionFailed)
    }
}

impl Display for Interface {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut mac = "None".to_string();
        let mut ipv4 = "None".to_string();
        let mut ipv6 = "None".to_string();
        let mut mask = "None".to_string();
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
mac:  {}
ipv4: {}                
ipv6: {}
mask: {}",
            self.name, self.index, self.description, mac, ipv4, ipv6, mask
        )
    }
}

impl FromStr for Interface {
    type Err = InterfaceNotExists;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(index) = s.parse::<u32>() {
            Self::by_index(index)
        } else {
            Self::by_name(s)
        }
    }
}
#[derive(Debug, Clone, PartialEq, FromLua)]
pub struct Path(pub PathBuf);

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl FromStr for Path {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Path(PathBuf::from(s)))
    }
}

#[derive(Debug, Clone, PartialEq, FromLua)]
pub struct Url(pub url::Url);

impl Display for Url {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Url {
    type Err = url::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match url::Url::parse(s) {
            Ok(url) => Ok(Url(url)),
            Err(e) => Err(e),
        }
    }
}
