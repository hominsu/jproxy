# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2025-01-24)

<csr-id-720bc8415b477b85dd5d39279fb0f94695b71a29/>
<csr-id-1ee1f2a1b6a0c144e58610ad23bfa7419bf41d12/>
<csr-id-e6c73b87eb806430c1d75d9fe662196e97237070/>
<csr-id-584e02f5605e2abc1fa6fc6b471aaf2e32bc9184/>
<csr-id-6e42dc58e23e910869d80ebe5fa0b163a2dd7da4/>
<csr-id-53332e00cacaeed45094604cf05094b2cf75ab4e/>
<csr-id-40eecaaf6fa470061073bc8ca796fc5e20b0bfbb/>
<csr-id-852180d4b118aa3571cd1290487439eaf7322880/>
<csr-id-61f14ba664267c8beb88a72de696a74249ab902b/>
<csr-id-bc94018db7e8ed2c0a59c7444a99d9bc5a151a76/>

### Chore

 - <csr-id-720bc8415b477b85dd5d39279fb0f94695b71a29/> add README.md and update Cargo.toml
 - <csr-id-1ee1f2a1b6a0c144e58610ad23bfa7419bf41d12/> update LICENSE
 - <csr-id-e6c73b87eb806430c1d75d9fe662196e97237070/> refactor HTTP client to use hyper-util
   - Refactor HTTP handling in `http/mod.rs` to use `hyper_util::client::legacy::Client`.
   - Change HTTP connection handling to use `Client::builder` with specified executor and HTTP1 settings.

### New Features

<csr-id-0f1d7869f5ac436a29e3f16a5dbdb24eeb901329/>
<csr-id-84b545fe1b89e9a7c3e8dd396ad6ff402616d672/>

 - <csr-id-be27e5c2a5e56e534eefde6096a866de0a6a7dc8/> add config into HttpProxy
 - <csr-id-9acce5fe3b4cab663d88ab59d0b63977cca4066e/> impl Default for Config
 - <csr-id-ba49e5a19aafcdd8be50c024653fb738b0081cd5/> add Connector
 - <csr-id-2aafc2c1e89f5c5885c9519178f947e644c5217d/> implement HTTP proxy server functionality
   - Implement an HTTP proxy with `CONNECT` method support
- Add a serve module that handles incoming connections, and graceful shutdowns

### Other

 - <csr-id-584e02f5605e2abc1fa6fc6b471aaf2e32bc9184/> rename

### Refactor

 - <csr-id-6e42dc58e23e910869d80ebe5fa0b163a2dd7da4/> graceful shutdown
   - Refactor `run` function to use `tokio::select!` for concurrent task management.
   - Implement graceful shutdown using `watch::channel` and `Arc`.
   - Modify `shutdown_signal` function to accept a `Sender` for graceful shutdown.
 - <csr-id-53332e00cacaeed45094604cf05094b2cf75ab4e/> HTTP connector error handling
   - Refactor the `HttpConnector` to use the new error type.
   - Update the `HttpConnector` to handle errors more gracefully.
   - Change the HTTP client to use the new `HttpConnector`.
 - <csr-id-40eecaaf6fa470061073bc8ca796fc5e20b0bfbb/> improve configuration management
   - Moved configuration parameters (debug, bind, concurrent) to a central config file.
   - Refactored configuration loading and management.
   - Separated configuration reading from other functions.
 - <csr-id-852180d4b118aa3571cd1290487439eaf7322880/> improve HTTP proxy and server
   - Refactor proxy to use Tokio runtime.
   - Use graceful shutdown for the serve future.
 - <csr-id-61f14ba664267c8beb88a72de696a74249ab902b/> rename http::error enum items
 - <csr-id-bc94018db7e8ed2c0a59c7444a99d9bc5a151a76/> application for error handling and configuration

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 13 calendar days.
 - 16 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add README.md and update Cargo.toml ([`720bc84`](https://github.com/hominsu/jproxy/commit/720bc8415b477b85dd5d39279fb0f94695b71a29))
    - Update LICENSE ([`1ee1f2a`](https://github.com/hominsu/jproxy/commit/1ee1f2a1b6a0c144e58610ad23bfa7419bf41d12))
    - Graceful shutdown ([`6e42dc5`](https://github.com/hominsu/jproxy/commit/6e42dc58e23e910869d80ebe5fa0b163a2dd7da4))
    - Add config into HttpProxy ([`be27e5c`](https://github.com/hominsu/jproxy/commit/be27e5c2a5e56e534eefde6096a866de0a6a7dc8))
    - Impl Default for Config ([`9acce5f`](https://github.com/hominsu/jproxy/commit/9acce5fe3b4cab663d88ab59d0b63977cca4066e))
    - HTTP connector error handling ([`53332e0`](https://github.com/hominsu/jproxy/commit/53332e00cacaeed45094604cf05094b2cf75ab4e))
    - Add Connector ([`ba49e5a`](https://github.com/hominsu/jproxy/commit/ba49e5a19aafcdd8be50c024653fb738b0081cd5))
    - Refactor HTTP client to use hyper-util ([`e6c73b8`](https://github.com/hominsu/jproxy/commit/e6c73b87eb806430c1d75d9fe662196e97237070))
    - Improve configuration management ([`40eecaa`](https://github.com/hominsu/jproxy/commit/40eecaaf6fa470061073bc8ca796fc5e20b0bfbb))
    - Improve HTTP proxy and server ([`852180d`](https://github.com/hominsu/jproxy/commit/852180d4b118aa3571cd1290487439eaf7322880))
    - Rename http::error enum items ([`61f14ba`](https://github.com/hominsu/jproxy/commit/61f14ba664267c8beb88a72de696a74249ab902b))
    - Implement HTTP proxy server functionality ([`2aafc2c`](https://github.com/hominsu/jproxy/commit/2aafc2c1e89f5c5885c9519178f947e644c5217d))
    - Implement basic HTTP proxy functionality ([`0f1d786`](https://github.com/hominsu/jproxy/commit/0f1d7869f5ac436a29e3f16a5dbdb24eeb901329))
    - Application for error handling and configuration ([`bc94018`](https://github.com/hominsu/jproxy/commit/bc94018db7e8ed2c0a59c7444a99d9bc5a151a76))
    - Rename ([`584e02f`](https://github.com/hominsu/jproxy/commit/584e02f5605e2abc1fa6fc6b471aaf2e32bc9184))
    - Initial commit ([`84b545f`](https://github.com/hominsu/jproxy/commit/84b545fe1b89e9a7c3e8dd396ad6ff402616d672))
</details>

<csr-unknown>
 implement basic HTTP proxy functionality initial commit<csr-unknown/>

