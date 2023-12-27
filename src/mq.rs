// @Create  : 2023/12/27 17:22
// @Author  : yuanhao1998
// @Remark  :

struct MQ<'a> {
    host: &'a str,
    port: u8,
    pwd: &'a str,
}

impl MQ {

    // 数据库默认密码都为空，因此new方法默认不需要传递pwd，如有特殊情况，可以使用 new_with_pwd
    fn new(host: &str, port: u8) -> MQ {
        let pwd = "";
        MQ {
            host,
            port,
            pwd,
        }
    }

    fn new_with_pwd(host: &str, port: u8, pwd: &str) -> MQ {
        MQ {
            host,
            port,
            pwd,
        }
    }

    fn mq_list(&self) {

    }
}
