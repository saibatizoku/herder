//! Create the Error, ErrorKind, ResultExt, and Result types.
use hyper;
use serde_json;
use std::io;
use url;

error_chain! {
    foreign_links {
        Io(io::Error);
        Json(serde_json::Error);
        Url(url::ParseError);
        Hyper(hyper::error::Error);
    }
}
