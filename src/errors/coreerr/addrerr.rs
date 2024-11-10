use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

pub struct InvalidIpAddress;
pub struct InvalidIpV4Address;
pub struct InvalidMacAddress;
pub struct InvalidMask;
pub struct InvalidPrefix;
pub struct InvalidNetwork;

impl Error for InvalidIpAddress {}
impl Error for InvalidMacAddress {}
impl Error for InvalidMask {}
impl Error for InvalidNetwork {}
impl Error for InvalidIpV4Address {}
impl Error for InvalidPrefix {}

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


