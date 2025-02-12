use super::Config;
use crate::error::configerr::*;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
pub mod router;
pub mod switch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineType {
    Console(u8),
    Vty(u8, u8),
    Tty(u8),
    Aux(u8),
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum LineLoginType {
    AAA,
    LoginLocal,
    Local(String),
    #[default]
    None,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum TransportType {
    All,
    #[default]
    None,
    SSH,
    Telnet,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FlowControlType {
    Software,
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParityType {
    Even,
    Odd,
    Space,
    Mark,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    kind: LineType,
    login: Option<LineLoginType>,
    banner: Option<bool>,
    session_limit: Option<u32>,
    speed: Option<u32>,
    transport_input: Option<TransportType>,
    transport_output: Option<TransportType>,
    privilage_level: Option<u8>,
    timeout: Option<u16>,
    flowcontrol: Option<FlowControlType>,
    parity: Option<ParityType>,
    stopbits: Option<f32>,
    databits: Option<u8>,
}

impl Line {
    pub fn new(kind: LineType) -> Line {
        Line {
            kind,
            login: None,
            banner: None,
            session_limit: None,
            speed: None,
            transport_input: None,
            transport_output: None,
            privilage_level: None,
            flowcontrol: None,
            timeout: None,
            parity: None,
            stopbits: None,
            databits: None,
        }
    }
    pub fn console(index: u8) -> Self {
        Self::new(LineType::Console(index))
    }
    pub fn vty(start: u8, end: u8) -> Self {
        Self::new(LineType::Vty(start, end))
    }
    pub fn tty(index: u8) -> Self {
        Self::new(LineType::Tty(index))
    }
    pub fn aux(index: u8) -> Self {
        Self::new(LineType::Aux(index))
    }
    pub fn is_console(&self) -> bool {
        if let LineType::Console(_) = self.kind {
            true
        } else {
            false
        }
    }
    pub fn is_vty(&self) -> bool {
        if let LineType::Vty(_, _) = self.kind {
            true
        } else {
            false
        }
    }
    pub fn is_aux(&self) -> bool {
        if let LineType::Aux(_) = self.kind {
            true
        } else {
            false
        }
    }
    pub fn is_tty(&self) -> bool {
        if let LineType::Tty(_) = self.kind {
            true
        } else {
            false
        }
    }
    pub fn transport_input(
        &mut self,
        transport: Option<TransportType>,
    ) -> Result<&mut Self, InvalidLineType> {
        if let LineType::Vty(_, _) = self.kind {
            self.transport_input = transport;
            Ok(self)
        } else {
            Err(InvalidLineType)
        }
    }
    pub fn transport_output(&mut self, transport: Option<TransportType>) -> &mut Self {
        self.transport_output = transport;
        self
    }
    pub fn login(&mut self, login: Option<LineLoginType>) -> &mut Self {
        self.login = login;
        self
    }
    pub fn banner(&mut self, banner: Option<bool>) -> &mut Self {
        if banner.is_none() {
            self.banner = None
        }
        self.banner = banner;
        self
    }
    pub fn session_limit(&mut self, limit: Option<u32>) -> &mut Self {
        self.session_limit = limit;
        self
    }
    pub fn speed(&mut self, speed: Option<u32>) -> &mut Self {
        self.speed = speed;
        self
    }
    pub fn privilage(&mut self, level: Option<u8>) -> Result<&mut Self, InvalidPrivilageLevel> {
        if let Some(level) = level {
            if level > 15 {
                Err(InvalidPrivilageLevel)
            } else {
                self.privilage_level = Some(level);
                Ok(self)
            }
        } else {
            self.privilage_level = None;
            Ok(self)
        }
    }
    pub fn timeout(&mut self, timeout: Option<u16>) -> Result<&mut Self, InvalidTimeout> {
        if let Some(timeout) = timeout {
            if timeout > 35791 {
                Err(InvalidTimeout)
            } else {
                self.timeout = Some(timeout);
                Ok(self)
            }
        } else {
            self.timeout = None;
            Ok(self)
        }
    }
    pub fn flow_control(&mut self, flowcontrol: Option<FlowControlType>) -> &mut Self {
        self.flowcontrol = flowcontrol;
        self
    }
    pub fn parity(&mut self, parity: Option<ParityType>) -> &mut Self {
        self.parity = parity;
        self
    }
    pub fn stopbits(&mut self, stopbits: Option<f32>) -> Result<&mut Self, InvalidStopbits> {
        if let Some(stopbits) = stopbits {
            if stopbits == 1.0 || stopbits == 1.5 || stopbits == 2.0 {
                self.stopbits = Some(stopbits);
                Ok(self)
            } else {
                Err(InvalidStopbits)
            }
        } else {
            self.stopbits = stopbits;
            Ok(self)
        }
    }
    pub fn databits(&mut self, databits: Option<u8>) -> Result<&mut Self, InvalidDatabits> {
        if let Some(databits) = databits {
            if databits >= 5 && databits <= 8 {
                self.databits = Some(databits);
                Ok(self)
            } else {
                Err(InvalidDatabits)
            }
        } else {
            self.databits = None;
            Ok(self)
        }
    }
    pub fn ssh(&mut self) -> Result<&mut Self, InvalidLineType> {
        self.transport_input(Some(TransportType::SSH))
    }
}
impl Config for Line {
    fn config(&self) -> String {
        let mut config = String::new();
        let enter = match &self.kind {
            LineType::Console(n) => format!("line console {}\n", n),
            LineType::Vty(start, end) => format!("line vty {} {}\n", start, end),
            LineType::Tty(n) => format!("line tty {}\n", n),
            LineType::Aux(n) => format!("line aux {}\n", n),
        };
        if let Some(login) = &self.login {
            match login {
                LineLoginType::Local(passwd) => {
                    config = format!("{config}password {}\nlogin\n", passwd)
                }
                LineLoginType::AAA => {}
                LineLoginType::LoginLocal => config = format!("{config}login local\n"),
                LineLoginType::None => {}
            }
        }
        if let Some(banner) = self.banner {
            if banner {
                config = format!("{config}motd-banner");
            } else {
                config = format!("{config}no motd-banner");
            }
        }
        if let Some(limit) = self.session_limit {
            config = format!("{config}session-limit {}\n", limit);
        }
        if let Some(speed) = self.speed {
            config = format!("{config}speed {}\n", speed);
        }
        if let Some(transport_input) = &self.transport_input {
            match transport_input {
                TransportType::All => config = format!("{config}transport input all\n"),
                TransportType::SSH => config = format!("{config}transport input ssh\n"),
                TransportType::Telnet => config = format!("{config}transport input telnet\n"),
                TransportType::None => config = format!("{config}transport input none\n"),
            }
        }
        if let Some(transport_output) = &self.transport_output {
            match transport_output {
                TransportType::All => config = format!("{config}transport output all\n"),
                TransportType::SSH => config = format!("{config}transport output ssh\n"),
                TransportType::Telnet => config = format!("{config}transport output telnet\n"),
                TransportType::None => config = format!("{config}transport output none\n"),
            }
        }
        if let Some(privilage) = self.privilage_level {
            config = format!("{config}privilage level {}\n", privilage);
        }
        if let Some(timeout) = &self.timeout {
            config = format!("{config}exec-timeout {}\n", timeout);
        }
        if let Some(flowcontrol) = &self.flowcontrol {
            match flowcontrol {
                FlowControlType::None => config = format!("{config}flowcontrol none\n"),
                FlowControlType::Software => config = format!("{config}flowcontrol software\n"),
            }
        }
        if let Some(parity) = &self.parity {
            match parity {
                ParityType::Even => {
                    config = format!("{config}parity even\n");
                }
                ParityType::Mark => {
                    config = format!("{config}parity mark\n");
                }
                ParityType::None => {
                    config = format!("{config}parity none\n");
                }
                ParityType::Odd => {
                    config = format!("{config}parity odd\n");
                }
                ParityType::Space => {
                    config = format!("{config}parity space\n");
                }
            }
        }
        if let Some(stopbits) = &self.stopbits {
            config = format!("{config}stopbits {}\n", stopbits);
        }
        if let Some(databits) = &self.databits {
            config = format!("{config}databits {}\n", databits);
        }
        if config != "" {
            format!("{enter}{config}exit\n")
        } else {
            String::new()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Service {
    PasswordEncryption(bool),
    LogTimestemp(bool),
    DebugTimestemp(bool),
    Dhcp(bool),
    Nagle(bool),
}
impl Config for Service {
    fn config(&self) -> String {
        let mut config = String::new();
        match self {
            Service::PasswordEncryption(enable) => {
                if *enable {
                    config = format!("service password-encryption\n")
                } else {
                    config = format!("no service password-encryption\n")
                }
            }
            Service::LogTimestemp(enable) => {
                if *enable {
                    config = format!("service timestemp log datetime msec\n")
                } else {
                    config = format!("no service timestemp log datetime msec\n")
                }
            }
            Service::DebugTimestemp(enable) => {
                if *enable {
                    config = format!("service timestemp debug datetime msec\n")
                } else {
                    config = format!("no service timestemp debug datetime msec\n")
                }
            }
            Service::Dhcp(enable) => {
                if *enable {
                    config = format!("service dhcpc\n")
                } else {
                    config = format!("no service dhcp\n")
                }
            }
            Service::Nagle(enable) => {
                if *enable {
                    config = format!("service nagle\n")
                } else {
                    config = format!("no service nagle\n")
                }
            }
        }
        config
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InterfaceKind {
    FastEthernet(Vec<u8>, Option<u32>),
    GigabitEthernet(Vec<u8>, Option<u32>),
    Lookback(u8),
    Vlan(u16),
}
impl InterfaceKind {
    pub fn fast_ethernet(indexes: Vec<u8>) -> Self {
        Self::FastEthernet(indexes, None)
    }
    pub fn gigabit_ethernet(indexes: Vec<u8>) -> Self {
        Self::GigabitEthernet(indexes, None)
    }
    pub fn is_sub(&self) -> bool {
        match self {
            Self::FastEthernet(_, sub) | Self::GigabitEthernet(_, sub) => sub.is_some(),
            _ => false,
        }
    }
}
impl FromStr for InterfaceKind {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = if s.to_lowercase().starts_with("gigabit") {
            vec!["gigabit", &s[7..]]
        } else if s.to_lowercase().starts_with("fast") {
            vec!["fast", &s[4..]]
        } else {
            vec![]
        };
        if parts.len() != 2 {
            todo!()
        } else {
            let kind = parts[0];
            let indexes = parts[1];
            if kind.to_lowercase() == "vlan" {
                if let Ok(i) = indexes.parse() {
                    return Ok(Self::Vlan(i));
                } else {
                    todo!();
                }
            }
            let indexes: Vec<&str> = indexes.trim().split(".").collect();
            let sub: Option<u32> = if indexes.len() == 2 {
                if let Ok(i) = indexes[1].parse() {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            };
            let indexes: Vec<u8> = indexes[0]
                .split("/")
                .map(|i| {
                    i.parse().unwrap_or_else(|e| {
                        println!("{e}");
                        todo!()
                    })
                })
                .collect();
            let inf = if kind.to_lowercase() == "fast" {
                Self::FastEthernet(indexes, sub)
            } else if kind.to_lowercase() == "gigabit" {
                Self::GigabitEthernet(indexes, sub)
            } else if kind.to_lowercase() == "loopback" {
                return Ok(Self::Lookback(indexes[0]));
            } else {
                return Err(todo!());
            };
            Ok(inf)
        }
    }
}
impl Display for InterfaceKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InterfaceKind::Vlan(n) => format!("vlan {}", n),
                InterfaceKind::GigabitEthernet(ind, sub) => format!(
                    "GigabitEthernet {}{}",
                    ind.into_iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join("/"),
                    if let Some(sub) = sub {
                        format!(".{}", sub)
                    } else {
                        "".to_string()
                    }
                ),
                InterfaceKind::FastEthernet(ind, sub) => format!(
                    "FastEthernet {}{}",
                    ind.into_iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join("/"),
                    if let Some(sub) = sub {
                        format!(".{}", sub)
                    } else {
                        "".to_string()
                    }
                ),
                InterfaceKind::Lookback(n) => format!("loopback {}", n),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum InterfaceState {
    Up,
    Down,
}
