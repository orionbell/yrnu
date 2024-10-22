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
