use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

pub enum IpVersion {
    V4,
    V6,
}

pub enum IpKind {
    Public,
    Private,
    Loopback,
    Linklocal,
    Apipa,
    Uniqelocal,
    Uniqeglobal,
    Broadcast,
    Multicast,
    Unspecified,
}

pub struct IpAddress {
    address: String,
    version: IpVersion,
    kind: IpKind,
}

impl IpVersion {
    pub fn is_v4(address: &str) -> bool {
        match IpAddr::from_str(address) {
            Ok(addr) => IpAddr::is_ipv4(&addr),
            Err(..) => false,
        }
    }
    pub fn is_v6(address: &str) -> bool {
        match IpAddr::from_str(address) {
            Ok(addr) => IpAddr::is_ipv6(&addr),
            Err(..) => false,
        }
    }
}

impl IpKind {
    pub fn is_private(address: &str) -> bool {
        if IpVersion::is_v4(address) {
            match Ipv4Addr::from_str(address) {
                Ok(addr) => Ipv4Addr::is_private(&addr),
                Err(..) => false,
            }
        } else if IpVersion::is_v6(address) {
            if address[0..3].to_lowercase() == "fd00" || address[0..3].to_lowercase() == "fc00" {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

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

    pub fn is_broadcast(address: &str) -> bool {
        if IpVersion::is_v4(address) {
            match Ipv4Addr::from_str(address) {
                Ok(addr) => Ipv4Addr::is_broadcast(&addr),
                Err(..) => false,
            }
        } else {
            false
        }
    }

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

    pub fn is_public(address: &str) -> bool {
        if IpKind::is_loopback(address)
            || IpKind::is_private(address)
            || IpKind::is_broadcast(address)
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

    pub fn get_kind(address: &str) -> Result<IpKind, &str> {
        if IpKind::is_private(address) {
            if IpVersion::is_v4(address) {
                return Ok(IpKind::Private);
            }
            Ok(IpKind::Uniqelocal)
        } else if IpKind::is_loopback(address) {
            Ok(IpKind::Loopback)
        } else if IpKind::is_broadcast(address) {
            Ok(IpKind::Broadcast)
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
            Err("Invalid Address")
        }
    }
}

impl IpAddress {
    pub fn is_valid(address: &str) -> bool {
        if IpVersion::is_v4(address) || IpVersion::is_v6(address) {
            true
        } else {
            false
        }
    }

    pub fn new(address: &str) -> Result<IpAddress, &str> {
        if IpAddress::is_valid(address) {
            match IpKind::get_kind(address) {            
                Ok(kind) => return Ok(IpAddress {    
                    address: address.to_string(),
                    version: if IpVersion::is_v4(address) { IpVersion::V4 } else { IpVersion::V6 },
                    kind
                }),
                Err(s) => return Err(s)
            }
        } else {
            Err("invalid address")
        }
    }
}
