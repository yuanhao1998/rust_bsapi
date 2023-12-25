// @Create  : 2023/12/22 11:02
// @Author  : yuanhao1998
// @Remark  : 数据解析

use std::io;
use std::io::Cursor;

enum ResponseDataType {
    Head,
    Body,
}

struct ResponseData {
    cb_len: u16,
    data_type: u16,
    err_code: u32,
    add_data: u32,
    add_data_64: u64
}

#[derive(Debug)]
struct Response<'a> {
    data: &'a [u8],
    data_type: ResponseDataType,
}

impl Response {
    fn new(data: &[u8], data_type: ResponseDataType) -> Response {
        Response {
            data,
            data_type,
        }
    }

    fn parse(&self) -> Result<ResponseData, io::Error> {
        let mut cursor = Cursor::new(self.data);
        Ok(ResponseData {
            cb_len: cursor.read_u16::<u16>()?,
            data_type: cursor.read_u16::<u16>()?,
            err_code: cursor.read_u32::<u32>()?,
            add_data: cursor.read_u32::<u32>()?,
            add_data_64: cursor.read_u64::<u64>()?,
        })
    }
}