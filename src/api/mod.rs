use encoding_rs::EUC_KR;
use hex::FromHex;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::net::TcpStream;
use std::path::Path;

use crate::utils::response::Success;

fn read_buffer(mut stream: &TcpStream, size: usize) -> Vec<u8> {
  let mut buffer = vec![0 as u8; size];
  match stream.read_exact(&mut buffer) {
    Ok(_) => {}
    Err(e) => {
      println!("Failed to read buffer: {}", e);
    }
  };
  return buffer;
}

fn decode_to_utf8(buffer: Vec<u8>) -> String {
  let mut utf8 = String::new();
  let (cow, _, _) = EUC_KR.decode(&buffer);
  let read = &cow[..].to_string();
  utf8.push_str(read);

  return utf8.replace("\u{0}", " ");
}

fn encode_to_euckr(fixed_length: &str, size: usize) -> Vec<u8> {
  let mut buffer = vec![0 as u8; size];
  let (cow, _, _) = EUC_KR.encode(&fixed_length);
  let val = &cow[..];
  let vec = val.to_vec();
  buffer.splice(..vec.len(), vec);

  return buffer;
}

fn set_res(stream: &TcpStream, buffer: Vec<u8>) {
  let mut writer = BufWriter::new(stream);
  writer.write_all(&buffer[..]).expect("could not write");
  writer.flush().expect("could not flush");
}

fn test_1(stream: &TcpStream) {
  let id = encode_to_euckr("01", 2);
  let seq = read_buffer(&stream, 5);
  let extra_id = read_buffer(&stream, 20);
  let code = encode_to_euckr(Success::res().code.as_str(), 2);
  let msg = encode_to_euckr(Success::res().msg.as_str(), 255);
  let res = [id, seq, extra_id, code, msg].concat();
  set_res(&stream, res);
}

fn test_2(stream: &TcpStream) {
  let id = encode_to_euckr("02", 2);
  let seq = read_buffer(&stream, 5);
  let return_id = read_buffer(&stream, 20);
  let return_number = read_buffer(&stream, 19);
  let extra_id = read_buffer(&stream, 20);
  let msg_type = read_buffer(&stream, 2);
  let code = encode_to_euckr(Success::res().code.as_str(), 2);
  let msg = encode_to_euckr(Success::res().msg.as_str(), 255);
  let res = [
    id,
    seq,
    return_id,
    return_number,
    extra_id,
    msg_type,
    code,
    msg,
  ]
  .concat();
  set_res(&stream, res);
}

fn test_3(stream: &TcpStream) {
  let id = encode_to_euckr("03", 2);
  let seq = read_buffer(&stream, 5);
  let extra_id = read_buffer(&stream, 20);

  let encrypted_data = Vec::from_hex(env::var("CUSTOMER_INFO").unwrap()).unwrap();

  let data_len = encode_to_euckr(encrypted_data.len().to_string().as_str(), 5);

  let code = encode_to_euckr(Success::res().code.as_str(), 2);
  let msg = encode_to_euckr(Success::res().msg.as_str(), 255);

  let res = [id, seq, extra_id, data_len, encrypted_data, code, msg].concat();
  set_res(&stream, res);
}

fn test_4(stream: &TcpStream) {
  let path = Path::new("./4_2.txt");
  let mut file = File::open(&path).unwrap();

  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer).unwrap();

  let mut writer = BufWriter::new(stream);
  writer.write_all(&*buffer).expect("could not write");
  writer.flush().expect("could not flush")
}

fn test_5(stream: &TcpStream) {
  let id = encode_to_euckr("05", 2);
  let seq = read_buffer(&stream, 5);
  let code = encode_to_euckr(Success::res().code.as_str(), 2);
  let msg = encode_to_euckr(Success::res().msg.as_str(), 255);
  let res = [id, seq, code, msg].concat();
  set_res(&stream, res);
}

fn test_6(stream: &TcpStream) {
  let id = encode_to_euckr("06", 2);
  let seq = read_buffer(&stream, 5);
  let code = encode_to_euckr(Success::res().code.as_str(), 2);
  let msg = encode_to_euckr(Success::res().msg.as_str(), 255);
  let res = [id, seq, code, msg].concat();
  set_res(&stream, res);
}

fn test_7(stream: &TcpStream) {
  let id = encode_to_euckr("07", 2);
  let seq = read_buffer(&stream, 5);
  let code = encode_to_euckr(Success::res().code.as_str(), 2);
  let msg = encode_to_euckr(Success::res().msg.as_str(), 255);
  let res = [id, seq, code, msg].concat();
  set_res(&stream, res);
}

pub fn router(stream: &TcpStream) {
  let buffer = read_buffer(&stream, 2);
  let route = decode_to_utf8(buffer);
  match &*route {
    "01" => test_1(&stream),
    "02" => test_2(&stream),
    "03" => test_3(&stream),
    "04" => test_4(&stream),
    "05" => test_5(&stream),
    "06" => test_6(&stream),
    "07" => test_7(&stream),
    _ => println!("{:?}", route),
  }
}
