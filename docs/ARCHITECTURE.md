# Architecture

## Overview

Soroban Escrow is a decentralized escrow platform built on Stellar using Soroban smart contracts with three main components:

1. **Smart Contract** — Core escrow logic deployed on Stellar/Soroban
2. **Utility Library** — Rust library for Soroban developers  
3. **CLI Tool** — Command-line interface for interacting with contracts

## Contract Functions

| Function | Description | Auth Required |
|----------|-------------|---------------|
| `initialize` | Set up contract with admin | Admin |
| `create_escrow` | Lock funds in escrow | Depositor |
| `release` | Release funds to beneficiary | Depositor |
| `refund` | Refund funds to depositor | Admin |
| `dispute` | Raise a dispute | Beneficiary |
| `get_escrow` | Read escrow details | None |
| `get_count` | Get total escrow count | None |

## Security

- All functions require authorization via `require_auth()`
- Token transfers use the official Soroban token interface
- Admin can only trigger refunds — cannot access funds arbitrarily
- All state is stored on-chain — fully transparent and auditable