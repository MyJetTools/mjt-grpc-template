# Mjt grpc service template
## Overview

Simple rust grpc service template povered by [cargo-generate][generate].

Features list:
* Out of box [ServiceBus][sb] integration.
* Out of box [NoSql][ns] integration.
* Out of box [PostgreSQL][psql] integration.
* Out of box [SeqLogger][seq] integration.
* GitHub CI template.
* Dockerfile with base features.

[generate]: https://github.com/cargo-generate/cargo-generate
[sb]: https://github.com/MyJetTools/my-service-bus
[ns]: https://github.com/MyJetTools/my-no-sql-server
[psql]: https://github.com/MyJetTools/my-postgres
[seq]: https://github.com/MyJetTools/my-seq-logger


Here's a **`generator`** demo:
![demo](internal/demo.gif)

## Install and start

---
### Install cargo generate
```sh
cargo install cargo-generate
```

### Add this into ur bash config file
```sh
alias mjt-grpc="cargo generate --git https://github.com/MyJetTools/mjt-grpc-template.git"
```