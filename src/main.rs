use clap::{builder, command, value_parser, Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use yrnu::config::netdev::router::{
    Bgp, DistributeType, Eigrp, Interface, Ospf, Rip, Router, RouterConfig,
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
                .about("configure linux/Windows machines or network devices.")
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
                                        .value_parser(["gigabit", "fast"])
                                        .required(true),
                                )
                                .arg(
                                    Arg::new("index")
                                        .short('i')
                                        .long("index")
                                        .help("interface type index.")
                                        .value_name("INDEX")
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
                                let indexes = interface
                                    .get_one::<String>("index")
                                    .unwrap()
                                    .split("/")
                                    .map(|i| i.parse::<u8>().expect("invalid index"))
                                    .collect::<Vec<u8>>();

                                let mut iface =
                                    if interface.get_one::<String>("type").unwrap() == "gigabit" {
                                        Interface::gigabit_ethernet(indexes)
                                    } else {
                                        Interface::fast_ethernet(indexes)
                                    };
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
                                            InterfaceKind::GigabitEthernet(indexs)
                                        } else {
                                            InterfaceKind::FastEthernet(indexs)
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
                                        if let Ok(network) = Network::from_str(net) {
                                            ospf.add_network(&network);
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
                                            InterfaceKind::GigabitEthernet(indexs)
                                        } else {
                                            InterfaceKind::FastEthernet(indexs)
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
                                if let Some(variance) =
                                    eigrp_args.get_one::<u8>("variance")
                                {
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
                                            InterfaceKind::GigabitEthernet(indexs)
                                        } else {
                                            InterfaceKind::FastEthernet(indexs)
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
                                if let Some(neighbors) = eigrp_args.get_many::<String>("neighbors") {
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
                                            bgp.router_id(Some(
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
                                if let Some(sync) =
                                    bgp_args.get_one::<bool>("sync")
                                {
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
                                            InterfaceKind::GigabitEthernet(indexs)
                                        } else {
                                            InterfaceKind::FastEthernet(indexs)
                                        };
                                        bgp.add_passive_if(&inf);
                                    }
                                }

                                if let Some(dis_types) =
                                    bgp_args.get_many::<String>("redistribute")
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
