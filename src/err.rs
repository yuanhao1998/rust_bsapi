// @Create  : 2023/12/25 15:34
// @Author  : yuanhao1998
// @Remark  : 错误枚举

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DBError<'a> {
    // 网络错误
    Network(&'a str),

    // 响应头解析错误
    ParesHead(&'a str),

    // 响应数据解析错误
    ParesData(&'a str),

    // 验证错误
    Valid(&'a str),

    // 格式错误
    Format(&'a str),

    // 内部错误
    Internal(&'a str),

    // 数据库错误码
    Err(u32)
}

impl fmt::Display for DBError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBError::Network(err_mes) => write!(f, "网络错误：{}", err_mes),
            DBError::ParesHead(err_msg) => write!(f, "解析响应头出错：{}", err_msg),
            DBError::ParesData(err_msg) => write!(f, "解析响应数据出错：{}", err_msg),
            DBError::Valid(err_msg) => write!(f, "验证错误：{}", err_msg),
            DBError::Format(err_msg) => write!(f, "格式错误：{}", err_msg),
            DBError::Internal(err_msg) => write!(f, "内部错误：{}", err_msg),
            DBError::Err(err_code) => write!(f, "错误码：{}", err_code)
        }
    }
}

impl Error for DBError {}
