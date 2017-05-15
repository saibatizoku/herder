extern crate herder;
extern crate hyper;

use herder::api::APIMethod;
use hyper::Method::{Get};
use hyper::{Headers, Uri};
use std::str::FromStr;

#[test]
fn default_api_method() {
    let default = APIMethod {
        body: None,
        headers: Headers::new(),
        method: Get,
        query: None,
        uri: Uri::from_str("/").unwrap()
    };
    assert_eq!(default, APIMethod::default())
}
