use crate::data::models::{
    DEFAULT_CONTROL_PORT, DEFAULT_IPERF_PORT, DEFAULT_MAX_MTU, DEFAULT_MIN_MTU, DEFAULT_STEP,
};
use chrono::Local;
use clap::{Parser, Subcommand};

fn default_csv_filename() -> String {
    format!("wg_mtu_finder_{}.csv", Local::now().format("%Y%m%dT%H%M%S"))
}

fn default_heatmap_filename() -> String {
    format!("heatmap_{}.png", Local::now().format("%Y%m%dT%H%M%S"))
}

#[derive(Parser)]
#[command(
    name = "WireGuard MTU Finder",
    version = "1.0",
    author = "MrSomFeRGO",
    about = "Utility to find optimal MTU for WireGuard connections"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run in server mode
    Server {
        /// WireGuard interface name
        #[arg(short, long, value_name = "INTERFACE")]
        interface: String,

        /// Minimum MTU to test
        #[arg(long, value_name = "MIN_MTU", default_value_t = DEFAULT_MIN_MTU)]
        min_mtu: u32,

        /// Maximum MTU to test
        #[arg(long, value_name = "MAX_MTU", default_value_t = DEFAULT_MAX_MTU)]
        max_mtu: u32,

        /// MTU step size
        #[arg(long, value_name = "STEP", default_value_t = DEFAULT_STEP)]
        step: u32,

        /// Control connection port
        #[arg(long, value_name = "PORT", default_value_t = DEFAULT_CONTROL_PORT)]
        server_port: u16,

        /// Port for iperf tests
        #[arg(long, value_name = "PORT", default_value_t = DEFAULT_IPERF_PORT)]
        iperf_port: u16,
    },
    /// Run in peer mode
    Peer {
        /// WireGuard interface name
        #[arg(short, long, value_name = "INTERFACE")]
        interface: String,

        /// Server IP address
        #[arg(long, value_name = "SERVER_IP")]
        server_ip: String,

        /// Control connection port
        #[arg(long, value_name = "PORT", default_value_t = DEFAULT_CONTROL_PORT)]
        server_port: u16,

        /// Port for iperf tests
        #[arg(long, value_name = "PORT", default_value_t = DEFAULT_IPERF_PORT)]
        iperf_port: u16,

        /// Minimum MTU to test
        #[arg(long, value_name = "MIN_MTU", default_value_t = DEFAULT_MIN_MTU)]
        min_mtu: u32,

        /// Maximum MTU to test
        #[arg(long, value_name = "MAX_MTU", default_value_t = DEFAULT_MAX_MTU)]
        max_mtu: u32,

        /// MTU step size
        #[arg(long, value_name = "STEP", default_value_t = DEFAULT_STEP)]
        step: u32,

        /// Path to CSV output file
        #[arg(long, value_name = "FILE", default_value_t = default_csv_filename())]
        csv_file: String,
    },
    /// Generate heatmap from existing log file
    Heatmap {
        /// The filepath to the log file (CSV) for heatmap generation
        #[arg(long, value_name = "FILE")]
        log_filepath: String,

        /// The filepath where the heatmap PNG file will be saved
        #[arg(long, value_name = "FILE", default_value_t = default_heatmap_filename())]
        heatmap_filepath: String,
    },
}
