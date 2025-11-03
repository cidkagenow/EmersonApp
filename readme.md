# EMtoken (Stylus ERC-20 Challenge)

## Overview
EMtoken is an enhanced ERC-20 token built with the Stylus SDK for Arbitrum Orbit chains.  
It introduces gamified mechanics, ownership control, and full ERC-20 compliance.

## Features
- Minting (capped)
- Burnable
- Pausable
- Ownership-based access control
- Level-up reward system
- Event-ready architecture

## Commit History
| Commit | Description |
|--------|--------------|
| **1️⃣** | Initialized base ERC-20 contract |
| **2️⃣** | Added ownership & admin controls |
| **3️⃣** | Added level-up reward system |
| **4️⃣** | Added view helpers and events |
| **5️⃣** | Final documentation and cleanup |

## Build & Deploy
```bash
cargo stylus build
cargo stylus deploy --rpc-url https://sepolia-rollup.arbitrum.io/rpc --private-key <PRIVATE_KEY>
