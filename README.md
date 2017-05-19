herder
======

A rust client for Mastodon, a GNU Social-compatible microblogging service.

**This crate is under development, and will keep on changing often. Be warned.**

**As the crate is under development, the examples will be changing often.**

EXAMPLES
--------

Currently, you can checkout the `examples` folder for use-cases involving the code.

## [examples/create-app.rs](examples/create-app.rs)

To run this example:


Get help with how the example works, anytime by running:
```
cargo run --example create-app -- -h
```

To register a new app, simply specify the HTTPS path to your Mastodon node.
```
cargo run --example create-app -- https://mastodon.example.com/ my-app.json
```

where `my-app.json` save the returned client credentials in JSON format.
```
```


TODO
----

* Implement Mastodon API (v1) as found at the official [API Overview](https://github.com/tootsuite/documentation/blob/master/Using-the-API/API.md)

  - [X] OAuth Application registration / OAuth Application authorization. See [example](examples/create-app.rs)

  - [X] API Entities
	  - [X] Tests for JSON deserialization
	  - [X] JSON deserialization
  - API Methods
	  - [X] Tests all endpoints
	  - [ ] Build endpoint requests
	  - [ ] Parse endpoint responses
  - Web Client, hyper-based
