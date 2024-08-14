use bmbp_http_type::{PageData, RespVo};

#[test]
fn test_resp() {
    let resp = RespVo::<String>::new(0, "success".to_string(), Some("hello world".to_string()));
    let resp2 = RespVo::<Vec<String>>::new(0, "success".to_string(), Some(vec!["ddd".to_string()]));
    let resp3 = RespVo::<PageData<Vec<String>>>::new(0, "success".to_string(), Some(PageData::default()));
}