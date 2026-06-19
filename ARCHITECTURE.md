# Architecture

## Overview

Soroban Escrow is a single smart contract deployed on the Stellar blockchain. All state lives on-chain. There is no backend server or database.

## Escrow Lifecycle
Depositor                Contract               Beneficiary
|                       |                       |
|-- create_escrow() --> |                       |
|                       | (funds locked)        |
|                       |                       |
|                       | <-- dispute() --------|
|                       | (disputed state)      |
|                       |                       |
|-- release() -------> |                       |
|                       |-- transfer funds --> |
|                       |                       |

## Contract State

Each escrow is stored as a map entry keyed by `escrow_id` (a `u64` counter). Each entry contains:

- `depositor` — Stellar address that locked the funds
- `beneficiary` — Stellar address to receive funds on release
- `token` — Stellar/Soroban token contract address
- `amount` — Token amount in base units
- `release_time` — Unix timestamp after which release is allowed
- `status` — `Active | Released | Disputed | Refunded`

## Files

| Path | Purpose |
|---|---|
| `contracts/escrow/src/lib.rs` | Main contract implementation |
| `src/` | Rust utility library (address validation, hashing, XLM formatting) |
| `.github/workflows/ci.yml` | CI pipeline — runs `cargo test` and `cargo clippy`
