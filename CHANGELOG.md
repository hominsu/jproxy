# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.0.1 (2025-02-10)

### New Features

 - <csr-id-9d44e47f3649e1cf5d807556619347881d140c01/> improve logging for local address assignments
   - Use `tracing::trace!` instead of `println!` for assigning the local address.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Improve logging for local address assignments ([`9d44e47`](https://github.com/hominsu/jproxy/commit/9d44e47f3649e1cf5d807556619347881d140c01))
</details>

## v1.0.0 (2025-02-04)

<csr-id-2d4f527fa53108052c18a71cddcfdf54209c68f8/>

### New Features

 - <csr-id-cd692c1a1171777cc8df0a99e65488f50e280d08/> implement route adding functionality for linux target
   - Add a new `route` module and conditionally include it on linux.
- Add a function to add a route based on a CIDR on linux.

### Other

 - <csr-id-2d4f527fa53108052c18a71cddcfdf54209c68f8/> add Docker support for building and deploying
   - Add a GitHub action workflow for building and pushing Docker images on release
   - Add a Dockerfile for building the application, using multi-stage builds for optimization
   - Add a docker-bake.hcl file for defining Docker build targets and configurations
   - Add a docker-compose.yml file for local development and deployment using Docker Compose

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release jproxy v1.0.0 ([`913cffc`](https://github.com/hominsu/jproxy/commit/913cffc7b61050fd01ca6343ab8ee5f0d8e780ca))
    - Implement route adding functionality for linux target ([`cd692c1`](https://github.com/hominsu/jproxy/commit/cd692c1a1171777cc8df0a99e65488f50e280d08))
    - Add Docker support for building and deploying ([`2d4f527`](https://github.com/hominsu/jproxy/commit/2d4f527fa53108052c18a71cddcfdf54209c68f8))
</details>

## v0.1.5 (2025-02-03)

### New Features

 - <csr-id-328e00d45dd7b22e14196bbef8d89d8a1a12c27c/> implement configurable memory allocators
   - Add feature flags for memory allocators, and set a global allocator based on feature flags.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release jproxy v0.1.5 ([`5a9abb3`](https://github.com/hominsu/jproxy/commit/5a9abb3c7cf48171f4e879150cd6cf425469b9f3))
    - Implement configurable memory allocators ([`328e00d`](https://github.com/hominsu/jproxy/commit/328e00d45dd7b22e14196bbef8d89d8a1a12c27c))
</details>

<csr-unknown>
Add support for mimalloc, rpmalloc, snmalloc, and tikv-jemallocator.<csr-unknown/>

## v0.1.4 (2025-02-02)

<csr-id-b6df33f305b82b661ad7288d344f89d8998ed390/>

### Bug Fixes

 - <csr-id-f3c77222cd53496c1a410a7870c0e07f49b4aebf/> CIDR IP address generation
   - Implement a more efficient way of picking a random IP within a CIDR.

### Refactor

 - <csr-id-b6df33f305b82b661ad7288d344f89d8998ed390/> server socket configuration
   - Change the `bind` config from a String to a SocketAddr
   - Change the `concurrent` config from usize to u32
   - Parse the bind address directly in the config default

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release jproxy v0.1.4 ([`a01bdba`](https://github.com/hominsu/jproxy/commit/a01bdba0eff3509d52681ca08d24382d82d1e390))
    - CIDR IP address generation ([`f3c7722`](https://github.com/hominsu/jproxy/commit/f3c77222cd53496c1a410a7870c0e07f49b4aebf))
    - Server socket configuration ([`b6df33f`](https://github.com/hominsu/jproxy/commit/b6df33f305b82b661ad7288d344f89d8998ed390))
</details>

## v0.1.3 (2025-01-31)

<csr-id-8c1242c822def63bf633e0876bd516f5f9158811/>

### Bug Fixes

 - <csr-id-b520d30cfbd06781a8e0b76f1d6c364d800504ab/> lint

### Refactor

 - <csr-id-8c1242c822def63bf633e0876bd516f5f9158811/> network bindings to use local CIDR range
   - Remove unused `cidr_range` field from `Config` struct
   - Filter out empty `SocketAddrs`
   - Add a function to assign a local address from a CIDR range to `TcpConnector`
   - Use the new function in `HttpProxy` to assign a local address from the configured CIDR range

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release jproxy v0.1.3 ([`e560891`](https://github.com/hominsu/jproxy/commit/e5608914f0c48454de874174167dd7de60cfcf3c))
    - Lint ([`b520d30`](https://github.com/hominsu/jproxy/commit/b520d30cfbd06781a8e0b76f1d6c364d800504ab))
    - Network bindings to use local CIDR range ([`8c1242c`](https://github.com/hominsu/jproxy/commit/8c1242c822def63bf633e0876bd516f5f9158811))
</details>

## v0.1.2 (2025-01-31)

<csr-id-159e82b42e65ce6812b05251c4bf878dbeecf973/>

### Refactor

 - <csr-id-159e82b42e65ce6812b05251c4bf878dbeecf973/> HTTP proxy
   - Remove square brackets from the host address in `tcp.rs`.
   - Refactor `HttpProxy` to use separate functions for `CONNECT` and other requests.
   - Change default log level from `info` to `trace` when in debug mode.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release jproxy v0.1.2 ([`25accff`](https://github.com/hominsu/jproxy/commit/25accff916a2bacb76859f8927afc67d7e70a48c))
    - HTTP proxy ([`159e82b`](https://github.com/hominsu/jproxy/commit/159e82b42e65ce6812b05251c4bf878dbeecf973))
</details>

## v0.1.1 (2025-01-30)

<csr-id-f1ddaecbfdecf9730e250f0381e2fc43479bb081/>
<csr-id-bee1387c7371dfca12600a151f0ef73b48fa670f/>

### New Features

 - <csr-id-ad0fc28f61daa6f49ec638da396c4323de73ee4a/> add tcp connector
 - <csr-id-1053cb389e27e08d97f7aa7b59da9bddd7a769f0/> add dns resolver

### Refactor

 - <csr-id-f1ddaecbfdecf9730e250f0381e2fc43479bb081/> set connection timeout
 - <csr-id-bee1387c7371dfca12600a151f0ef73b48fa670f/> restructure the code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 6 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release jproxy v0.1.1 ([`f657dd6`](https://github.com/hominsu/jproxy/commit/f657dd6e3b6a33a72679f895da73679ffd930f35))
    - Set connection timeout ([`f1ddaec`](https://github.com/hominsu/jproxy/commit/f1ddaecbfdecf9730e250f0381e2fc43479bb081))
    - Restructure the code ([`bee1387`](https://github.com/hominsu/jproxy/commit/bee1387c7371dfca12600a151f0ef73b48fa670f))
    - Add tcp connector ([`ad0fc28`](https://github.com/hominsu/jproxy/commit/ad0fc28f61daa6f49ec638da396c4323de73ee4a))
    - Add dns resolver ([`1053cb3`](https://github.com/hominsu/jproxy/commit/1053cb389e27e08d97f7aa7b59da9bddd7a769f0))
</details>

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

 - 17 commits contributed to the release over the course of 14 calendar days.
 - 16 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release jproxy v0.1.0 ([`4ddef84`](https://github.com/hominsu/jproxy/commit/4ddef84a0f21e2a31e16344fa76fe335da553dc6))
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

