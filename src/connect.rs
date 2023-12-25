// @Create  : 2023/12/20 16:04
// @Author  : yuanhao1998
// @Remark  : 数据库连接

use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::db_def::protocol::{BS_RECV_HEAD_LEN, BS_RECV_DATA_LEN};

#[derive(Debug)]
pub struct Connect<'a> {
    pub host: &'a str,
    pub port: u8,
    pub conn: TcpStream,
}

impl Connect {
    fn new(host: &str, port: u8) -> Result<Connect, io::Error> {
        let mut conn = TcpStream::connect(format!("{}:{}", host, port))?;

        Ok(Connect {
            host,
            port,
            conn,
        })
    }

    // 发送请求数据
    fn send_data(&mut self, data: &[u8]) -> Result<(), io::Error> {
        self.conn.write_all(data)
    }

    // 接受请求头
    fn recv_head(&mut self) -> Result<&[u8], io::Error> {
        return self.recv_data(BS_RECV_HEAD_LEN)
    }

    // 接受请求内容
    fn recv_data(&mut self, data_len: usize) -> Result<&[u8], io::Error> {
        let mut data = vec![];
        let mut buffer = [0; BS_RECV_DATA_LEN];
        let mut recv_len: usize = 0;
        while data_len > recv_len {
            self.conn.read(&mut buffer)?;
            data.push(buffer);
            recv_len += BS_RECV_DATA_LEN
        }
        Ok(&buffer)
    }
}
