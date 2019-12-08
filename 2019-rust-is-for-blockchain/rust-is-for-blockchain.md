# Rust is for blockchain

Rust is a fairly popular language to build blockchain projects, for obvious
reasons &mdash; blockchain projects need to be reliable, fast, and free of bugs.
They are responsible for securing large volumes of capital, and Rust gives
blockchain developers confidence they can do so. Aimee from Cryptape explains
their rationale in [Why Rust?][wr].

[wr]: https://medium.com/@Aimeedeer/why-rust-c877fba0ca94

In retrospect blockchains are an obvious market for Rust, but I've
been pleased to see these projects spring up and thrive.

The Rust developer market is very competitive right now. I've talked to may
companies using Rust, and they are nearly all desperate to find talented Rust
developers. This problem seems to be even greater in the Rust-blockchain
industry. Maybe it's because of the stigma &mdash; blockchain has passionate
believers and passionate detractors. I've had the fortune of meeting a number of
people from these projects, and like most people most everywhere, they are good
people with good intentions.

If you are both a Rust developer and a blockchain enthusiast, you have
opportunities available.

Some of these companies, like Parity, have been part of the Rust community a
long time and have contributed significant code back to the community, in the
form of crates, and upstream contributions. They are major users of various
crypto crates, and their experience gives us confidence in them. Some of them
have contributed more than code and have a presence

Some of these are _massive_ Rust projects, some of the largest for sure.

TODO: In this post I'll ...

## A domain for Rust

## The virtual machines of blockchain Rust

One of my favorite thing about these projects is that many of them run contracts
on some form of VM, and VM's are fun. Parity has an implementation of EVM (the
Ethereum VM), and WASM. Cryptape's VM runs RISC-V. Solana runs an eBPF VM.

## Why Rust for blockchain?

TODO: Get off your ass and write this stuff.

## Companies using Rust for blockchain

Most of these are companies with blockchain projects, some are in the industry
and using Rust for other purposes, some have open source Rust projects, some
_appear_ to have or are very interested in closed source Rust projects.

