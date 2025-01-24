<div id="top"></div>

<!-- PROJECT SHIELDS -->
<p align="center">
<a href="https://github.com/hominsu/jproxy/graphs/contributors"><img src="https://img.shields.io/github/contributors/hominsu/jproxy.svg?style=for-the-badge" alt="Contributors"></a>
<a href="https://github.com/hominsu/jproxy/network/members"><img src="https://img.shields.io/github/forks/hominsu/jproxy.svg?style=for-the-badge" alt="Forks"></a>
<a href="https://github.com/hominsu/jproxy/stargazers"><img src="https://img.shields.io/github/stars/hominsu/jproxy.svg?style=for-the-badge" alt="Stargazers"></a>
<a href="https://github.com/hominsu/jproxy/issues"><img src="https://img.shields.io/github/issues/hominsu/jproxy.svg?style=for-the-badge" alt="Issues"></a>
<a href="https://github.com/hominsu/jproxy/blob/master/LICENSE"><img src="https://img.shields.io/github/license/hominsu/jproxy.svg?style=for-the-badge" alt="License"></a>
<a href="https://github.com/hominsu/jproxy/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/hominsu/jproxy/ci.yml?branch=main&style=for-the-badge" alt="Build"></a>
</p>


<!-- PROJECT LOGO -->
<br/>
<div align="center">
<h3 align="center">jproxy</h3>
  <p align="center">
    Just a proxy
    <br/>
    <br/>
    <a href="#installation">Installation</a>
    ·
    <a href="#usage">Usage</a>
    ·
    <a href="#license">License</a>
  </p>
</div>

## Installation

Install with cargo:

```shell
cargo install jproxy
```

## Usage

```shell
$ jproxy -h
Just a proxy

Usage: jproxy
       jproxy <COMMAND>

Commands:
  run   Run jproxy
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

$ jproxy run -h
Run jproxy

Usage: jproxy run [OPTIONS]

Options:
  -c, --conf <CONF>  Config path, eg: --conf ./configs [default: configs]
  -h, --help         Print help
```

## License

Distributed under the MIT License. See `LICENSE` for more information.