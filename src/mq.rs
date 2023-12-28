// @Create  : 2023/12/27 17:22
// @Author  : yuanhao1998
// @Remark  :

struct MQ<'a> {
    host: &'a str,
    port: u8,
    pwd: Option<&'a str>,
}

impl Default for MQ {
    fn default() -> Self {
        Self {
            host: "",
            port: 0,
            pwd: None,
        }
    }
}


impl MQ {
    fn new(host: &str, port: u8, pwd: Option<&str>) -> MQ {
        let mut mq = MQ::default();
        mq.host = host;
        mq.port = port;
        if pwd != None { mq.pwd = pwd }
        mq
    }

    fn mq_list(&self) {}
}
