use crate::data::models::{
    HeatmapParameters, PeerParameters, TestParameters, DEFAULT_CONTROL_PORT, DEFAULT_IPERF_PORT,
    DEFAULT_MAX_MTU, DEFAULT_MIN_MTU, DEFAULT_STEP,
};
use chrono::Local;
use clap::{App, Arg, SubCommand};
use std::str::FromStr;

// Функция для создания значений по умолчанию
pub fn create_default_values() -> Vec<String> {
    let min_mtu_str = DEFAULT_MIN_MTU.to_string();
    let max_mtu_str = DEFAULT_MAX_MTU.to_string();
    let step_str = DEFAULT_STEP.to_string();
    let control_port_str = DEFAULT_CONTROL_PORT.to_string();
    let iperf_port_str = DEFAULT_IPERF_PORT.to_string();
    let csv_filename = format!("wg_mtu_finder_{}.csv", Local::now().format("%Y%m%dT%H%M%S"));
    let heatmap_filename = format!("heatmap_{}.png", Local::now().format("%Y%m%dT%H%M%S"));

    vec![
        min_mtu_str,
        max_mtu_str,
        step_str,
        control_port_str,
        iperf_port_str,
        csv_filename,
        heatmap_filename,
    ]
}

// Функция для создания CLI интерфейса
pub fn create_cli_app(defaults: &[String]) -> App {
    App::new("WireGuard MTU Finder")
        .version("1.0")
        .author("MrSomFeRGO")
        .about("Utility to find optimal MTU for WireGuard connections")
        .subcommand(
            SubCommand::with_name("server")
                .about("Run in server mode")
                .arg(
                    Arg::with_name("interface")
                        .short("i")
                        .long("interface")
                        .value_name("INTERFACE")
                        .help("WireGuard interface name")
                        .required(true),
                )
                .arg(
                    Arg::with_name("min-mtu")
                        .long("min-mtu")
                        .value_name("MIN_MTU")
                        .help("Minimum MTU to test")
                        .default_value(&defaults[0]),
                )
                .arg(
                    Arg::with_name("max-mtu")
                        .long("max-mtu")
                        .value_name("MAX_MTU")
                        .help("Maximum MTU to test")
                        .default_value(&defaults[1]),
                )
                .arg(
                    Arg::with_name("step")
                        .long("step")
                        .value_name("STEP")
                        .help("MTU step size")
                        .default_value(&defaults[2]),
                )
                .arg(
                    Arg::with_name("server-port")
                        .long("server-port")
                        .value_name("PORT")
                        .help("Control connection port")
                        .default_value(&defaults[3]),
                )
                .arg(
                    Arg::with_name("iperf-port")
                        .long("iperf-port")
                        .value_name("PORT")
                        .help("Port for iperf tests")
                        .default_value(&defaults[4]),
                ),
        )
        .subcommand(
            SubCommand::with_name("peer")
                .about("Run in peer mode")
                .arg(
                    Arg::with_name("interface")
                        .short("i")
                        .long("interface")
                        .value_name("INTERFACE")
                        .help("WireGuard interface name")
                        .required(true),
                )
                .arg(
                    Arg::with_name("server-ip")
                        .long("server-ip")
                        .value_name("SERVER_IP")
                        .help("Server IP address")
                        .required(true),
                )
                .arg(
                    Arg::with_name("server-port")
                        .long("server-port")
                        .value_name("PORT")
                        .help("Control connection port")
                        .default_value(&defaults[3]),
                )
                .arg(
                    Arg::with_name("iperf-port")
                        .long("iperf-port")
                        .value_name("PORT")
                        .help("Port for iperf tests")
                        .default_value(&defaults[4]),
                )
                .arg(
                    Arg::with_name("min-mtu")
                        .long("min-mtu")
                        .value_name("MIN_MTU")
                        .help("Minimum MTU to test")
                        .default_value(&defaults[0]),
                )
                .arg(
                    Arg::with_name("max-mtu")
                        .long("max-mtu")
                        .value_name("MAX_MTU")
                        .help("Maximum MTU to test")
                        .default_value(&defaults[1]),
                )
                .arg(
                    Arg::with_name("step")
                        .long("step")
                        .value_name("STEP")
                        .help("MTU step size")
                        .default_value(&defaults[2]),
                )
                .arg(
                    Arg::with_name("csv-file")
                        .long("csv-file")
                        .value_name("FILE")
                        .help("Path to CSV output file")
                        .default_value(&defaults[5]),
                ),
        )
        .subcommand(
            SubCommand::with_name("heatmap")
                .about("Generate heatmap from existing log file")
                .arg(
                    Arg::with_name("log-filepath")
                        .long("log-filepath")
                        .value_name("FILE")
                        .help("The filepath to the log file (CSV) for heatmap generation.")
                        .required(true),
                )
                .arg(
                    Arg::with_name("heatmap-filepath")
                        .long("heatmap-filepath")
                        .value_name("FILE")
                        .help("The filepath where the heatmap PNG file will be saved.")
                        .default_value(&defaults[6]),
                ),
        )
}

// Функция для парсинга параметров сервера
pub fn parse_server_params(matches: &clap::ArgMatches) -> TestParameters {
    TestParameters {
        interface: matches.value_of("interface").unwrap().to_string(),
        min_mtu: u32::from_str(matches.value_of("min-mtu").unwrap()).unwrap(),
        max_mtu: u32::from_str(matches.value_of("max-mtu").unwrap()).unwrap(),
        step: u32::from_str(matches.value_of("step").unwrap()).unwrap(),
        control_port: u16::from_str(matches.value_of("server-port").unwrap()).unwrap(),
        iperf_port: u16::from_str(matches.value_of("iperf-port").unwrap()).unwrap(),
    }
}

// Функция для парсинга параметров клиента
pub fn parse_peer_params(matches: &clap::ArgMatches) -> PeerParameters {
    PeerParameters {
        interface: matches.value_of("interface").unwrap().to_string(),
        server_ip: matches.value_of("server-ip").unwrap().to_string(),
        control_port: u16::from_str(matches.value_of("server-port").unwrap()).unwrap(),
        iperf_port: u16::from_str(matches.value_of("iperf-port").unwrap()).unwrap(),
        min_mtu: u32::from_str(matches.value_of("min-mtu").unwrap()).unwrap(),
        max_mtu: u32::from_str(matches.value_of("max-mtu").unwrap()).unwrap(),
        step: u32::from_str(matches.value_of("step").unwrap()).unwrap(),
        csv_file: matches.value_of("csv-file").unwrap().to_string(),
    }
}

// Функция для парсинга параметров клиента
pub fn parse_heatmap_params(matches: &clap::ArgMatches) -> HeatmapParameters {
    HeatmapParameters {
        log_filepath: matches.value_of("log-filepath").unwrap().to_string(),
        heatmap_filepath: matches.value_of("heatmap-filepath").unwrap().to_string(),
    }
}
