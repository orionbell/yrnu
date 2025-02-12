use clap::{builder, command, value_parser, Arg, ArgAction, ArgGroup, Command};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use yrnu::config::netdev::router::{
    Bgp, DhcpPool, DistributeType, Eigrp, EncapsulationType, Hsrp, Interface, Ospf, OspfAreaType,
    Rip, Route, Router, RouterConfig, StaticRoute,
};
use yrnu::config::netdev::{InterfaceKind, Service};
use yrnu::config::Config;
use yrnu::core::{IpAddress, IpVersion, MacAddress, Mask, Network};
use yrnu::lua;

fn read_script(name: String) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let args_matches = command!()
        .about(
            "a tool for networking and cyber specialists, 
featuring Lua scripting and the yrnu library for crafting networking tools and automating tasks. 
Key features include configuring network settings, sending custom traffic, and deploying servers.",
        )
        .version("0.0.1")
        .subcommand(
            Command::new("config")
                .about("configure linux/Windows machines and network devices.")
                .subcommand(
                    Command::new("router")
                        .about("router configuration")
                        .arg(
                            Arg::new("hostname")
                                .short('H')
                                .long("hostname")
                                .help("configure router hostname."),
                        )
                        .arg(
                            Arg::new("interactive")
                                .short('I')
                                .action(ArgAction::SetTrue)
                                .long("interactive")
                                .help("configure router in interactive mode."),
                        )
                        .arg(
                            Arg::new("secret")
                                .short('s')
                                .long("secret")
                                .help("configure router enable secret."),
                        )
                        .arg(
                            Arg::new("password")
                                .short('p')
                                .long("password")
                                .help("configure router enable password."),
                        )
                        .arg(
                            Arg::new("password_enc")
                                .short('E')
                                .long("password-encryption")
                                .help("set enable password encryption service.")
                                .value_parser(builder::BoolishValueParser::new()),
                        )
                        .arg(
                            Arg::new("banner")
                                .short('b')
                                .long("banner")
                                .help("configure router banner message."),
                        )
                        .arg(
                            Arg::new("enable_ipv6")
                                .short('6')
                                .long("enable-ipv6")
                                .value_parser(builder::BoolishValueParser::new())
                                .help("enable ipv6 on the router."),
                        )
                        .subcommand(
                            Command::new("interface")
                                .about("interface configuration.")
                                .arg(
                                    Arg::new("ipv4")
                                        .short('4')
                                        .long("ipv4")
                                        .help("configure ipv4 address.")
                                        .value_name("address/prefix")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("ipv6")
                                        .short('6')
                                        .long("ipv6")
                                        .help("configure ipv6 address.")
                                        .value_name("address/prefix")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("mac")
                                        .short('m')
                                        .long("mac")
                                        .help("configure mac address.")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("type")
                                        .short('t')
                                        .long("type")
                                        .help("configure interface type.")
                                        .required(true),
                                )
                                .arg(
                                    Arg::new("state")
                                        .short('s')
                                        .long("state")
                                        .help("configure interface state.")
                                        .value_parser(["up", "down"])
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("speed")
                                        .short('S')
                                        .long("speed")
                                        .value_parser(value_parser!(u16))
                                        .help("configure interface speed.")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("delay")
                                        .short('d')
                                        .long("delay")
                                        .value_parser(value_parser!(u32))
                                        .help("configure interface delay.")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("bandwidth")
                                        .short('b')
                                        .long("bandwidth")
                                        .value_parser(value_parser!(u32))
                                        .help("configure interface bandwidth.")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("mtu")
                                        .short('M')
                                        .long("mtu")
                                        .value_parser(value_parser!(u16))
                                        .help("configure interface mtu.")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("encapsulation")
                                        .short('e')
                                        .long("encapsulation")
                                        .value_parser(["dot1q"])
                                        .help("configure sub-interface encapsulation type.")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("vlan")
                                        .short('v')
                                        .long("vlan")
                                        .value_parser(value_parser!(u16))
                                        .help("configure encapsulation vlan.")
                                        .requires("encapsulation")
                                        .required_if_eq("encapsulation", "dot1q"),
                                )
                                .arg(
                                    Arg::new("native")
                                        .short('n')
                                        .long("native")
                                        .action(ArgAction::SetTrue)
                                        .help("configure encapsulation vlan as native")
                                        .requires("encapsulation")
                                        .required(false),
                                ),
                        )
                        .subcommand(
                            Command::new("rip")
                                .about("rip configuration.")
                                .arg(
                                    Arg::new("version")
                                        .short('v')
                                        .long("version")
                                        .help("configure rip version.")
                                        .value_parser(["1", "2"])
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("auto_sum")
                                        .short('a')
                                        .long("auto-summary")
                                        .help("configure automatic summarization in rip.")
                                        .value_parser(builder::BoolishValueParser::new())
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("default_originate")
                                        .short('d')
                                        .long("default-originate")
                                        .help("configure rip default originate.")
                                        .value_parser(builder::BoolishValueParser::new())
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("redistribute")
                                        .short('R')
                                        .long("redistribute")
                                        .ignore_case(true)
                                        .help("configure rip redistribute.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("networks")
                                        .short('n')
                                        .long("networks")
                                        .help("configure rip networks.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("passive_interfaces")
                                        .short('i')
                                        .long("passive-interfaces")
                                        .help("configure rip passive interfaces.")
                                        .value_delimiter(','),
                                ),
                        )
                        .subcommand(
                            Command::new("ospf")
                                .about("ospf configuration.")
                                .arg(
                                    Arg::new("pid")
                                        .short('p')
                                        .long("process-id")
                                        .value_parser(value_parser!(u32))
                                        .help("configure ospf process id.")
                                        .required(true),
                                )
                                .arg(
                                    Arg::new("router_id")
                                        .short('r')
                                        .long("router-id")
                                        .help("configure ospf router id.")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("default_originate")
                                        .short('d')
                                        .long("default-originate")
                                        .help("configure ospf default originate.")
                                        .value_parser(builder::BoolishValueParser::new())
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("redistribute")
                                        .short('R')
                                        .long("redistribute")
                                        .ignore_case(true)
                                        .help("configure ospf redistribute.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("networks")
                                        .short('n')
                                        .long("networks")
                                        .help("configure ospf networks.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("passive_interfaces")
                                        .short('i')
                                        .long("passive-interfaces")
                                        .help("configure ospf passive interfaces.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("neighbors")
                                        .short('N')
                                        .long("neighbors")
                                        .help("configure ospf neighbors.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("ref_bandwidth")
                                        .short('b')
                                        .long("reference-bandwidth")
                                        .value_parser(value_parser!(u32))
                                        .help("configure ospf reference bandwidth."),
                                )
                                .arg(
                                    Arg::new("special_areas")
                                        .short('a')
                                        .long("special-areas")
                                        .value_delimiter(',')
                                        .help("configure ospf special areas."),
                                ),
                        )
                        .subcommand(
                            Command::new("eigrp")
                                .about("eigrp configuration.")
                                .arg(
                                    Arg::new("as_number")
                                        .short('a')
                                        .long("as-number")
                                        .value_parser(value_parser!(u16))
                                        .help("configure eigrp AS number.")
                                        .required(true),
                                )
                                .arg(
                                    Arg::new("router_id")
                                        .short('r')
                                        .long("router-id")
                                        .help("configure eigrp router id.")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("auto_sum")
                                        .short('A')
                                        .long("auto-summary")
                                        .help("configure eigrp automatic summarization.")
                                        .value_parser(builder::BoolishValueParser::new())
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("redistribute")
                                        .short('R')
                                        .long("redistribute")
                                        .help("configure eigrp redistribute.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("networks")
                                        .short('n')
                                        .long("networks")
                                        .help("configure eigrp networks.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("passive_interfaces")
                                        .short('i')
                                        .long("passive-interfaces")
                                        .help("configure eigrp passive interfaces.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("neighbors")
                                        .short('N')
                                        .long("neighbors")
                                        .help("configure eigrp neighbors.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("variance")
                                        .short('v')
                                        .long("variance")
                                        .help("configure eigrp variance."),
                                ),
                        )
                        .subcommand(
                            Command::new("bgp")
                                .about("bgp configuration.")
                                .arg(
                                    Arg::new("as_number")
                                        .short('A')
                                        .long("as-number")
                                        .value_parser(value_parser!(u32))
                                        .help("configure bgp AS number.")
                                        .required(true),
                                )
                                .arg(
                                    Arg::new("router_id")
                                        .short('r')
                                        .long("router-id")
                                        .help("configure bgp router id.")
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("sync")
                                        .short('s')
                                        .long("synchronization")
                                        .help("configure bgp synchronization.")
                                        .value_parser(builder::BoolishValueParser::new())
                                        .required(false),
                                )
                                .arg(
                                    Arg::new("redistribute")
                                        .short('R')
                                        .long("redistribute")
                                        .help("configure bgp redistribute.")
                                        .value_parser([
                                            "connected",
                                            "staic",
                                            "rip",
                                            "ospf",
                                            "eigrp",
                                        ])
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("networks")
                                        .short('n')
                                        .long("networks")
                                        .help("configure eigrp networks.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("passive_interfaces")
                                        .short('i')
                                        .long("passive-interfaces")
                                        .help("configure bgp passive interfaces.")
                                        .value_delimiter(','),
                                )
                                .arg(
                                    Arg::new("neighbors")
                                        .short('N')
                                        .long("neighbors")
                                        .help("configure bgp neighbors.")
                                        .value_delimiter(','),
                                ),
                        )
                        .subcommand(
                            Command::new("static")
                                .about("static route configuration.")
                                .arg(
                                    Arg::new("destination")
                                        .short('d')
                                        .long("destination")
                                        .help("configure static route destination."),
                                )
                                .arg(
                                    Arg::new("interface")
                                        .short('i')
                                        .long("interface")
                                        .help("configure static route via out interface."),
                                )
                                .arg(
                                    Arg::new("address")
                                        .short('a')
                                        .long("address")
                                        .help("configure static route via next router address."),
                                )
                                .group(
                                    ArgGroup::new("via")
                                        .args(["address", "interface"])
                                        .required(true),
                                ),
                        )
                        .subcommand(
                            Command::new("dhcp-pool")
                                .about("dhcp pool configuration.")
                                .arg(
                                    Arg::new("name")
                                        .short('N')
                                        .long("name")
                                        .required(true)
                                        .help("configure dhcp pool name."),
                                )
                                .arg(
                                    Arg::new("network")
                                        .short('n')
                                        .long("network")
                                        .value_parser(Network::from_str)
                                        .required(true)
                                        .help("configure dhcp pool network."),
                                )
                                .arg(
                                    Arg::new("gateway")
                                        .short('g')
                                        .long("default-gateway")
                                        .value_parser(IpAddress::new)
                                        .help("configure dhcp pool default gateway address."),
                                )
                                .arg(
                                    Arg::new("dns")
                                        .short('d')
                                        .long("dns-server")
                                        .value_parser(IpAddress::new)
                                        .help("configure dhcp pool dns server address."),
                                )
                                .arg(
                                    Arg::new("domain")
                                        .short('D')
                                        .long("domain-name")
                                        .help("configure dhcp pool domain name."),
                                )
                                .arg(
                                    Arg::new("excluded_low")
                                        .short('l')
                                        .long("exclude-low-address")
                                        .help("configure dhcp pool exclude low address.")
                                        .value_parser(IpAddress::new),
                                )
                                .arg(
                                    Arg::new("excluded_high")
                                        .short('H')
                                        .long("exclude-high-address")
                                        .help("configure dhcp pool exclude high address.")
                                        .value_parser(IpAddress::new),
                                ),
                        )
                        .subcommand(
                            Command::new("hsrp")
                                .about("hsrp configuration.")
                                .arg(
                                    Arg::new("interface")
                                        .short('i')
                                        .long("interface")
                                        .required(true)
                                        .help("configure hsrp interface name."),
                                )
                                .arg(
                                    Arg::new("address")
                                        .short('a')
                                        .long("virtual-address")
                                        .value_parser(IpAddress::new)
                                        .required(true)
                                        .help("configure hsrp virtual address."),
                                )
                                .arg(
                                    Arg::new("priority")
                                        .short('p')
                                        .long("priority")
                                        .value_parser(value_parser!(u8))
                                        .help("configure hsrp interface priority."),
                                )
                                .arg(
                                    Arg::new("group")
                                        .short('g')
                                        .long("group")
                                        .value_parser(value_parser!(u16).range(..4097))
                                        .help("configure hsrp group number."),
                                )
                                .arg(
                                    Arg::new("preempt")
                                        .short('P')
                                        .long("preempt")
                                        .action(ArgAction::SetTrue)
                                        .help("configure hsrp as preempt."),
                                )
                                .arg(
                                    Arg::new("version")
                                        .short('v')
                                        .long("version")
                                        .help("configure hsrp version.")
                                        .value_parser(value_parser!(u8).range(..3)),
                                ),
                        ),
                ),
        )
        .subcommand(Command::new("packet").about("send and sniff network packets."))
        .subcommand(Command::new("server").about("spown varius types of servers."))
        .arg(
            Arg::new("script")
                .help("A lua script to execute")
                .exclusive(true)
                .value_name("SCRIPT"),
        )
        .get_matches();
    if let Some(script) = args_matches.get_one::<String>("script") {
        match read_script(script.to_string()) {
            Ok(contents) => {
                let lua_ctx = lua::init();
                if let Err(e) = lua::run(&lua_ctx.unwrap(), &contents) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    } else {
        match args_matches.subcommand() {
            Some(("config", config)) => match config.subcommand() {
                Some(("router", router)) => {
                    let mut rtr = Router::default();
                    if let Some(hostname) = router.get_one::<String>("hostname") {
                        _ = rtr.hostname(Some(hostname.to_owned()));
                    }
                    if let Some(secret) = router.get_one::<String>("secret") {
                        _ = rtr.secret(Some(secret.to_owned()))
                    }
                    if let Some(password) = router.get_one::<String>("password") {
                        _ = rtr.password(Some(password.to_owned()))
                    }
                    if let Some(password_enc) = router.get_one::<bool>("password_enc") {
                        _ = rtr.add_service(Service::PasswordEncryption(*password_enc));
                    }
                    if let Some(banner) = router.get_one::<String>("banner") {
                        _ = rtr.banner(Some(banner.to_owned()));
                    }
                    if let Some(banner) = router.get_one::<String>("banner") {
                        _ = rtr.banner(Some(banner.to_owned()));
                    }
                    if let Some(enable) = router.get_one::<bool>("enable_ipv6") {
                        _ = rtr.enable_ipv6(Some(*enable));
                    }
                    let mut config: String = rtr.config();
                    config = format!(
                        "{config}{}",
                        match router.subcommand() {
                            Some(("interface", interface)) => {
                                let kind = InterfaceKind::from_str(
                                    interface.get_one::<String>("type").unwrap(),
                                )
                                .unwrap_or_else(|_| {
                                    eprintln!(
                                        "Invalid interface type {}.",
                                        interface.get_one::<String>("type").unwrap()
                                    );
                                    std::process::exit(-1)
                                });
                                let is_sub = kind.is_sub();
                                let mut iface = Interface::new(kind);
                                if let Some(ipv4) = interface.get_one::<String>("ipv4") {
                                    let parts = ipv4.split("/").collect::<Vec<&str>>();
                                    let address =
                                        IpAddress::new(parts[0]).expect("Invalid ipv4 address");
                                    let prefix = parts[1].parse::<u8>().expect("Invalid prefix");
                                    let mask =
                                        Mask::from_prefix(prefix).expect("Invalid ipv4 address");
                                    _ = iface.ipv4(Some(address), Some(mask));
                                }
                                if let Some(ipv6) = interface.get_one::<String>("ipv6") {
                                    let parts = ipv6.split("/").collect::<Vec<&str>>();
                                    let address =
                                        IpAddress::new(parts[0]).expect("Invalid ipv4 address");
                                    let prefix = parts[1].parse::<u8>().expect("Invalid prefix");
                                    _ = iface.ipv6(Some(address), Some(prefix));
                                }
                                if let Some(mac) = interface.get_one::<String>("mac") {
                                    let mac = MacAddress::new(mac).expect("Invalid mac address");
                                    iface.mac(Some(mac));
                                }
                                if let Some(state) = interface.get_one::<String>("state") {
                                    if state == "up" {
                                        iface.turn_on();
                                    } else {
                                        iface.shutdown();
                                    }
                                }
                                if let Some(speed) = interface.get_one::<u16>("speed") {
                                    iface.speed(Some(*speed)).expect("Invalid speed value");
                                }
                                if let Some(delay) = interface.get_one::<u32>("delay") {
                                    iface.delay(Some(*delay)).expect("Invalid dealy value");
                                }
                                if let Some(bandwidth) = interface.get_one::<u32>("bandwidth") {
                                    iface
                                        .bandwidth(Some(*bandwidth))
                                        .expect("Invalid bandwidth value");
                                }
                                if let Some(mtu) = interface.get_one::<u16>("mtu") {
                                    iface.mtu(Some(*mtu)).expect("Invalid mtu value");
                                }
                                if is_sub {
                                    if let Some(enc) = interface.get_one::<String>("encapsulation")
                                    {
                                        let enc = EncapsulationType::from_str(enc).unwrap();
                                        let vlan: u16 = *interface.get_one("vlan").unwrap();
                                        let as_native =
                                            interface.get_one::<bool>("native").unwrap();
                                        iface.encapsulation(Some((enc, vlan, *as_native)));
                                    }
                                }
                                iface.config()
                            }
                            Some(("rip", rip_args)) => {
                                let mut rip = Rip::new();
                                if let Some(auto_sum) = rip_args.get_one::<bool>("auto_sum") {
                                    rip.auto_sum(Some(*auto_sum));
                                }
                                if let Some(version) = rip_args.get_one::<String>("version") {
                                    if version == "2" {
                                        rip.version_2();
                                    } else {
                                        rip.version_1();
                                    }
                                }
                                if let Some(def_originate) =
                                    rip_args.get_one::<bool>("default_originate")
                                {
                                    rip.default_originate(Some(*def_originate));
                                }
                                if let Some(networks) = rip_args.get_many::<String>("networks") {
                                    for net in networks {
                                        if let Ok(network) = Network::from_str(net) {
                                            rip.add_network(&network);
                                        } else {
                                            eprintln!("Invalid network {}", net);
                                            std::process::exit(-1)
                                        }
                                    }
                                }
                                if let Some(passive_ifs) =
                                    rip_args.get_many::<String>("passive_interfaces")
                                {
                                    let mut inf;
                                    let mut pass_if;
                                    let mut inf_type;
                                    let mut indexs: Vec<u8>;
                                    for passive_if in passive_ifs {
                                        pass_if = passive_if.split(' ');
                                        inf_type = pass_if.next();
                                        indexs = pass_if
                                            .next()
                                            .unwrap_or_else(|| {
                                                eprintln!("Interface Indexes are missing");
                                                std::process::exit(-1)
                                            })
                                            .split('/')
                                            .map(|i| {
                                                i.parse::<u8>().expect("Invalid index value {i}")
                                            })
                                            .collect();
                                        inf = if inf_type == Some("gigabit") {
                                            InterfaceKind::GigabitEthernet(indexs, None)
                                        } else {
                                            InterfaceKind::FastEthernet(indexs, None)
                                        };
                                        rip.add_passive_if(&inf);
                                    }
                                }

                                if let Some(dis_types) = rip_args.get_many::<String>("redistribute")
                                {
                                    let mut distro;
                                    for dis_type in dis_types {
                                        distro = dis_type.split(' ');
                                        let dis_type =
                                            match distro.next().unwrap().to_lowercase().as_str() {
                                                "connected" => DistributeType::Connected,
                                                "static" => DistributeType::Static,
                                                "rip" => DistributeType::Rip,
                                                "ospf" => {
                                                    if let Ok(pid) = distro
                                                        .next()
                                                        .unwrap_or_else(|| {
                                                            eprintln!("Proccess number is missing");
                                                            std::process::exit(-1)
                                                        })
                                                        .parse::<u32>()
                                                    {
                                                        DistributeType::Ospf(pid)
                                                    } else {
                                                        eprintln!("AS number is missing");
                                                        std::process::exit(-1)
                                                    }
                                                }
                                                "eigrp" => {
                                                    if let Ok(as_num) = distro
                                                        .next()
                                                        .unwrap_or_else(|| {
                                                            eprintln!("AS number is missing");
                                                            std::process::exit(-1)
                                                        })
                                                        .parse::<u32>()
                                                    {
                                                        DistributeType::Eigrp(as_num)
                                                    } else {
                                                        eprintln!("AS number is missing");
                                                        std::process::exit(-1)
                                                    }
                                                }
                                                _ => todo!(),
                                            };
                                        rip.add_redistribute_type(&dis_type);
                                    }
                                }

                                rip.config()
                            }
                            Some(("ospf", ospf_args)) => {
                                let pid = ospf_args.get_one::<u32>("pid").unwrap();
                                let mut ospf = Ospf::new(*pid);
                                if let Some(router_id) = ospf_args.get_one::<String>("router_id") {
                                    if IpVersion::is_v4(router_id) {
                                        if let Ok(router_id) = IpAddress::octets_from_str(router_id)
                                        {
                                            ospf.router_id(Some(
                                                router_id[..4].try_into().unwrap(),
                                            ));
                                        } else {
                                            eprintln!("Invalid argument -router-id {}", router_id);
                                            std::process::exit(-1)
                                        }
                                    } else {
                                        eprintln!("Invalid argument -router-id {}", router_id);
                                        std::process::exit(-1)
                                    }
                                }
                                if let Some(ref_bandwidth) =
                                    ospf_args.get_one::<u32>("ref_bandwidth")
                                {
                                    ospf.reference_bandwidth(Some(*ref_bandwidth));
                                }
                                if let Some(def_originate) =
                                    ospf_args.get_one::<bool>("default_originate")
                                {
                                    ospf.default_originate(Some(*def_originate));
                                }
                                if let Some(networks) = ospf_args.get_many::<String>("networks") {
                                    for net in networks {
                                        let mut parts = net.split(" ");
                                        let net = parts.next().unwrap_or_else(|| {
                                            eprintln!("Empty network is invalid");
                                            std::process::exit(-1)
                                        });
                                        let area = parts.next().unwrap_or("0");
                                        let area: u32 = area.parse().unwrap_or_else(|_| {
                                            eprintln!("{} is an invalid value for area number (should be u32)", area);
                                            std::process::exit(-1)
                                        });
                                        if let Ok(network) = Network::from_str(net) {
                                            ospf.add_network(&network, area);
                                        } else {
                                            eprintln!("Invalid network {}", net);
                                            std::process::exit(-1)
                                        }
                                    }
                                }
                                if let Some(passive_ifs) =
                                    ospf_args.get_many::<String>("passive_interfaces")
                                {
                                    let mut inf;
                                    let mut pass_if;
                                    let mut inf_type;
                                    let mut indexs: Vec<u8>;
                                    for passive_if in passive_ifs {
                                        pass_if = passive_if.split(' ');
                                        inf_type = pass_if.next();
                                        indexs = pass_if
                                            .next()
                                            .unwrap_or_else(|| {
                                                eprintln!("Interface Indexes are missing");
                                                std::process::exit(-1)
                                            })
                                            .split('/')
                                            .map(|i| {
                                                i.parse::<u8>().expect("Invalid index value {i}")
                                            })
                                            .collect();
                                        inf = if inf_type == Some("gigabit") {
                                            InterfaceKind::GigabitEthernet(indexs, None)
                                        } else {
                                            InterfaceKind::FastEthernet(indexs, None)
                                        };
                                        ospf.add_passive_if(&inf);
                                    }
                                }

                                if let Some(dis_types) =
                                    ospf_args.get_many::<String>("redistribute")
                                {
                                    let mut distro;
                                    for dis_type in dis_types {
                                        distro = dis_type.split(' ');
                                        let dis_type =
                                            match distro.next().unwrap().to_lowercase().as_str() {
                                                "connected" => DistributeType::Connected,
                                                "static" => DistributeType::Static,
                                                "rip" => DistributeType::Rip,
                                                "ospf" => {
                                                    if let Ok(pid) = distro
                                                        .next()
                                                        .unwrap_or_else(|| {
                                                            eprintln!("Proccess number is missing");
                                                            std::process::exit(-1)
                                                        })
                                                        .parse::<u32>()
                                                    {
                                                        DistributeType::Ospf(pid)
                                                    } else {
                                                        eprintln!("AS number is missing");
                                                        std::process::exit(-1)
                                                    }
                                                }
                                                "eigrp" => {
                                                    if let Ok(as_num) = distro
                                                        .next()
                                                        .unwrap_or_else(|| {
                                                            eprintln!("AS number is missing");
                                                            std::process::exit(-1)
                                                        })
                                                        .parse::<u32>()
                                                    {
                                                        DistributeType::Eigrp(as_num)
                                                    } else {
                                                        eprintln!("AS number is missing");
                                                        std::process::exit(-1)
                                                    }
                                                }
                                                _ => todo!(),
                                            };
                                        ospf.add_redistribute_type(&dis_type);
                                    }
                                }
                                if let Some(neighbors) = ospf_args.get_many::<String>("neighbors") {
                                    for neighbor in neighbors {
                                        if let Ok(neighbor) = IpAddress::new(neighbor) {
                                            ospf.add_neighbor(&neighbor);
                                        } else {
                                            eprintln!("Invalid neighbor address {}", neighbor);
                                            std::process::exit(-1)
                                        }
                                    }
                                }
                                if let Some(areas) = ospf_args.get_many::<String>("special_areas") {
                                    for area in areas {
                                        if let Ok(area) = OspfAreaType::from_str(area) {
                                            ospf.add_special_area(&area);
                                        } else {
                                            eprintln!("Invalid area type {}", area);
                                            std::process::exit(-1)
                                        }
                                    }
                                }
                                ospf.config()
                            }
                            Some(("eigrp", eigrp_args)) => {
                                let as_num = eigrp_args.get_one::<u16>("as_number").unwrap();
                                let mut eigrp = Eigrp::new(*as_num);
                                if let Some(router_id) = eigrp_args.get_one::<String>("router_id") {
                                    if IpVersion::is_v4(router_id) {
                                        if let Ok(router_id) = IpAddress::octets_from_str(router_id)
                                        {
                                            eigrp.router_id(Some(
                                                router_id[..4].try_into().unwrap(),
                                            ));
                                        } else {
                                            eprintln!("Invalid argument -router-id {}", router_id);
                                            std::process::exit(-1)
                                        }
                                    } else {
                                        eprintln!("Invalid argument -router-id {}", router_id);
                                        std::process::exit(-1)
                                    }
                                }
                                if let Some(variance) = eigrp_args.get_one::<u8>("variance") {
                                    eigrp.variance(Some(*variance));
                                }
                                if let Some(networks) = eigrp_args.get_many::<String>("networks") {
                                    for net in networks {
                                        if let Ok(network) = Network::from_str(net) {
                                            eigrp.add_network(&network);
                                        } else {
                                            eprintln!("Invalid network {}", net);
                                            std::process::exit(-1)
                                        }
                                    }
                                }
                                if let Some(passive_ifs) =
                                    eigrp_args.get_many::<String>("passive_interfaces")
                                {
                                    let mut inf;
                                    let mut pass_if;
                                    let mut inf_type;
                                    let mut indexs: Vec<u8>;
                                    for passive_if in passive_ifs {
                                        pass_if = passive_if.split(' ');
                                        inf_type = pass_if.next();
                                        indexs = pass_if
                                            .next()
                                            .unwrap_or_else(|| {
                                                eprintln!("Interface Indexes are missing");
                                                std::process::exit(-1)
                                            })
                                            .split('/')
                                            .map(|i| {
                                                i.parse::<u8>().expect("Invalid index value {i}")
                                            })
                                            .collect();
                                        inf = if inf_type == Some("gigabit") {
                                            InterfaceKind::GigabitEthernet(indexs, None)
                                        } else {
                                            InterfaceKind::FastEthernet(indexs, None)
                                        };
                                        eigrp.add_passive_if(&inf);
                                    }
                                }

                                if let Some(dis_types) =
                                    eigrp_args.get_many::<String>("redistribute")
                                {
                                    let mut distro;
                                    for dis_type in dis_types {
                                        distro = dis_type.split(' ');
                                        let dis_type =
                                            match distro.next().unwrap().to_lowercase().as_str() {
                                                "connected" => DistributeType::Connected,
                                                "static" => DistributeType::Static,
                                                "rip" => DistributeType::Rip,
                                                "ospf" => {
                                                    if let Ok(pid) = distro
                                                        .next()
                                                        .unwrap_or_else(|| {
                                                            eprintln!("Proccess number is missing");
                                                            std::process::exit(-1)
                                                        })
                                                        .parse::<u32>()
                                                    {
                                                        DistributeType::Ospf(pid)
                                                    } else {
                                                        eprintln!("AS number is missing");
                                                        std::process::exit(-1)
                                                    }
                                                }
                                                "eigrp" => {
                                                    if let Ok(as_num) = distro
                                                        .next()
                                                        .unwrap_or_else(|| {
                                                            eprintln!("AS number is missing");
                                                            std::process::exit(-1)
                                                        })
                                                        .parse::<u32>()
                                                    {
                                                        DistributeType::Eigrp(as_num)
                                                    } else {
                                                        eprintln!("AS number is missing");
                                                        std::process::exit(-1)
                                                    }
                                                }
                                                _ => todo!(),
                                            };
                                        eigrp.add_redistribute_type(&dis_type);
                                    }
                                }
                                if let Some(neighbors) = eigrp_args.get_many::<String>("neighbors")
                                {
                                    for neighbor in neighbors {
                                        if let Ok(neighbor) = IpAddress::new(neighbor) {
                                            eigrp.add_neighbor(&neighbor);
                                        } else {
                                            eprintln!("Invalid neighbor address {}", neighbor);
                                            std::process::exit(-1)
                                        }
                                    }
                                }
                                eigrp.config()
                            }
                            Some(("bgp", bgp_args)) => {
                                let as_num = bgp_args.get_one::<u16>("as_number").unwrap();
                                let mut bgp = Bgp::new(*as_num);
                                if let Some(router_id) = bgp_args.get_one::<String>("router_id") {
                                    if IpVersion::is_v4(router_id) {
                                        if let Ok(router_id) = IpAddress::octets_from_str(router_id)
                                        {
                                            bgp.router_id(Some(router_id[..4].try_into().unwrap()));
                                        } else {
                                            eprintln!("Invalid argument -router-id {}", router_id);
                                            std::process::exit(-1)
                                        }
                                    } else {
                                        eprintln!("Invalid argument -router-id {}", router_id);
                                        std::process::exit(-1)
                                    }
                                }
                                if let Some(sync) = bgp_args.get_one::<bool>("sync") {
                                    bgp.synchronization(Some(*sync));
                                }
                                if let Some(networks) = bgp_args.get_many::<String>("networks") {
                                    for net in networks {
                                        if let Ok(network) = Network::from_str(net) {
                                            bgp.add_network(&network);
                                        } else {
                                            eprintln!("Invalid network {}", net);
                                            std::process::exit(-1)
                                        }
                                    }
                                }
                                if let Some(passive_ifs) =
                                    bgp_args.get_many::<String>("passive_interfaces")
                                {
                                    let mut inf;
                                    let mut pass_if;
                                    let mut inf_type;
                                    let mut indexs: Vec<u8>;
                                    for passive_if in passive_ifs {
                                        pass_if = passive_if.split(' ');
                                        inf_type = pass_if.next();
                                        indexs = pass_if
                                            .next()
                                            .unwrap_or_else(|| {
                                                eprintln!("Interface Indexes are missing");
                                                std::process::exit(-1)
                                            })
                                            .split('/')
                                            .map(|i| {
                                                i.parse::<u8>().expect("Invalid index value {i}")
                                            })
                                            .collect();
                                        inf = if inf_type == Some("gigabit") {
                                            InterfaceKind::GigabitEthernet(indexs, None)
                                        } else {
                                            InterfaceKind::FastEthernet(indexs, None)
                                        };
                                        bgp.add_passive_if(&inf);
                                    }
                                }

                                if let Some(dis_types) = bgp_args.get_many::<String>("redistribute")
                                {
                                    let mut distro;
                                    for dis_type in dis_types {
                                        distro = dis_type.split(' ');
                                        let dis_type =
                                            match distro.next().unwrap().to_lowercase().as_str() {
                                                "connected" => DistributeType::Connected,
                                                "static" => DistributeType::Static,
                                                "rip" => DistributeType::Rip,
                                                "ospf" => {
                                                    if let Ok(pid) = distro
                                                        .next()
                                                        .unwrap_or_else(|| {
                                                            eprintln!("Proccess number is missing");
                                                            std::process::exit(-1)
                                                        })
                                                        .parse::<u32>()
                                                    {
                                                        DistributeType::Ospf(pid)
                                                    } else {
                                                        eprintln!("AS number is missing");
                                                        std::process::exit(-1)
                                                    }
                                                }
                                                "eigrp" => {
                                                    if let Ok(as_num) = distro
                                                        .next()
                                                        .unwrap_or_else(|| {
                                                            eprintln!("AS number is missing");
                                                            std::process::exit(-1)
                                                        })
                                                        .parse::<u32>()
                                                    {
                                                        DistributeType::Eigrp(as_num)
                                                    } else {
                                                        eprintln!("AS number is missing");
                                                        std::process::exit(-1)
                                                    }
                                                }
                                                _ => todo!(),
                                            };
                                        bgp.add_redistribute_type(&dis_type);
                                    }
                                }
                                if let Some(neighbors) = bgp_args.get_many::<String>("neighbors") {
                                    for neighbor in neighbors {
                                        if let Ok(neighbor) = IpAddress::new(neighbor) {
                                            bgp.add_neighbor(&neighbor);
                                        } else {
                                            eprintln!("Invalid neighbor address {}", neighbor);
                                            std::process::exit(-1)
                                        }
                                    }
                                }
                                bgp.config()
                            }
                            Some(("static", static_args)) => {
                                let dst = Network::from_str(
                                    if let Some(dst) = static_args.get_one::<String>("destination")
                                    {
                                        dst
                                    } else {
                                        "0.0.0.0/0"
                                    },
                                )
                                .unwrap_or_else(|_| todo!());
                                if let Some(address) = static_args.get_one::<String>("address") {
                                    if let Ok(address) = IpAddress::new(address) {
                                        Route::Static(dst, StaticRoute::ViaAddress(address))
                                            .config()
                                    } else {
                                        todo!()
                                    }
                                } else {
                                    todo!()
                                }
                            }
                            Some(("dhcp-pool", pool_args)) => {
                                let mut pool = DhcpPool::new(
                                    pool_args.get_one::<String>("name").unwrap().to_owned(),
                                    pool_args.get_one::<Network>("network").unwrap().to_owned(),
                                );
                                if let Some(def) = pool_args.get_one::<IpAddress>("gateway") {
                                    pool.default_gateway(Some(def.to_owned()));
                                }
                                if let Some(dns) = pool_args.get_one::<IpAddress>("dns") {
                                    pool.dns(Some(dns.to_owned()));
                                }
                                if let Some(domain) = pool_args.get_one::<String>("domain") {
                                    pool.domain(Some(domain.to_owned()));
                                }
                                let low = pool_args.get_one::<IpAddress>("excluded_low");
                                let high = pool_args.get_one::<IpAddress>("excluded_high");
                                if low.is_some() || high.is_some() {
                                    pool.excluded_addresses(low, high);
                                }
                                pool.config()
                            }
                            Some(("hsrp", hsrp_args)) => {
                                let interface = InterfaceKind::from_str(
                                    hsrp_args.get_one::<String>("interface").unwrap(),
                                )
                                .unwrap_or_else(|e| {
                                    eprintln!("{}.", e);
                                    std::process::exit(-1)
                                });
                                let addr = hsrp_args.get_one::<IpAddress>("address").unwrap();
                                let version =
                                    hsrp_args.get_one::<u8>("version").unwrap_or(&(3 as u8));
                                if *addr.version() == IpVersion::V6 && *version != 2 {
                                    eprintln!("Hsrp version have to be version 2 to enable Ipv6.");
                                    std::process::exit(-1)
                                }
                                let mut hsrp = Hsrp::new(interface.to_owned(), addr.to_owned());
                                if *version > 2 {
                                    eprintln!(
                                        "Invalid hsrp version {}, possible values are [1,2].",
                                        version
                                    );
                                    std::process::exit(-1)
                                } else {
                                    hsrp.version(Some(*version));
                                }
                                if let Some(priority) = hsrp_args.get_one::<u8>("priority") {
                                    hsrp.priority(Some(*priority));
                                }
                                if let Some(preempt) = hsrp_args.get_one::<bool>("preempt") {
                                    hsrp.preempt(Some(*preempt));
                                }
                                if let Some(group) = hsrp_args.get_one::<u16>("group") {
                                    hsrp.group(Some(*group));
                                }
                                hsrp.config()
                            }
                            _ => {
                                todo!()
                            }
                        }
                    );
                    println!("{config}");
                }
                _ => {}
            },
            _ => lua::interpreter::start_interpreter().expect("Failed to run interpreter."),
        }
    }
}
