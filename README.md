# simple-messaging (Aleo demo)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Status](https://img.shields.io/badge/status-demo-green.svg)
![Rust](https://img.shields.io/badge/Rust-stable-orange)
![Leo](https://img.shields.io/badge/Leo-Aleo-blue)

This repository provides a minimal, self-contained educational example demonstrating:

- how to define and use a private `record` in Leo,
- how to implement a simple transition (`send_message`),
- how to invoke Leo transitions from a Rust command-line wrapper,
- how Leo and Rust components interact in a clean, modular structure.

The project is intentionally minimalistic, with its primary focus on **clarity and instructional value** rather than full application functionality.

---

## 📂 Project Structure

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
│       ├─ src/main.leo
│       ├─ build/
│       └─ manifest.json
│
└─ rust/
    └─ simple_messaging_cli/
        ├─ src/main.rs
        └─ Cargo.toml
```

---

## ⚙️ Development Environment

Recommended setup:

- **Windows 10/11**
- **PowerShell or Windows Terminal**
- **Rust toolchain (`cargo`)**
- **Leo CLI (executed under Linux/WSL)**
- **WSL2 + Ubuntu — optional, but recommended especially for devnet/testnet node use**

> The Leo toolchain is optimized for Linux environments.  
> On Windows, running Leo inside WSL (Ubuntu) provides the most reliable experience.

---

## 🔷 1. Repository Setup (PowerShell)

```powershell
cd C:\
git clone https://github.com/DavidsProof33/aleo-simple-messaging.git
cd .\aleo-simple-messaging\
```

---

## 🔷 2. Leo Program

```rust
program simple_messaging.aleo {

    record Message {
        owner: address,
        msg_id: field,
        data0: field,
        data1: field,
        data2: field,
    }

    transition send_message(
        sender: address,
        recipient: address,
        msg_id: field,
        data0: field,
        data1: field,
        data2: field,
    ) -> Message {
        return Message {
            owner: recipient,
            msg_id: msg_id,
            data0: data0,
            data1: data1,
            data2: data2,
        };
    }
}
```

### Building the Leo program (WSL)

```bash
cd /mnt/c/aleo-simple-messaging/leo/simple_messaging
leo build
```

### Running Leo build from PowerShell (through WSL)

```powershell
cd C:\aleo-simple-messaging
wsl bash -lc "cd /mnt/c/aleo-simple-messaging/leo/simple_messaging && leo build"
```

---

## 🔷 3. Rust CLI Wrapper

### Build

```powershell
cd C:\aleo-simple-messaging\rust\simple_messaging_cli
cargo build
```

### Dry-run Execution

```powershell
cargo run -- `
  --sender aleo1SENDER... `
  --recipient aleo1RECIPIENT... `
  --msg-id 1field `
  --data0 10field `
  --data1 20field `
  --data2 30field `
  --private-key "APrivateKey1..." `
  --network testnet
```

### Actual Execution (`--run`)

```powershell
cargo run -- `
  --sender aleo1SENDER... `
  --recipient aleo1RECIPIENT... `
  --msg-id 1field `
  --data0 10field `
  --data1 20field `
  --data2 30field `
  --private-key "APrivateKey1..." `
  --network testnet `
  --run
```

---

## 🔷 Example Output

```
Building program... OK
Compiling... OK
Executing `send_message`...

➡️  New private record created:
- owner: aleo1recipient...
- msg_id: 1field
- data0: 10field
- data1: 20field
- data2: 30field

Execution finished successfully ✔
```

---

## 🔷 Requirements

- Windows 10/11  
- PowerShell or Windows Terminal  
- Git  
- Rust toolchain (`cargo`)  
- Leo CLI (via Linux/WSL)  
- **WSL2 + Ubuntu — recommended especially for devnet/testnet usage**

---

## 🔷 Installation Summary

**Git (Windows)**  
```powershell
git --version
```

**WSL2 (optional but recommended)**  
```powershell
wsl --install -d Ubuntu
```

**Rust**  
https://rustup.rs

**Leo CLI**  
https://developer.aleo.org

---

## 🔷 Purpose of This Project

This repository aims to provide a **clear, minimal, instructional reference implementation** for:

- handling private data in Leo,
- invoking transitions,
- integrating Leo execution via Rust.

It is a compact example designed for education, experimentation, and as a foundation for further development.

---

## 🔷 License

This project is distributed under the MIT License.  
See the `LICENSE` file for details.

---

## 🔷 Motivation

Our goal is to offer a clean, structured, easy-to-understand example that demonstrates:
- private record creation,
- transition mechanics,
- Rust → Leo interaction patterns.

---

## 🔷 Roadmap

- devnet/testnet deployment example  
- record querying examples  
- enhanced output formatting  
- expanded documentation  

---

## 🔷 Related Documentation

- [Architecture](docs/architecture.md)
- [Message Format](docs/message_format.md)
- [FAQ](FAQ.md)

---

## 🔷 Useful Links

- Leo documentation: https://developer.aleo.org  
- Aleo website: https://www.aleo.org  

---

## 🔷 Known Issues

- The project is currently tested offline only.  
- devnet/testnet deployment is not included in the demo.  
- Rust CLI uses basic input validation.  

---
