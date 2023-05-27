use log::*;
use std::net::UdpSocket;
use std::str;

//udpは1つのソケットですべてのクライアントを処理するので、threadを立ち上げる必要がない
pub fn serve(address: &str) -> Result<(), failure::Error> {
    let server_socket = UdpSocket::bind(address)?;

    loop {
        //指定したバイトを超えるデータは破棄
        let mut buf = [0u8; 256];
        //UDPソケットからデータを受信
        let (size, src) = server_socket.recv_from(&mut buf)?;
        //受信したデータの送信元アドレスを表示
        debug!("handling data from {}", src);
        print!("{}", str::from_utf8(&buf[..size])?);
        //受信データを送信
        server_socket.send_to(&buf, src)?;
    }
}
