use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PageData<T> where T: Serialize + Clone + Default + Send + Sync {
    page_no: u32,
    page_size: u32,
    total: u32,
    data: Vec<T>,
}

impl<T> PageData<T> where T: Serialize + Clone + Default + Send + Sync {
    pub fn new(page_no: u32, page_size: u32, total: u32, data: Vec<T>) -> Self {
        PageData {
            page_no,
            page_size,
            total,
            data,
        }
    }
    pub fn get_page_no(&self) -> &u32 {
        &self.page_no
    }
    pub fn get_page_size(&self) -> &u32 {
        &self.page_size
    }
    pub fn get_total(&self) -> &u32 {
        &self.total
    }
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }
    pub fn get_mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
    pub fn set_page_no(&mut self, page_no: u32) -> &mut Self {
        self.page_no = page_no;
        self
    }
    pub fn set_page_size(&mut self, page_size: u32) -> &mut Self {
        self.page_size = page_size;
        self
    }
    pub fn set_total(&mut self, total: u32) -> &mut Self {
        self.total = total;
        self
    }
    pub fn set_data(&mut self, data: Vec<T>) -> &mut Self {
        self.data = data;
        self
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RespVo<T> where T: Serialize + Clone + Default + Send + Sync {
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T> RespVo<T> where T: Serialize + Clone + Default + Send + Sync {
    pub fn get_code(&self) -> &i32 {
        &self.code
    }
    pub fn get_msg(&self) -> &String {
        &self.msg
    }
    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
    pub fn get_mut_data(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }
}

impl<T> RespVo<T> where T: Serialize + Clone + Default + Send + Sync {
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        RespVo {
            code,
            msg,
            data,
        }
    }
    pub fn ok_data_msg(data: Option<T>, msg: String) -> Self {
        RespVo {
            code: 0,
            msg,
            data,
        }
    }
    pub fn fail_msg(msg: String) -> Self {
        RespVo {
            code: -1,
            msg,
            data: None,
        }
    }
}


#[async_trait]
impl<T> Writer for RespVo<T>
    where
        T: Clone + Default + Serialize + Send + Sync,
{
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self))
    }
}
