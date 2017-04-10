herder
======

A rust client for Mastodon, a GNU Social-compatible microblogging service.

** This crate is under development, and will keep on changing often. Be warned.**

Usage
-----

[WIP]
### Create a client for a Mastodon Node

    >> extern crate herder;
    >> use herder::Mastodon;
    >> let mastodon_node = Mastodon::new("https://localhost");

---

[WIP]
### Create a new OAuth2 App on a Mastodon Node
    >> let mut oauthapp = mastodon_node.oauth_app("my_app"):
    >> oauthapp.redirect_uris = "urn:ietf:wg:oauth:2.0:oob";
    >> oauthapp.scopes = "read write follow";

---

[WIP]
### Register a new OAuth2 App on a Mastodon Node
    >> let mut app_secrets = oauthapp.add().unwrap();
    >> app_secrets.username("yourusername");
    >> app_secrets.password("yourpassword");
    >> let api_token = oauthapp.register(app_secrets).unwrap();

  You would now need to login to your Mastodon node, and authorize the
  new app we have just registered. Once authorized, the token

---
[WIP]
Or, if the client app is already registered, it may be loaded from file.
    >> let mut herder = mastodon_node.load_app("my_app.json"):

TODO
----

* Implement Mastodon API (v1) as found at the official [API Overview](https://github.com/tootsuite/mastodon/blob/master/docs/Using-the-API/API.md)
