# Pump.fun Solana Smart Contract

This repository contains a Solana-based smart contract designed for token launches. It integrates the latest updates from the **Pump.fun** protocol and incorporates **Meteora migration** functionality. The contract leverages advanced bonding curve mechanics, dynamic fee structures, and robust administrative controls to ensure seamless token distribution and liquidity management.

## Core Features

### Bonding Curve Price Logic

The protocol implements a **constant product linear bonding curve** (`x * y = k`) to enable price discovery and continuous liquidity for the token. Once the bonding curve accumulates **85 SOL**, the remaining tokens are migrated to **Meteora**.

### Dynamic Fee Structure

The contract employs a **piecewise linear fee structure** based on user participation slots. All fees collected are directed to the protocol's **multisig wallet**, ensuring transparency and security.

## Administrative Roles

The protocol defines two key administrative roles:

- **Global Authority**: Responsible for managing global settings and whitelist configurations.
- **Fork Migration**: Fork migration of pump.fun to migrate to pump.fun amm pool. This can handle in one transaction
- **Migration Authority**: Handles the migration of tokens to Meteora.

### Whitelist Management

- The **Global Authority** can configure whitelist allocation settings.
- Whitelist entries are managed via **PDA accounts** (Program Derived Addresses):
  - Add a whitelist entry by creating a PDA for the user.
  - Remove a whitelist entry by deleting the corresponding PDA account.

### Administrative Capabilities

- Modify protocol parameters.
- Manage fee settings.
- Control whitelist status.

## Recent Updates

This project integrates the **latest version of the Pump.fun smart contract and IDL** (Interface Definition Language). The updates include enhanced bonding curve mechanics, improved fee structures, and streamlined migration processes.

## Transaction Examples

Below are some example transactions executed on the **Solana Devnet**:

- **Create Global PDA**: [View Transaction](https://solscan.io/tx/5YmZqVgFcKk11uUVTBZvtMCnbbfthM4QpYHvvWdRNqXhmeyFmE85H5XeQF9pAX6M8DApqn1PeyCH9mYhdCsEkvce?cluster=devnet)
- **Add Whitelist (Create WL PDA)**: [View Transaction](https://solscan.io/tx/3R4fXk3VYXUAAFEXhVoR52g8ZPnjeZcuEhkCPSiBKJQGbjgDW9dBNE7REsz3KwYPV582HzUZ9Qv7SwgnDxgoTXHU?cluster=devnet)
- **Create Bonding Curve**: [View Transaction](https://solscan.io/tx/22cFFDRgLnBpce97FhSE9srHcopkmDG3WpiwbgpwAj6VReu8cLMaZv3vnEvXMBr48XrCLGQ2xAzdUKBxKdfHFx2i?cluster=devnet)
- **Migrate to Meteora**: [View Transaction](https://solscan.io/tx/5F1R9WBYgDXyATWjyyrCJKL2wudjK4WNom6KL4H2LQjcabfLR3agoaifiQWwMEWpmR47bKozJSn1esLCWmyMaRHe?cluster=devnet)

## Contact

For inquiries or support, please reach out via [Telegram](https://t.me/ptcbink).
