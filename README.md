herder
======

A rust client for Mastodon, a GNU Social-compatible microblogging service.

**This crate is under development, and will keep on changing often. Be warned.**

**As the crate is under development, the examples will be changing often.**

EXAMPLES
--------

Currently, you can checkout the `examples` folder for use-cases involving the code.

## [examples/create-oauth-app.rs](examples/create-oauth-app.rs)

To run this example:


Get help with how the example works, anytime by running:
```
cargo run --example create-oauth-app -- -h
```

To register a new app, simply specify the HTTPS path to your Mastodon node.
```
cargo run --example create-oauth-app -- https://mastodon.example.com
```

To save the response in JSON format:
```
cargo run --example create-oauth-app -- https://mastodon.otherexample.com/ -j out.json
```
or
```
cargo run --example create-oauth-app -- https://mastodon.otherexample.com/ --json-file out.json
```


TODO
----

* Implement Mastodon API (v1) as found at the official [API Overview](https://github.com/tootsuite/documentation/blob/master/Using-the-API/API.md)

  - [X] OAuth Application registration / OAuth Application authorization. See [example](examples/create-oauth-app.rs)

  - [X] API Entities
	  - [X] Tests for JSON deserialization
	  - [X] JSON deserialization
  - API Methods
	  - [X] Tests all endpoints
	  - [ ] Build endpoint requests
	  - [ ] Parse endpoint responses
  - Web Client, hyper-based