- [Parity](https://www.parity.io/)
  - [Parity Ethereum](https://github.com/paritytech/parity-ethereum)
  - [Polkadot](https://github.com/paritytech/polkadot)
  - [Substrate](https://github.com/paritytech/substrate)
    - Framework for building blockchain projects
  - [parity-zcash](https://github.com/paritytech/parity-zcash)
  - [parity-bitcoin](https://github.com/paritytech/parity-bitcoin)
  - [wasmi](https://github.com/paritytech/wasmi)
- [ChainX](http://www.chainx.io/)
  - [Source](https://github.com/chainx-org/ChainX)
- [Cryptape](https://www.cryptape.com/)
  - [Nervos](https://www.nervos.org)
  - [CKB](https://github.com/nervosnetwork/ckb)
  - [CKB-VM](https://github.com/nervosnetwork/ckb-vm)
  - [CITA](https://github.com/cryptape/cita) -  https://www.citahub.com/
- [Grin]
  - [Source](https://github.com/mimblewimble/grin)
- [Solana](https://solana.com/)
  - [Source](https://github.com/solana-labs/solana)
- [NEAR protocol](https://nearprotocol.com/)
  - [nearcore](https://github.com/nearprotocol/nearcore)
- [Cardano](https://www.cardano.org/)
  - [Rust SDK](https://github.com/input-output-hk/rust-cardano)
  - [cardano-cli](https://github.com/input-output-hk/cardano-cli)
- [Holochain](https://holochain.org/)
  - [holochain-rust](https://github.com/holochain/holochain-rust)
- [Coinbase]
  - [coinbase-pro-rs](https://github.com/inv2004/coinbase-pro-rs)
  - 3rd party
- [Zcash](https://z.cash/)
  - [librustzcash](https://github.com/zcash/librustzcash)
- [Mobilecoin](https://www.mobilecoin.com/)
  - [Partial source](https://github.com/mobilecoind/mobilecoin)
  - Source not public yet
  - Using SGX
- [ETCDev](https://www.etcdevteam.com/)
  - Ethereum Classic
  - https://github.com/ETCDEVTeam/sputnikvm
- [Chain](https://chain.com/)
  - https://github.com/chain
  - Ledger-as-a-service
  - Tony Arciery
- MaidSAFE
- [Dock](https://dock.io/)
- [Facebook Libra](https://libra.org/en-US/)
  - todo rust announced?
- [Stellar](https://www.stellar.org/)
  - [Slingshot](https://github.com/stellar/slingshot)
- [Lighthouse](https://github.com/sigp/lighthouse)
  - Ethereum 2.0
  - uses rust-libp2p
  - lighthouse
- [Nimiq](https://github.com/nimiq/core-rs)
- [Celo](https://celo.org) - maybe? recruitment email mentioned rust
- [Qtum](https://qtum.org/en)
  - x86 emulator in Rust
- [The Graph](https://thegraph.com/)
  - Web3 indexing via GraphQL
  - https://github.com/graphprotocol
- [Salt](https://www.saltlending.com/)
  - Cryptocurrency lending platform, sponsored Colorado Gold Rust
  - Rust usage not obvious
  - Mostly closed?
- [POA](https://poa.network/)
  - Ethereum sidechain w/ proof of authority consensus
  - https://github.com/poanetwork
- [Intraverse](https://intraversetech.com/)
  - Corporate blockchains
- [CodeChain](https://codechain.io/)
  - Asset tracking with logal compliance
  - https://github.com/CodeChain-io
- [DFINITY](https://dfinity.org)
  - https://github.com/dfinity
  - Sponsoring RustConf 2019
  - Not actually using Rust directly?
  - Using WASM VM, so supporting Rust.
- [Rlay](https://rlay.com/)
  - Built on Ethereum
  - https://github.com/rlay-project
- [Bitfury](https://bitfury.com/)
  - Exonum framework: https://github.com/exonum/
  - Yew, a Rust UI framework
  - Sponsored RustFest Ukraine 2017, Paris 2018
- [Wildfish](https://wildfish.com/)
  - Cryptocurrency market prediction
  - On friends of Rust page
- [Brave](https://brave.com/)
  - Web browser with BAT token
  - Using Rust for ad-blocking
  - https://brave.com/improved-ad-blocker-performance/
  - https://github.com/brave/adblock-rust
- [ZondaX](http://zondax.ch/)
  - Ledger interop
  - https://github.com/ZondaX/ledger-rs
- [Tendermint](https://tendermint.com/)
  - Using Rust for key management
  - https://github.com/tendermint/kms
  - https://github.com/tendermint/signatory
- [Iqlusion](https://www.iqlusion.io/)
  - https://iqlusion.blog/introducing-abscissa-rust-application-framework
- [Oasis Labs](https://www.oasislabs.com/)
  - https://github.com/oasislabs
- [eian](https://eian.io/)
- [Transparent Systems](https://www.transparentsystems.com/)
- [Fluence Labs](https://fluence.network/)
  - "main backend development language" - https://github.com/fluencelabs/tutorials
- Ripple
  - [Interledger](https://interledger.org/)
  - https://github.com/interledger-rs/interledger-rs
- Protocol Labs (Filecoin)?
  - funding rust-libp2p
- [imToken](https://token.im/)
- [Findora](https://findora.org/)
- [Enigma](https://enigma.co/)
  - https://github.com/enigmampc/enigma-core


## Rust crates used by or created by blockchain projects

### Crypto

- [dalek-cryptography](https://github.com/dalek-cryptography)
  - popular elliptic curve crypto libs
  - Isis Lovecruft and Henry de Valence
  - curve25519, zkps, bulletproofs, constant time functions
  - used by Solana, probably more
- [bn](https://github.com/zcash-hackworks/bn)
  - pairing crypto
  - zcash
- [signatory](https://github.com/tendermint/signatory)
  - digital signatures
  - tendermint
- [yubihsm-rs](https://github.com/tendermint/yubihsm-rs)
  - YubiKey interop
  - tendermint

## VMs

- [rbpf](https://github.com/solana-labs/rbpf)
  - an implementation of the eBPF VM, the Berkely packet filter
- [ckb-vm](https://github.com/nervosnetwork/ckb-vm)
  - a RISC-V VM
  - created by Cryptape
- [wasmi](https://github.com/paritytech/wasmi)
  - a WASM interpreter
  - created by Parity
- [qx86-rs](https://github.com/qtumproject/qx86-rs)
  - x86 emulator
- [sputnikvm](https://github.com/ETCDEVTeam/sputnikvm)
  - EVM

### Highlights

- [rust-libp2p](https://github.com/libp2p/rust-libp2p)
  - official Rust implementation of [libP2p](https://libp2p.io/)
  - it has a bunch of features that make network better on the today's internet
- [p2p](https://github.com/driftluo/p2p)
  - framework for building custom peer-to-peer protocols
  - alternative to libp2p2
- [ws-rs](https://github.com/solana-labs/ws-rs)
  - WebSockets
- [jsonrpc](https://github.com/paritytech/jsonrpc)
  - implements JSON-RPC 2.0
  - created by Parity

## And yet more

- [pwbox-rs](https://github.com/exonum/pwbox-rs)
  - Modular password-based encryption for Rust
- [hbbft](https://github.com/poanetwork/hbbft)
 - "Honey Badger of BFT Protocols"
- [cfb](https://github.com/nervosnetwork/cfb)
  - Canonical FlatBuffers, a variation of FlatBuffers
  - Created by Cryptape
  - Probably a dead-end
- [unsigned-varint](https://github.com/paritytech/unsigned-varint)
- [adblock-rust](https://github.com/brave/adblock-rust)


## Blockchain companies that have sponsored Rust

- Cryptape
- Parity
- Solana
- Zcash
- The Graph
- NEAR
- Salt
- CodeChain
- Bitfury
- Rlay
- [SNZ](https://snzholding.com/)
- DFINITY
- Intraverse
- Oasis Labs
- Transparent systems


## Unsorted source links

- https://www.reddit.com/r/rust/comments/c20aed/facebook_just_picked_rust_to_implement_their_new/erhsz9q/
- https://news.ycombinator.com/item?id=20222349
- https://medium.com/solana-labs/solana-at-portland-dev-meetup-72e4dc7ad32c
- https://rustinblockchain.org/
- https://github.com/rust-in-blockchain/awesome-blockchain-rust

## Blockchain conference sponsors

- Rust Camp 2015 - https://web.archive.org/web/20151230020407/http://rustcamp.com/
  - 6 sponsors
  - 0 blockchain sponsors
- RustConf 2016 - http://2016.rustconf.com/sponsors.html
  - 6 sponsors
  - 0 blockchain sponsors
- RustConf 2017 - http://2017.rustconf.com/sponsors.html
  - 6 sponsors
  - 0 blockchain sponsors
- RustConf 2018 - http://2018.rustconf.com/sponsors.html
  - 22 sponsors
  - 5 blockchain sponsors
- RustConf 2019 - https://rustconf.com/sponsors/
  - 13 sponsors
  - 2 blockchain sponsors
- RustCon Asia 2019 - https://rustcon.asia/
  - 7 sponsors
  - 2 blockchain sponsors
- RustFest Paris 2016 - https://2016.rustfest.eu/sponsoring/
  - 7 sponsors
  - 1 blockchain sponsor
- RustFest Ukraine 2017 - https://2017.rustfest.eu/sponsoring/
  - 6 sponsors
  - 0 blockchain sponsors
- RustFest Zurich 2017 - https://zurich.rustfest.eu/sponsoring/
  - 4 sponsors
  - 1 blockchain sponsor
- RustFest Paris 2018 - https://paris.rustfest.eu/sponsors/
  - 8 sponsors
  - 3 blockchain sponsors
- RustFest Rome 2018 - https://rome.rustfest.eu/sponsors/
  - 11 sponsors
  - 3 blockchain sponsors
- RustFest Barcelona 2019 - https://barcelona.rustfest.eu/sponsors/
  - 1 sponsor
  - 0 blockchain sponsors
- Rust Belt Rust 2016 - http://conf2016.rust-belt-rust.com/
  - 10 sponsors
  - 0 blockchain sponsors
- Rust Belt Rust 2017 - http://conf2017.rust-belt-rust.com/sponsors.html
  - 9 sponsors
  - 0 blockchain sponsor
- Rust Belt Rust 2018 - http://conf2018.rust-belt-rust.com/sponsors/
  - 7 sponsors
  - 1 blockchain sponsor
- Rust Belt Rust 2019 - https://www.rust-belt-rust.com/sponsors/
  - 2 sponsors
  - 0 blockchain sponsors
- Colorado Gold Rust 2019 - https://www.cogoldrust.com/sponsors/
  - 3 sponsors
  - 1 blockchain sponsor
- Rust LatAm 2019 - https://rustlatam.org/#our_sponsors
  - 10 sponsors
  - 4 blockchain sponsors
- Rust Rush - https://rustrush.ru/index
  - 5 sponsors
  - 1 blockchain sponsor

19 conferences
143 sponsors
24 blockchain sponsors
16% of Rust conference sponsorship

----


### TODO

- Rust blockchain discrimination
  - https://github.com/cmr/this-week-in-rust/pull/1029
  - https://www.reddit.com/r/rust/comments/dpakxq/rust_2020_more_or_less/f5tpnkx
  - https://www.reddit.com/r/rust/comments/ds606s/an_apology/
- rust blockchain runtimes / toolkits
  - substrate
  - muta / cita
  - sealevel
- solana sealevel
  - https://github.com/solana-labs/solana/pull/6239
