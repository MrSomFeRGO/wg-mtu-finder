mod cli;
mod data;
mod heatmap;
mod messages;
mod mtu_testing;
mod network;
mod utils;

use crate::cli::parse_args::{create_cli_app, create_default_values, parse_heatmap_params, parse_peer_params, parse_server_params};
use crate::heatmap::generate_heatmap;
use crate::mtu_testing::{run_peer, run_server};

fn main() {
    // Создаем параметры по умолчанию
    let default_values = create_default_values();

    // Парсим аргументы командной строки
    let matches = create_cli_app(&default_values).get_matches();

    // Запускаем соответствующий режим
    match matches.subcommand() {
        ("server", Some(server_matches)) => {
            let params = parse_server_params(server_matches);
            run_server(params);
        }
        ("peer", Some(peer_matches)) => {
            let params = parse_peer_params(peer_matches);
            run_peer(params);
        }
        ("heatmap", Some(heatmap_matches)) => {
            let params = parse_heatmap_params(heatmap_matches);
            generate_heatmap(params).unwrap();
        }
        _ => {
            println!("Please specify either 'server', 'peer' or 'heatmap' mode.");
            println!("Use --help for more information.");
        }
    }
}
