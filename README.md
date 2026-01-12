# 🛡️ Non-Custodial Collateral Vault (Solana/Anchor)

A secure, non-custodial DeFi primitive built on **Solana** using the **Anchor Framework**. This protocol allows users to deposit assets into isolated vaults managed by PDAs, ensuring fund safety and strictly enforced logic.

## 🚀 Key Features

* **Program Derived Addresses (PDAs):** Uses deterministic PDAs to create isolated vaults for each user, preventing address collisions.
* **Cross-Program Invocations (CPI):** Integrates with the **SPL Token Program** to handle secure transfers of collateral (deposits/withdrawals) directly on-chain.
* **Math Overflow Protection:** Implements SafeMath checks to prevent integer overflow/underflow attacks during balance updates.
* **Access Control:** Strict checks (`Signer` verification) to ensure only the vault owner can initiate withdrawals.

## 🛠️ Tech Stack

* **Language:** Rust
* **Framework:** Anchor
* **Network:** Solana Devnet
* **Testing:** TypeScript (Mocha/Chai)

## 🧪 Testing & Verification

The protocol includes a comprehensive test suite validating the entire lifecycle:
1.  ✅ **Vault Initialization:** Verifies PDA creation and account allocation.
2.  ✅ **Deposit Logic:** Checks token transfer from User Wallet -> Vault PDA.
3.  ✅ **Withdrawal Logic:** Checks token transfer from Vault PDA -> User Wallet.
4.  ✅ **Security:** Verifies that unauthorized users *cannot* withdraw funds.

## 👨‍💻 Author

**Kaustubh Dhage**
*Blockchain Engineer | Rust & Flutter Developer*
