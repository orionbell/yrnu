use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

pub struct InvalidIpAddress;
pub struct InvalidIpV4Address;
pub struct InvalidIpV6Address;
pub struct InvalidMacAddress;
pub struct InvalidMask;
pub struct InvalidPrefix;
pub struct InvalidNetwork;
pub struct InterfaceNotExists;
pub struct InterfaceConvertionFailed;

impl Error for InvalidIpAddress {}
impl Error for InvalidMacAddress {}
impl Error for InvalidMask {}
impl Error for InvalidNetwork {}
impl Error for InvalidIpV4Address {}
impl Error for InvalidIpV6Address {}
impl Error for InvalidPrefix {}
impl Error for InterfaceNotExists {}
impl Error for InterfaceConvertionFailed {}

impl Display for InvalidIpAddress {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid ip address.")
    }
}
impl Debug for InvalidIpAddress {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InvalidMacAddress {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid mac address.")
    }
}
impl Debug for InvalidMacAddress {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InvalidMask {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid subnet mask.")
    }
}
impl Debug for InvalidMask {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InvalidNetwork {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid ip address.")
    }
}
impl Debug for InvalidNetwork {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InvalidIpV4Address {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid ipv4 address.")
    }
}
impl Debug for InvalidIpV4Address {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InvalidIpV6Address {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid ipv6 address.")
    }
}
impl Debug for InvalidIpV6Address {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InvalidPrefix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid mask prefix.")
    }
}
impl Debug for InvalidPrefix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InterfaceNotExists {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Interface doesn't exists.")
    }
}
impl Debug for InterfaceNotExists {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InterfaceConvertionFailed {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Failed converting Interface into NetworkInterface.")
    }
}
impl Debug for InterfaceConvertionFailed {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}
