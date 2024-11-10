use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

pub struct InterfaceNotExists;

impl Error for InterfaceNotExists {}

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

pub struct InterfaceConvertionFailed;

impl Error for InterfaceConvertionFailed {}

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
