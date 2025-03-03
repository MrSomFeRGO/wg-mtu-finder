use std::net::TcpStream;
use crate::data::models::{MtuTestResult, PeerParameters};
use crate::network::messages::{Message, send_message, receive_message};
use crate::network::iperf::{check_iperf_installed, run_iperf_test};
use crate::network::mtu::{get_remote_mtu, set_mtu};
use crate::utils::csv_utils::{create_csv_file, save_result_to_csv};

pub fn run_peer(params: PeerParameters) {
    // Проверяем наличие iperf3
    if !check_iperf_installed() {
        eprintln!("Error: iperf3 is not installed or not in PATH");
        return;
    }

    // Создаем CSV файл для результатов
    let mut writer = create_csv_file(&params.csv_file);

    // Подключаемся к серверу
    println!("Connecting to server {}:{}", params.server_ip, params.control_port);
    let addr = format!("{}:{}", params.server_ip, params.control_port);
    let mut stream = match TcpStream::connect(&addr) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Failed to connect to server: {}", e);
            return;
        }
    };

    // Основной цикл тестирования
    loop {
        // Получаем сообщение о готовности сервера
        match receive_message::<Message>(&mut stream) {
            Ok(Message::ServerReady) => {
                println!("Server is ready for testing");
            },
            Ok(Message::Finish) => {
                println!("Server signals testing is complete");
                break;
            },
            Ok(_) => {
                println!("Unexpected message from server");
                break;
            },
            Err(e) => {
                println!("Error receiving message from server: {}", e);
                break;
            }
        }

        // Получаем текущий MTU сервера
        let server_mtu = match get_remote_mtu(&mut stream, &params.interface) {
            Ok(mtu) => {
                println!("Server MTU: {}", mtu);
                mtu
            },
            Err(e) => {
                println!("Could not get server MTU: {}, using default: {}", e, params.max_mtu);
                params.max_mtu
            }
        };

        // Тестирование с разными MTU на стороне клиента
        run_client_side_tests(
            &params,
            server_mtu,
            &mut writer,
        );

        // Сообщаем серверу о завершении цикла тестов
        send_message(&mut stream, Message::PeerDone);
        println!("Sent PeerDone message to server");
    }

    // Завершение и сохранение результатов
    println!("Results saved to {}", params.csv_file);
}

// Функция для запуска клиентских тестов с разными MTU
fn run_client_side_tests(
    params: &PeerParameters,
    server_mtu: u32,
    writer: &mut csv::Writer<std::fs::File>,
) {
    let mut client_mtu = params.max_mtu;

    while client_mtu >= params.min_mtu {
        println!("Testing with client MTU: {}", client_mtu);

        // Установить MTU на интерфейсе
        set_mtu(&params.interface, client_mtu);

        // Выполнить тесты скорости
        let test_results = run_speed_tests(&params.server_ip, params.iperf_port);

        // Сохранить результаты, если они получены
        if let Some((upload, download)) = test_results {
            let result = MtuTestResult {
                server_mtu,
                client_mtu,
                upload_rcv_mbps: upload.0,
                upload_send_mbps: upload.1,
                download_rcv_mbps: download.0,
                download_send_mbps: download.1,
            };

            // Вывод результатов
            println!("Results: {:?}", &result);

            // Сохранение в CSV
            save_result_to_csv(writer, &result);
        }

        // Уменьшить MTU для следующего цикла
        client_mtu -= params.step;
    }
}

// Функция для запуска тестов скорости
fn run_speed_tests(server_ip: &str, iperf_port: u16) -> Option<((f64, f64), (f64, f64))> {
    // Выполнить тест скорости upload
    println!("Running upload test...");
    let upload_result = run_iperf_test(server_ip, iperf_port, false);

    // Выполнить тест скорости download
    println!("Running download test...");
    let download_result = run_iperf_test(server_ip, iperf_port, true);

    match (upload_result, download_result) {
        (Some(upload), Some(download)) => Some((upload, download)),
        _ => None
    }
}