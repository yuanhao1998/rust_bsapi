// @Create  : 2023/12/27 16:55
// @Author  : yuanhao1998
// @Remark  : mq数据库操作方法

use crate::codec::encode::Request;
use crate::db::connect_pool::ConnPool;
use crate::db_def::protocol::{BCT_HEAD_ACTYPE_GETMQNAMES, BCT_HEAD_DBTYPE_MQ};
use crate::err::DBError;

struct MQ<'a> {
    host: &'a str,
    port: u8,
}

impl MQ {
    fn new(host: &str, port: u8) -> MQ {
        MQ {host, port}
    }

    fn bs_mq_query_all_names(&self, mq: Vec<&str>){
        let mut conn = ConnPool::global().get_conn(self.host, self.port)?;
        let req = Request::new(BCT_HEAD_DBTYPE_MQ, BCT_HEAD_ACTYPE_GETMQNAMES, 0);

        conn.send_data(req.serialize())?;

        let recv = conn.recv_head();
    }
}
