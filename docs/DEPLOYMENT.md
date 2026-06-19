# Deployment Guide

Guide for deploying the Soroban Escrow contract to Stellar testnet and mainnet.

---

## Prerequisites

- [Rust](https://rustup.rs/) 1.75+
- [Soroban CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup)
- A funded Stellar account

---

## 1. Build the Contract

```bash
soroban contract build --manifest-path contracts/escrow/Cargo.toml
```

---

## 2. Set Up Your Identity

```bash
soroban keys generate deployer --network testnet
```

Fund your testnet account:
```bash
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"
```

---

## 3. Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_escrow.wasm \
  --network testnet \
  --source deployer
```

---

## 4. Initialize the Contract

```bash
soroban contract invoke \
  --id YOUR_CONTRACT_ID \
  --network testnet \
  --source deployer \
  -- initialize \
  --admin $(soroban keys address deployer)
```

---

## 5. Deploy to Mainnet

Replace `--network testnet` with `--network mainnet` in all commands above.

---

## Network Configuration

| Network | RPC URL |
|---------|---------|
| Testnet | `https://soroban-testnet.stellar.org` |
| Mainnet | `https://soroban-rpc.stellar.org` |