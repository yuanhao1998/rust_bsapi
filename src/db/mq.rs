// @Create  : 2023/12/27 16:55
// @Author  : yuanhao1998
// @Remark  : mq数据库操作方法

use crate::db::connect::Connect;
use crate::err::DBError;

struct MQ<'a> {
    host: &'a str,
    port: u8,
    conn: Connect<'a>,
}

impl MQ {
    fn new(host: &str, port: u8) -> MQ {
        let connect = Connect::new(host, port);
        match connect {
            Ok(conn) => {
                MQ {
                    host,
                    port,
                    conn,
                }
            }
            Err(e) => {Err(DBError::Network(&*format!("连接host:{}, port{}失败, 错误详情: {}", host, port, e)))}
        }

    }
    fn bs_mq_query_all_names(mq: Vec<&str>) {}
}
