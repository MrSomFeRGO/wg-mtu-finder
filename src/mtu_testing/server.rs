use std::net::{SocketAddr, TcpListener};
use crate::data::models::TestParameters;
use crate::messages::{Message, send_message, receive_message};
use crate::network::iperf::check_iperf_installed;
use crate::network::iperf::start_iperf_server;
use crate::network::mtu::set_mtu;

pub fn run_server(params: TestParameters) {
    // Проверяем наличие iperf3
    if !check_iperf_installed() {
        eprintln!("Error: iperf3 is not installed or not in PATH");
        return;
    }

    // Запустить iperf сервер
    let mut iperf_process = start_iperf_server(params.iperf_port);
    println!("Started iperf3 server on port {}", params.iperf_port);

    // Настроить сервер для контрольных сообщений
    let addr = SocketAddr::from(([0, 0, 0, 0], params.control_port));
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");
    println!("Server listening on port {}", params.control_port);

    let mut current_mtu = params.max_mtu;
    let min_mtu = params.min_mtu;
    let mut client_done = false;

    // Принимаем первое соединение от клиента
    println!("Waiting for peer connection...");
    let (mut stream, client_addr) = listener.accept().expect("Failed to accept connection");
    println!("Peer connected from: {}", client_addr);

    // Основной цикл тестирования MTU
    while current_mtu >= min_mtu && !client_done {
        println!("Testing with server MTU: {}", current_mtu);

        // Установить MTU на интерфейсе
        set_mtu(&params.interface, current_mtu);

        // Отправить сообщение о готовности сервера
        send_message(&mut stream, Message::ServerReady);
        println!("Sent ServerReady message to peer");

        // Отправляем текущее значение MTU клиенту
        send_message(&mut stream, Message::MtuValue(current_mtu));

        // Ждать завершения тестов со стороны пира
        match receive_message::<Message>(&mut stream) {
            Ok(Message::PeerDone) => {
                println!("Peer completed tests for server MTU {}", current_mtu);

                // Уменьшаем MTU для следующего теста
                current_mtu -= params.step;

                // Если достигли минимального MTU, отправляем сигнал о завершении
                if current_mtu < min_mtu {
                    println!("All tests completed, sending Finish signal to peer");
                    send_message(&mut stream, Message::Finish);
                    client_done = true;
                }
            },
            Ok(_) => {
                println!("Unexpected message from peer");
                break;
            },
            Err(e) => {
                println!("Error receiving message from peer: {}", e);
                break;
            }
        }
    }

    println!("All MTU tests completed");
    // Завершить процесс iperf
    iperf_process
        .kill()
        .expect("Failed to kill iperf3 server process");
}