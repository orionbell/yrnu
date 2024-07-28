//! # address.rs
//! The `address` module provides general purpese tools for heandling and managing Ip and Mac
//! addresses as well as defining networks.

use core::fmt;
use std::{
    env::remove_var, fmt::{Display, Formatter}, net::{IpAddr, Ipv4Addr, Ipv6Addr}, str::FromStr
};

/// # IpVersion
/// `IpVersion` is an enum that present the two versions of Internet Protocol (IP) versions.
pub enum IpVersion {
    V4,
    V6,
}

/// # IpKind
/// `IpKind` is an enum that present the diffrent kinds of Internet Protocols (IP) addresses.
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
/// `IpAddress` is a struct that present an Internet Protocol (IP) address
pub struct IpAddress {
    address: String,
    version: IpVersion,
    kind: IpKind,
}

/// # Mask
/// `Mask` is a struct that present a CIDR subnet mask
pub struct Mask {
    mask: String,
    prefix: u8,
    num_of_hosts: u32,
}

/// # Network
/// `Network` is a struct that present computer network
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

    pub fn clone(&self) -> Self {
        match self {
            IpVersion::V4 => IpVersion::V4,
            IpVersion::V6 => IpVersion::V6,
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
            println!("{}", address);
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
            let octats = IpAddress::get_octats_from_str(address).unwrap();
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
        let hosts = mask.num_of_hosts() + 2;
        let octats = IpAddress::get_octats_from_str(address).unwrap();
        octats[3] as u32 % hosts == 0
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
            if address[0..3].to_lowercase() == "fe80" {
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
    pub fn get_kind(address: &str) -> Option<IpKind> {
        if IpKind::is_private(address) {
            if IpVersion::is_v4(address) {
                return Some(IpKind::Private);
            }
            Some(IpKind::Uniqelocal)
        } else if IpKind::is_loopback(address) {
            Some(IpKind::Loopback)
        } else if IpKind::is_multicast(address) {
            Some(IpKind::Multicast)
        } else if IpKind::is_apipa(address) {
            Some(IpKind::Apipa)
        } else if IpKind::is_linklocal(address) {
            Some(IpKind::Linklocal)
        } else if IpKind::is_unspecified(address) {
            Some(IpKind::Unspecified)
        } else if IpVersion::is_v4(address) {
            Some(IpKind::Public)
        } else if IpVersion::is_v6(address) {
            Some(IpKind::Uniqeglobal)
        } else {
            None
        }
    }

    pub fn get_broadcast(netid: &str, mask: &Mask) -> Option<IpAddress> {
        if IpKind::is_netid(netid, mask) {
            let max_hosts = mask.num_of_hosts();
            let octats = IpAddress::get_octats_from_str(netid).unwrap();
            let mut addr = String::new();
            if max_hosts < Mask::MAX_CLASS_C_ADDR as u32 {
                addr = format!(
                    "{}.{}.{}.{}",
                    octats[0],
                    octats[1],
                    octats[2],
                    max_hosts + octats[3] as u32 + 1
                );
            } else if max_hosts < Mask::MAX_CLASS_B_ADDR {
                let preportion = (max_hosts + 2) / Mask::MAX_CLASS_C_ADDR as u32;
                addr = format!(
                    "{}.{}.{}.{}",
                    octats[0],
                    octats[1],
                    octats[2] as u32 + preportion - 1,
                    255
                );
            } else if (max_hosts as u64) < Mask::MAX_CLASS_A_ADDR {
                let preportion = (max_hosts + 2) / Mask::MAX_CLASS_B_ADDR;
                addr = format!(
                    "{}.{}.{}.{}",
                    octats[0],
                    octats[1] as u32 + preportion - 1,
                    255,
                    255
                );
            } else {
                let preportion = ((max_hosts + 2) as u64) / Mask::MAX_CLASS_A_ADDR; 
                addr = format!(
                    "{}.{}.{}.{}",
                    octats[0] as u64 + preportion - 1,
                    255,
                    255,
                    255
                );
            }
            return Some(IpAddress {
                address: addr,
                version: IpVersion::V4,
                kind: IpKind::Broadcast,
            });
        }
        None
    }

    pub fn clone(&self) -> Self {
        match self {
            IpKind::Private => IpKind::Private,
            IpKind::Public => IpKind::Public,
            IpKind::Loopback => IpKind::Loopback,
            IpKind::Apipa => IpKind::Apipa,
            IpKind::Broadcast => IpKind::Broadcast,
            IpKind::Linklocal => IpKind::Linklocal,
            IpKind::Uniqelocal => IpKind::Uniqelocal,
            IpKind::Uniqeglobal => IpKind::Uniqeglobal,
            IpKind::Multicast => IpKind::Multicast,
            IpKind::Unspecified => IpKind::Unspecified,
            IpKind::Netid => IpKind::Netid,
        }
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
    pub fn new(address: &str) -> Option<IpAddress> {
        if IpAddress::is_valid(address) {
            return Some(IpAddress {
                address: address.to_string(),
                version: if IpVersion::is_v4(address) {
                    IpVersion::V4
                } else {
                    IpVersion::V6
                },
                kind: IpKind::get_kind(address)?,
            });
        } else {
            None
        }
    }
    /// get the octats values of an Ipv4 address as u8 vector from giving &str
    pub fn get_octats_from_str(address: &str) -> Option<Vec<u8>> {
        if !IpVersion::is_v4(address) {
            return None;
        }
        let octats: Vec<u8> = address
            .split('.')
            .collect::<Vec<&str>>()
            .iter()
            .map(|oct| oct.parse::<u8>())
            .collect::<Result<Vec<u8>, _>>()
            .unwrap_or(vec![]);
        Some(octats)
    }
    /// get the octats values of an ipv4 IpAddress instance
    pub fn get_octats(&self) -> Option<Vec<u8>> {
        if IpVersion::is_v4(self.address().as_str()) {
            return IpAddress::get_octats_from_str(self.address().as_str());
        }
        None
    }
    // getters for the IpAddress properties
    /// a getter function for the version propertie
    pub fn version(&self) -> IpVersion {
        self.version.clone()
    }
    /// a getter function for the address propertie
    pub fn address(&self) -> String {
        self.address.clone()
    }
    /// a getter function for the kind propertie
    pub fn kind(&self) -> IpKind {
        self.kind.clone()
    }
    /// Creates a clone from an existing instance
    pub fn clone(&self) -> IpAddress {
        IpAddress {
            address: self.address.clone(),
            version: self.version.clone(),
            kind: self.kind.clone(),
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
        let octats_values: Vec<u8> = IpAddress::get_octats_from_str(mask).unwrap_or(vec![]);
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
    pub fn get_prefix(mask: &str) -> Option<u8> {
        if Mask::is_valid(mask) {
            let octats: Vec<&str> = mask.split('.').collect();
            let octats_values: Vec<u8> = octats
                .iter()
                .map(|oct| oct.parse::<u8>())
                .collect::<Result<Vec<u8>, _>>()
                .unwrap();
            let mask_value = (octats_values[0] as u32) << 24
                | (octats_values[1] as u32) << 16
                | (octats_values[2] as u32) << 8
                | octats_values[3] as u32;
            return Some(mask_value.leading_ones() as u8);
        }
        None
    }
    /// creates a new Mask instance
    pub fn new(mask: &str) -> Option<Mask> {
        let prefix = Mask::get_prefix(mask).unwrap();
        if Mask::is_valid(mask) {
            return Some(Mask {
                mask: mask.to_string(),
                prefix,
                num_of_hosts: (prefix as u32 - 2),
            });
        }
        None
    }
    /// creates new Mask instance from giving prefix
    pub fn from_prefix(prefix: u8) -> Option<Mask> {
        if prefix > 32 {
            return None;
        }
        let full_bytes: u32 = u32::MAX;
        let mask_bytes = full_bytes >> 32 - prefix;
        let mut octats = vec![];
        octats.push(mask_bytes.to_ne_bytes()[0]);
        octats.push(mask_bytes.to_ne_bytes()[1]);
        octats.push(mask_bytes.to_ne_bytes()[2]);
        octats.push(mask_bytes.to_ne_bytes()[3]);
        Some(Mask {
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
            num_of_hosts: 2u32.pow(32 - prefix as u32) - 2,
        })
    }

    pub fn mask(&self) -> String {
        self.mask.clone()
    }

    pub fn prefix(&self) -> u8 {
        self.prefix.clone()
    }

    pub fn num_of_hosts(&self) -> u32 {
        self.num_of_hosts.clone()
    }
}

impl Display for Mask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.mask())
    }
}

impl Network {
    /// Creates a new ipv4 Network instance from giving net id and subnet mask
    pub fn new(id: IpAddress, mask: Mask) -> Option<Network> {
        if IpKind::is_netid(id.address().as_str(), &mask) {
            let octats = IpAddress::get_octats(&id).unwrap();
            let hosts = mask.num_of_hosts() + 1;
            if IpVersion::is_v4(&id.address().as_str()) {
                return Some(Network {
                    id,
                    mask,
                    broadcast: IpAddress::new(
                        format!(
                            "{}.{}.{}.{}",
                            octats[0],
                            octats[1],
                            octats[2],
                            octats[3] as u32 + hosts
                        )
                        .as_str(),
                    )
                    .unwrap(),
                });
            }
        }
        None
    }
    /// Creates a new network from string slice in the "netid/prefix" format
    pub fn from_str(net: &str) -> Option<Network> {
        let networks_items = net.split('/').collect::<Vec<&str>>();
        if networks_items.len() != 2 {
            return None;
        }
        let prefix = networks_items[1].parse::<u8>().unwrap_or(0);
        if prefix == 0 {
            return None;
        }
        let mask = Mask::from_prefix(prefix);
        if let Some(mask) = mask {
            if IpKind::is_netid(networks_items[0], &mask) {
                let netid = IpAddress::new(networks_items[0])?;
                return Some(Network {
                    id: netid.clone(),                     
                    broadcast: IpKind::get_broadcast(netid.address().as_str(), &mask)?,
                    mask,
                });
            } else {
                return None;
            }
        } else {
            None
        }
    }
    /// Checks if a giving Ip address is in the self network
    pub fn containes(&self, address: IpAddress) -> bool {
        if IpVersion::is_v4(address.address().as_str()) {
            let octats = address.get_octats().unwrap();
            let netid_octs = self.id.get_octats().unwrap();
            let bcast_octs = self.broadcast.get_octats().unwrap();
            let prefix = self.mask.prefix();
            if prefix >= 24 {
                octats[3] > netid_octs[3] && octats[3] < bcast_octs[3]
            } else if prefix >= 16 {
                octats[2] <= bcast_octs[2] && octats[3] > netid_octs[3] && octats[3] < bcast_octs[3]
            } else if prefix >= 8 {
                octats[1] <= bcast_octs[1] && octats[3] > netid_octs[3] && octats[3] < bcast_octs[3]
            } else {
                octats[0] <= bcast_octs[0] && octats[3] > netid_octs[3] && octats[3] < bcast_octs[3]
            }
        } else {
            false
        }
    }
    /// getter for the broadcast property
    pub fn broadcast(&self) -> IpAddress {
        self.broadcast.clone()
    }
    /// getter for the netid property
    pub fn netid(&self) -> IpAddress {
        self.id.clone()
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
