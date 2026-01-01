## 📂 Project Structure (tree)

```
aleo-simple-messaging/
├─ README.md
├─ LICENSE
├─ VERSION.md
├─ CHANGELOG.md
│
├─ docs/
│   ├─ architecture.md
│   ├─ message_format.md
│   └─ contributing.md
│
├─ examples/
│   └─ run_send_message.sh
│
├─ leo/
│   └─ simple_messaging/
│       ├─ src/
│       │   └─ main.leo
│       ├─ build/
│       └─ manifest.json
│
└─ rust/
    └─ simple_messaging_cli/
        ├─ src/
        │   └─ main.rs
        └─ Cargo.toml
```
> **Note on WSL usage**
>
> For reliable devnet/testnet deployment and execution, it is recommended to place the `leo/simple_messaging`
> directory on a **WSL-native filesystem** (e.g. `/home/<user>/...`) and run the Leo CLI from within WSL.
>
> In some Windows filesystem setups (e.g. `/mnt/c/...`), network transactions may appear to succeed
> but fail to be confirmed on-chain.

This recommendation reflects practical experience and aligns with the Leo toolchain’s Linux-first design.

```