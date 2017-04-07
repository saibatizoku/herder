# herder

A rust client for Mastodon, a GNU Social-compatible microblogging service.

# TODO

* OAuth2 authentication

    * Register client for token-access (a one-time thing)

    * Token authentication for API usage

* Implement Mastodon API

    * Accounts

        - */api/v1/accounts/:id*

        - */api/v1/accounts/verify_credentials*

        - */api/v1/accounts/:id/followers*

        - */api/v1/accounts/:id/following*

        - */api/v1/accounts/:id/statuses*

        - */api/v1/accounts/:id/follow*

        - */api/v1/accounts/:id/unfollow*

        - */api/v1/accounts/:id/block*

        - */api/v1/accounts/:id/unblock*

        - */api/v1/accounts/:id/mute*

        - */api/v1/accounts/:id/unmute*

        - */api/v1/accounts/relationships*

        - */api/v1/accounts/search*

    * ...
