# Version Information

## v1.0.1 — 2026-01-02

This release improves reproducibility and documentation for on-chain workflows.

Included updates:
- Documented devnet/testnet deployment using a known working v1 endpoint
- Documented private record scanning via `snarkos developer scan`
- Added PowerShell example scripts (`examples/deploy_testnet.ps1`, `examples/scan_testnet.ps1`)
- Documentation updates for consistency (Leo `Message` format, architecture overview)
- Minor README polish and consistency fixes

---

## v1.0.0 — 2025-11-21

This is the first public release of the `aleo-simple-messaging` project.

Included components:
- Leo program (`simple_messaging.aleo`)
- Rust CLI wrapper (`simple_messaging_cli`)
- Documentation set (`README.md`, `docs/architecture.md`, `docs/message_format.md`, `docs/contributing.md`, `FAQ.md`)
- Initial example script (`examples/run_send_message.sh`)

This version represents a minimal, educational baseline for demonstrating private record handling and Leo → Rust integration.
