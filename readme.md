# EMtoken (Stylus ERC-20 Challenge)

### Overview
Enhanced ERC-20 token using Stylus with ownership, pausing, and gamified level rewards.

### Features
- Capped supply
- Mint/burn with owner control
- Pausable contract
- Level system: every 3 levels → reward mint
- Deployable on Arbitrum Sepolia

### Deployment
```bash
cargo stylus build
cargo stylus deploy \
  --rpc-url https://sepolia-rollup.arbitrum.io/rpc \
  --private-key <YOUR_PRIVATE_KEY>

