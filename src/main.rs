use std::io::{self, BufReader};
use std::net::UdpSocket;
use serde::{Serialize, Deserialize};
use groqcloud::{Client, Llama};

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    let client = Client::new("https://api.groqcloud.com/v1")?;
    let llama = Llama::new("llama-3.1-8b-instant")?;
    loop {
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;
        let text = std::str::from_utf8(&buf[..amt])?;
        let response = client.call(llama, text)?;
        println!("{}", response);
    }
}

== ЧТЕНИЕ ФАЙЛА ==
ACTION:READ_FILE
PATH:modelling/groqcloud/README.md

== ЧТЕНИЕ ФАЙЛА ==
ACTION:READ_FILE
PATH:modelling/groqcloud/groqcloud.rs