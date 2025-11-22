# Architecture Overview — Aleo Simple Messaging

This document provides a structured, high-level overview of the `aleo-simple-messaging` project, describing the roles of the Leo and Rust components and outlining the execution pipeline.

Although the project is intentionally minimal and educational, it illustrates the core architectural patterns of a basic Aleo-based application.

---

## 1. High-Level Overview

The system consists of two primary components:

1. **Leo Program**  
   Implements a simple `send_message` transition that creates a single private `Message` record.

2. **Rust CLI**  
   A lightweight command-line wrapper responsible for:
   - collecting user parameters,
   - constructing a `leo execute` command,
   - optionally executing it (with the `--run` flag).

Together, these components demonstrate how an off-chain client can invoke a Leo transition and produce a private on-chain record.

---

## 2. Directory Structure

```
aleo-simple-messaging/
│
├─ leo/
│   └─ simple_messaging/
│       ├─ src/main.leo       # Leo program (smart contract)
│       ├─ build/             # Build artifacts
│       └─ manifest.json      # Leo configuration
│
└─ rust/
    └─ simple_messaging_cli/
        ├─ src/main.rs        # Rust CLI application
        ├─ Cargo.toml         # Rust build configuration
        └─ README.md (optional)
```

---

## 3. Execution Pipeline

The execution flow is as follows:

1. **The user provides input parameters:**
   - sender  
   - recipient  
   - msg_id  
   - data0, data1, data2  
   - private key  

2. **The Rust CLI constructs the corresponding command:**

   ```bash
   leo execute simple_messaging.aleo send_message <args...>
   ```

3. **The Leo CLI executes the transition**
   - a new private `Message` record is created,
   - ownership is assigned to the `recipient`.

4. **The output is displayed**
   - for offline execution, the result is printed locally,
   - in a networked environment (devnet/testnet), the result could be broadcast as a transaction.

---

## 4. Private Record Semantics

In Leo, all `record` types:

- are **private by default**,  
- are committed to the network in encrypted form,  
- can only be accessed by their designated owner.

This example demonstrates how to:

- create a private record within a transition,
- assign ownership to an arbitrary address.

---

## 5. Benefits of This Structure

- Clean separation between contract logic and client execution  
- Minimal and easy to understand  
- Simple foundation for educational use, workshops, or experimentation  
- Demonstrates the core Leo → Rust integration pattern

---

## 6. Extension Possibilities

Although intentionally minimal, the project can easily be extended with:

- richer message schemas  
- timestamps or nonces  
- additional record fields  
- record-reading or listing examples  
- automated devnet/testnet deployment scripts  

---

## 7. Summary

This project provides a clean, comprehensible baseline for understanding the fundamental patterns of Leo contract execution and Rust CLI integration.

Real-world Aleo applications are typically more complex, but the structure presented here captures the essential concepts clearly and concisely.
