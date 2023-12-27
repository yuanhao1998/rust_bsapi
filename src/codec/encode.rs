// @Create  : 2023/12/22 11:02
// @Author  : yuanhao1998
// @Remark  : 请求数据序列化

use std::io;
use crate::db_def::protocol::BS_SEND_DATA_LEN;

#[derive(Debug)]
pub struct Request{
    pub head_len: u16,
    pub version: u32,
    pub db_type: u16,
    pub action_type: u32,
    pub add: u32
}

impl Request {
    fn new(head_len: u16, version: u32, db_type: u16, action_type: u32, add: u32) -> Request{
        Request{
            head_len,
            version,
            db_type,
            action_type,
            add
        }
    }

    fn serialize<W: io::Write>(&self) -> Result<W, io::Error>{
        let mut req: [u8; BS_SEND_DATA_LEN] = [0; BS_SEND_DATA_LEN];
        req.write_all(&self.head_len.to_le_bytes())?;
        req.write_all(&self.version.to_le_bytes())?;
        req.write_all(&self.db_type.to_le_bytes())?;
        req.write_all(&self.action_type.to_le_bytes())?;
        req.write_all(&self.add.to_le_bytes())?;
        Ok(req)
    }
}