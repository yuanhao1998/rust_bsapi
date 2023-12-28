// @Create  : 2023/12/28 09:52
// @Author  : yuanhao1998
// @Remark  : 连接池

use std::collections::{HashMap, VecDeque};
use std::sync::OnceLock;

use crate::db::connect::Connect;
use crate::db_def::protocol::BS_POOL_IP_MAX;
use crate::err::DBError;

// 全局变量
pub static CONN_POOL: OnceLock<ConnPool> = OnceLock::new();

pub struct ConnPool<'a> {
    socket_map: HashMap<&'a str, VecDeque<Connect<'a>>>,
}

impl ConnPool {
    pub fn global() -> &'static ConnPool {
        CONN_POOL.get_or_init(ConnPool{ socket_map: Default::default() })
    }

    fn init(&mut self, host: &str, port: u8) {
        let key = format!("{}:{}", host, port);
        if !self.socket_map.contains_key(&key) {
            let mut deque = VecDeque::new();
            for _ in 0..BS_POOL_IP_MAX {
                let conn = Connect::new(host, port)?;
                deque.push_back(conn)
            }
            self.socket_map.insert(&key, deque)
        }
    }

    pub fn new_conn(host: &str, port: u8) -> Result<Connect, DBError> {
        Connect::new(host, port)
    }

    pub fn get_conn(&mut self, host: &str, port: u8) -> Option<Connect> {
        let key = format!("{}:{}", host, port);
        if !self.socket_map.get(&key) {
            self.init(host, port)
        }
        self.socket_map.get(&key).unwrap().pop_front()
    }

    pub fn release_conn(&mut self, host: &str, port: u8, conn: Connect) -> Result<(), DBError> {
        let key = format!("{}:{}", host, port);

        match &mut self.socket_map.get(&key) {
            None => { Err(DBError::Internal(&*format!("释放了一个未知的连接，找不到此连接对应存放的host：{}, port：{}", host, port))) }
            Some(data) => { data.push_back(conn) }
        }
    }
}
