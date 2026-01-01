# FAQ – simple-messaging (Aleo demo)

This document provides concise answers to the most common questions regarding the `simple-messaging` demonstration project.

---

## What is this project?

This repository is a minimal, educational example designed to illustrate:

- how a simple Leo program is structured,
- how private `record` types function in Aleo,
- how to invoke Leo transitions from a Rust command-line application.

The primary objective is **clarity and simplicity**.

---

## What is this demo useful for?

- Understanding Leo ↔ Rust integration  
- Educational purposes (workshops, tutorials, self-study)  
- Demonstrating how private records are constructed and stored  
- Serving as a starting point for developers building more advanced Leo applications  

---

## Why does it only handle three `field` inputs?

To keep the example minimal and accessible.

The goal is *not* to represent a full messaging system, but to provide:

- an easy-to-understand structure,
- a clear demonstration of transition mechanics,
- a clean foundation for more complex extensions.

The schema can easily be expanded (additional fields, timestamps, nonces, etc.).

---

## Does it work on testnet or devnet?

Yes.

The project includes documented examples for deploying the Leo program and scanning private records
on Aleo devnet/testnet using a known working v1 endpoint.

The focus of the demo remains educational and minimal, but on-chain deployment and record scanning
are supported and documented in `README.md` and `examples/`.

---

## Why is the Leo CLI executed under WSL?

The Leo toolchain is currently optimized for Linux.  
For stable execution on Windows, running Leo inside **WSL (Ubuntu)** is recommended.

The Rust CLI itself can run on:

- Windows (PowerShell or Windows Terminal),
- WSL/Ubuntu,
- native Linux,
- macOS.

---

## What is the long-term purpose of this project?

The aim is to provide a **clean, minimal, and instructional reference** for:

- private data handling in Leo,
- transition invocation,
- Rust → Leo execution workflows.

It is a compact, easily understandable example that can act as a solid foundation for future development or as a reference for other Leo projects.

---

## What do I need to install to run it?

- Git  
- Rust toolchain (`cargo`)  
- Leo CLI  
- WSL2 + Ubuntu (recommended, especially for devnet/testnet setups)

See `README.md` for complete installation instructions.

---

## Can I propose improvements or add extensions?

Absolutely.  
Contribution guidelines can be found in `docs/contributing.md`.

---

## Under what license is the project distributed?

The project is released under the **MIT License**, which permits free use, modification, and redistribution.
