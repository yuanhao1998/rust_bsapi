// @Create  : 2023/12/21 17:06
// @Author  : yuanhao1998
// @Remark  :

// 数据库类型
pub const BCT_HEAD_DBTYPE_MQ: u16 = 0x1;
pub const BCT_HEAD_DBTYPE_TRDB: u16 = 0x2;
pub const BCT_HEAD_DBTYPE_TDB: u16 = 0x3;
pub const BCT_HEAD_DBTYPE_SYSTEM: u16 = 0x4;

// 通讯协议类型
pub const VERSION: u32 = 16843009;

// 动作类型
pub const BCT_HEAD_ACTYPE_OPENMQ: u32 = 0x0001;
pub const BCT_HEAD_ACTYPE_GETMQNAMES: u32 = 0x0004;

// 请求头长度
pub const BS_SEND_DATA_LEN: usize= 16;

// 响应头长度
pub const BS_RECV_HEAD_LEN: usize = 20;

//每次接收响应数据的默认长度
pub const BS_RECV_DATA_LEN: usize = 20;

// 连接池每个HOST最大连接数
pub const BS_POOL_IP_MAX: usize = 5;
