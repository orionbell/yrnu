use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

pub struct InvalidLineType;
pub struct InvalidPrivilageLevel;
pub struct InvalidTimeout;
pub struct InvalidStopbits;
pub struct InvalidDatabits;

impl Error for InvalidLineType {}
impl Error for InvalidPrivilageLevel {}
impl Error for InvalidTimeout {}
impl Error for InvalidStopbits {}
impl Error for InvalidDatabits {}

impl Display for InvalidLineType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid line type for transport input, only vty lines are supported.")
    }
}
impl Debug for InvalidLineType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InvalidPrivilageLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid privilage level, valid value should be between 1 - 15.")
    }
}
impl Debug for InvalidPrivilageLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InvalidTimeout {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid timeout value, valid value should be between 0 - 35791.")
    }
}
impl Debug for InvalidTimeout {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InvalidStopbits {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid stopbits value, valid values are 1/1.5/2.")
    }
}
impl Debug for InvalidStopbits {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Display for InvalidDatabits {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "An invalid timeout value, valid value are between 5 to 8.")
    }
}
impl Debug for InvalidDatabits {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}
