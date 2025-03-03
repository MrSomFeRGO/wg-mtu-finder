use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    ServerReady,
    PeerDone,
    Finish,
    MtuValue(u32),
}

// Функция для отправки сообщения
pub fn send_message<T: Serialize>(stream: &mut TcpStream, message: T) {
    let serialized = serde_json::to_string(&message).expect("Failed to serialize message");
    let len = serialized.len() as u32;
    let len_bytes = len.to_be_bytes();

    stream
        .write_all(&len_bytes)
        .expect("Failed to write message length");
    stream
        .write_all(serialized.as_bytes())
        .expect("Failed to write message");
    stream.flush().expect("Failed to flush stream");
}

// Функция для получения сообщения
pub fn receive_message<T: for<'de> Deserialize<'de>>(stream: &mut TcpStream) -> io::Result<T> {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes)?;
    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer)?;

    let message: T = serde_json::from_slice(&buffer)?;
    Ok(message)
}
