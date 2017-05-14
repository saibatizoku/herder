extern crate herder;
extern crate hyper;

use herder::api::APIMethod;
use hyper::Method::{Get};

#[test]
fn default_api_method() {
    let default = APIMethod {
        request_method: Get,
        endpoint: String::new(),
        form_data: None,
        url_query: None
    };
    assert_eq!(default, APIMethod::default())
}
