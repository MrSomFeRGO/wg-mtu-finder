mod cli;
mod data;
mod heatmap;
mod mtu_testing;
mod network;
mod utils;

use crate::cli::{Cli, Commands};
use crate::heatmap::generate_heatmap;
use crate::mtu_testing::{run_peer, run_server};
use clap::Parser;
use crate::data::models::{HeatmapParameters, PeerParameters, TestParameters};

fn main() {
    // Парсим аргументы командной строки
    let cli = Cli::parse();

    // Запускаем соответствующий режим
    match &cli.command {
        Commands::Server {
            interface,
            min_mtu,
            max_mtu,
            step,
            server_port,
            iperf_port,
        } => {
            run_server(TestParameters {
                interface: interface.clone(),
                min_mtu: *min_mtu,
                max_mtu: *max_mtu,
                step: *step,
                control_port: *server_port,
                iperf_port: *iperf_port,
            });
        }
        Commands::Peer {
            interface,
            server_ip,
            server_port,
            iperf_port,
            min_mtu,
            max_mtu,
            step,
            csv_file,
        } => {
            run_peer(PeerParameters {
                interface: interface.clone(),
                server_ip: server_ip.clone(),
                control_port: *server_port,
                iperf_port: *iperf_port,
                min_mtu: *min_mtu,
                max_mtu: *max_mtu,
                step: *step,
                csv_file: csv_file.clone(),
            });
        }
        Commands::Heatmap {
            log_filepath,
            heatmap_filepath,
        } => {
            generate_heatmap(HeatmapParameters {
                log_filepath: log_filepath.clone(),
                heatmap_filepath: heatmap_filepath.clone(),
            }).unwrap();
        }
    }
}
