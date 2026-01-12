# Collateral Vault Management System

**Role:** Solana/Rust Developer
**Submission Type:** Technical Assessment
**Network:** Solana Devnet

## 1. System Architecture
The Collateral Vault is a non-custodial smart contract designed to manage user collateral for a Perpetual DEX.

### 1.1 Core Components
* **Vault PDA (`vault` + `vault_id`):** A unique, program-derived account for each user that stores balance state (`available` vs `locked`).
* **Vault Token Account (`token_account`):** An SPL Token Account owned by the Vault PDA. This ensures that **only the program** can withdraw or move funds (via CPI), eliminating custodial risk.

### 1.2 Security Mechanisms
1.  **PDA Ownership:** The Vault Token Account is owned by the Vault PDA, not the user's private key.
2.  **CPI Signers:** Withdrawals (or locking) require the program to sign using `CpiContext::new_with_signer` and the specific PDA seeds.
3.  **Seed Constraints:** Anchor constraints (`seeds = [...]`) are enforced on every instruction to prevent account spoofing.
4.  **Math Safety:** Rust's `checked_add` and `checked_sub` are used for all balance operations to prevent overflow attacks.

## 2. API / Instruction Reference

### `initialize_vault`
* **Input:** `vault_id` (u8) for idempotency.
* **Action:** Derives PDAs, calculates rent, and initializes the `CollateralVault` struct on-chain.

### `deposit`
* **Input:** `vault_id` (u8), `amount` (u64).
* **Action:** Executes a **Cross-Program Invocation (CPI)** to the SPL Token Program (`transfer` instruction). Moves USDT from User ATA -> Vault ATA. Updates `total_balance` and `available_balance`.

### `lock_collateral`
* **Input:** `vault_id` (u8), `amount` (u64).
* **Action:** Internal accounting update. Decreases `available_balance` and increases `locked_balance`. Validates that `available_balance >= amount` before locking.

## 3. Setup & Testing
The project was built and tested using **Solana Playground** (Anchor 0.29+).

**Running Tests:**
```bash
anchor build
anchor test