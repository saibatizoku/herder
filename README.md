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

or, you can read the new client configuration by running:
```
cargo run --example create-app -- https://mastodon.example.com/ -c myclient.json my-app.json
```

where `myclient.json` is a customized 'CreateApp' in JSON format.

The printed output instructs you on how to continue, by following a URL of the type:

    https://mastodon.example.com/oauth/authorize?client_id=CLIENT_ID&redirect_uri=urn:ietf:wg:oauth:2.0:oob&response_type=code

This will take you to a page where you may authorize the app on your Mastodon server. Once you click on 'authorize', you will be redirected to a page where you will find your **authorization code**. You'll use this code in the next example, in order to finish authorizing our app, and to retrieve our access token.


TODO
----

* Implement Mastodon API (v1) as found at the official [API Overview](https://github.com/tootsuite/documentation/blob/master/Using-the-API/API.md)

  - [X] Client

      - [X] API Entities
          - [X] Tests for JSON deserialization
          - [X] JSON deserialization

      - [ ] API Methods
          - [X] Tests all endpoints
          - [X] Build endpoint requests
          - [ ] Parse endpoint responses

      - [ ] OAuth
          - [X] Registering client with OAuth. See [example](examples/create-app.rs).
          - [ ] Requesting authorization with OAuth
          - [ ] Requesting the access token

  - [X] Mastodon

  - [ ] Herd. A collection of Mastodons

  - [ ] Herder. A manager for multiple Mastodons.
