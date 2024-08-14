use salvo::{async_trait, Depot, Request, Response, Writer};
use salvo::http::ParseError;
use salvo::prelude::Json;
use serde::Serialize;
use crate::RespVo;

type BmbpResp<T> = Result<T, BmbpRespErr>;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BmbpRespErr {
    kind: Option<String>,
    msg: Option<String>,
    cause: Option<String>,
}

impl BmbpRespErr {
    pub fn err(kind: Option<String>, msg: Option<String>) -> Self {
        BmbpRespErr {
            kind,
            msg,
            cause: None,
        }
    }
    pub fn err_case(kind: Option<String>, msg: Option<String>, cause: Option<String>) -> Self {
        BmbpRespErr {
            kind,
            msg,
            cause,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BmbpRespErrCase {
    file: Option<String>,
    line: Option<u32>,
    struct_: Option<String>,
    func_: Option<String>,
}

#[async_trait]
impl Writer for BmbpRespErr {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let msg = self.msg.unwrap_or("".to_string());
        let kind = self.kind.unwrap_or("".to_string());
        let vo: RespVo<String> = if kind.is_empty() {
            RespVo::fail_msg(format!("{}", msg))
        } else {
            RespVo::fail_msg(format!("[{}]{}", kind, msg))
        };
        res.render(Json(vo));
    }
}
impl From<ParseError> for BmbpRespErr {
    fn from(value: ParseError) -> Self {
        BmbpRespErr {
            kind: Some("HTTP".to_string()),
            msg: Some(value.to_string()),
            cause: None,
        }
    }
}
