// @Create  : 2023/12/22 11:02
// @Author  : yuanhao1998
// @Remark  : 数据解析

use std::io;
use crate::db_def::err::BS_NOERROR;
use crate::err::DBError;

#[derive(Clone, Copy)]
struct ResponseHead {
    cb_len: usize,
    data_type: u16,
    err_code: u32,
    add_data: u32,
    add_data_64: u64,
}

struct ResponseData {}

#[derive(Debug)]
struct Response {
    response_head: Option<ResponseHead>,
    response_data: Option<ResponseData>,
}

impl Response {
    fn new() -> Response {
        Response {
            response_head: None,
            response_data: None,
        }
    }

    fn parse_head(&mut self, head: &[u8]) -> Result<ResponseHead, DBError> {
        let mut cursor = io::Cursor::new(head);
        let res = ResponseHead {
            cb_len: cursor.read_u16::<u16>()?,
            data_type: cursor.read_u16::<u16>()?,
            err_code: cursor.read_u32::<u32>()?,
            add_data: cursor.read_u32::<u32>()?,
            add_data_64: cursor.read_u64::<u64>()?,
        };

        if res.err_code != BS_NOERROR {  // 错误码
            Err(DBError::Err(res.err_code))
        }

        self.response_head = Some(res);
        Ok(res)
    }

    fn parse_data(&mut self, data: &[u8]) -> Result<ResponseData, DBError> {
        if self.response_head.is_none() {
            Err(DBError::Valid("未解析response_head"))
        }

        let data = data[8..self.response_head.unwrap().add_data - 1];
        if let Ok(data) = std::str::from_utf8(data) {
            data.split("\0").collect()
        } else {
            Err(DBError::ParesData("数据转为utf-8失败"))
        }
    }
}