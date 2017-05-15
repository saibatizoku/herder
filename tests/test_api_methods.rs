extern crate herder;
extern crate hyper;

use herder::api::APIMethodRequest;
use hyper::Method::{Get};
use hyper::{Headers, Uri};
use std::str::FromStr;

#[test]
fn default_api_method() {
    let default = APIMethodRequest {
        body: None,
        headers: Headers::new(),
        method: Get,
        query: None,
        uri: Uri::from_str("/").unwrap()
    };
    assert_eq!(default, APIMethodRequest::default())
}
