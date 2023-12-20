// @Create  : 2023/12/20 16:04
// @Author  : yuanhao1998
// @Remark  : 数据库连接

struct Connect {
    host: String,
    port: u8
}

impl Connect {
    fn new(host: String, port: u8) -> Connect{
        Connect{
            host,
            port
        }
    }
}
