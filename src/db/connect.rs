// @Create  : 2023/12/27 17:37
// @Author  : yuanhao1998
// @Remark  :

use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::db_def::protocol::{BS_RECV_HEAD_LEN, BS_RECV_DATA_LEN};

pub struct Connect<'a> {
    host: &'a str,
    port: u8,
    socket: TcpStream,
}

impl Connect {
    pub fn new(host: &str, port: u8) -> Result<Connect, io::Error> {
        let mut socket = TcpStream::connect(format!("{}:{}", host, port))?;

        Ok(Connect {
            host,
            port,
            socket,
        })
    }

    pub fn send_data(&mut self, data: &[u8]) -> Result<(), io::Error> {
        self.socket.write_all(data)
    }

    pub fn recv_head(&mut self) -> Result<&[u8], io::Error> {
        return self.recv_data(BS_RECV_HEAD_LEN);
    }

    pub fn recv_data(&mut self, data_len: usize) -> Result<&[u8], io::Error> {
        let mut data = vec![];
        let mut buffer = [0; BS_RECV_DATA_LEN];
        let mut recv_len: usize = 0;
        while data_len > recv_len {
            self.socket.read(&mut buffer)?;
            data.push(buffer);
            recv_len += BS_RECV_DATA_LEN
        }
        Ok(&buffer)
    }
}