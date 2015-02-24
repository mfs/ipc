## ipc

[![Build Status](https://travis-ci.org/mfs/ipc.svg?branch=master)](https://travis-ci.org/mfs/ipc)

A simple clone of ipcalc in Rust. Written to get a feel for the language. Needs
cleaning up here and there and I have a couple of ideas for extra features.

It builds with the latest nightly release. Let me know if it doesn't build with
the latest nightly or if you find any bugs.

To build use cargo:

```
cargo build
```

Pass `ipc` a CIDR:

```
$ ipc 10.0.0.100/24
Address:   10.0.0.100       00001010.00000000.00000000.01100100
Netmask:   255.255.255.0    11111111.11111111.11111111.00000000
---------------------------------------------------------------
Network:   10.0.0.0         00001010.00000000.00000000.00000000
HostMin:   10.0.0.1         00001010.00000000.00000000.00000001
HostMax:   10.0.0.254       00001010.00000000.00000000.11111110
Broadcast: 10.0.0.255       00001010.00000000.00000000.11111111
NumHosts:  254              rDNS: 1.1.1.10.in-addr.arpa
```

## planned features

- [ ] IPv6 support
