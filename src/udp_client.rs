use std::net::UdpSocket;
use std::{io, str};

//Udpは通信相手が、本当に存在するという確認をしない
pub fn communicate(address: &str) -> Result<(), failure::Error> {
    //エラーを防ぐ(ポート番号を0にするとOSが空いているポートを割り当ててくれる)
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send_to(input.as_bytes(), address)?;

        //指定したバイトを超えるデータは破棄
        let mut buffer = [0u8; 126];

        //Socketからデータを受信
        socket.recv_from(&mut buffer).expect("failed to receive");
        print!("{}", str::from_utf8(&buffer).expect("failed to convert"));
    }
}
