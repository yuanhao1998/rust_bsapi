// @Create  : 2023/12/21 17:06
// @Author  : yuanhao1998
// @Remark  :
pub const BCT_HEAD_DBTYPE_MQ: u32 = 0x1;
pub const BCT_HEAD_DBTYPE_TRDB: u32 = 0x2;
pub const BCT_HEAD_DBTYPE_TDB: u32 = 0x3;
pub const BCT_HEAD_DBTYPE_SYSTEM: u32 = 0x4;

// 动作类型
pub const BCT_HEAD_ACTYPE_OPENMQ: u32 = 0x0001;

// 请求长度
pub const BS_SEND_DATA_LEN: usize= 16;

// 响应头长度
pub const BS_RECV_HEAD_LEN: usize = 20;

//每次接收响应数据的默认长度
pub const BS_RECV_DATA_LEN: usize = 20;
