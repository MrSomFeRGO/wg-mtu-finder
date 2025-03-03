use crate::data::models::MtuTestResult;
use csv::Writer;
use std::fs::File;

// Функция для создания CSV файла
pub fn create_csv_file(filename: &str) -> Writer<File> {
    let file = File::create(filename).expect("Failed to create CSV file");
    let mut writer = Writer::from_writer(file);

    // Записываем заголовок
    writer
        .write_record(&[
            "server_mtu",
            "client_mtu",
            "upload_rcv_mbps",
            "upload_send_mbps",
            "download_rcv_mbps",
            "download_send_mbps",
        ])
        .expect("Failed to write CSV header");

    let file = writer.into_inner().expect("Failed to get file from writer");
    let writer = Writer::from_writer(file);

    writer
}

// Функция для сохранения результата в CSV
pub fn save_result_to_csv(writer: &mut Writer<File>, result: &MtuTestResult) {
    writer
        .write_record(&[
            result.server_mtu.to_string(),
            result.client_mtu.to_string(),
            result.upload_rcv_mbps.to_string(),
            result.upload_send_mbps.to_string(),
            result.download_rcv_mbps.to_string(),
            result.download_send_mbps.to_string(),
        ])
        .expect("Failed to write CSV record");

    // Flush после каждой записи
    writer.flush().expect("Failed to flush CSV writer");
}