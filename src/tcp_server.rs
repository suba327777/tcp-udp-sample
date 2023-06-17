use log::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::time::Instant;
use std::{str, thread};

//指定のソケットアドレスで接続を待ち受ける
pub fn serve(address: &str) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(address)?;
    loop {
        let (stream, _) = listener.accept()?;

        //スレッドを立ち上げて接続に対処する-> 複数のクライアントからのコネクションを同時に処理
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });

        //1対1通信
        // handler(stream).unwrap_or_else(|error| error!("{:?}", error));
    }
}

//クライアントからの入力を待ち受け、受信したら同じものを返却する
fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    //接続元のアドレスを取得
    debug!("handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    let mut total_bytes = 0;
    let mut total_elapsed_time = Duration::default();
    loop {
        let start_time = Instant::now();
        let nbytes = stream.read(&mut buffer)?;
        total_bytes += nbytes;

        //クライアントの接続が切れたら終了
        if nbytes == 0 {
            dbg!("connection closed.");
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer[..nbytes])?);
        //受信データを返却
        stream.write_all(&buffer[..nbytes])?;
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        total_elapsed_time += elapsed_time;

        //受信速度
        let bits_per_second = (total_bytes as f64 * 8.0) / total_elapsed_time.as_secs_f64();
        println!(" {:?} bits/s", bits_per_second);
    }
}
