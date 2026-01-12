import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CollateralVault } from "../target/types/collateral_vault";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import type { CollateralVault } from "../target/types/collateral_vault";

const assert = {
  equal: (actual, expected) => { if(actual.toString() != expected.toString()) throw new Error(`Expected ${expected} got ${actual}`); },
  ok: (val) => { if(!val) throw new Error("Assertion failed"); }
};

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.CollateralVault as Program<CollateralVault>;
const wallet = provider.wallet as anchor.Wallet;

// Random ID to ensure fresh vault
const VAULT_ID = Math.floor(Math.random() * 255);
console.log(`🚀 TESTING WITH VAULT ID: ${VAULT_ID}`);

let usdcMint: PublicKey, userAta: PublicKey, vaultPda: PublicKey, vaultTokenPda: PublicKey;
const decimals = 6;
const amount100 = new anchor.BN(100000000);
const amount20 = new anchor.BN(20000000);

describe("Collateral Vault Final", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CollateralVault as anchor.Program<CollateralVault>;
  
  
  it("Setup", async () => {
    usdcMint = await createMint(provider.connection, wallet.keypair, wallet.publicKey, null, decimals);
    userAta = await getAssociatedTokenAddress(usdcMint, wallet.publicKey);
    await createAccount(provider.connection, wallet.keypair, usdcMint, wallet.publicKey);
    await mintTo(provider.connection, wallet.keypair, usdcMint, userAta, wallet.publicKey, 1000000000);
  });

  it("Initialize", async () => {
    [vaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), wallet.publicKey.toBuffer(), Buffer.from([VAULT_ID])],
      program.programId
    );
    [vaultTokenPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("token_account"), vaultPda.toBuffer()],
      program.programId
    );

    await program.methods.initializeVault(VAULT_ID)
      .accounts({
        signer: wallet.publicKey,
        vault: vaultPda,
        vaultTokenAccount: vaultTokenPda,
        mint: usdcMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      }).rpc();
  });

  it("Deposit", async () => {
    await program.methods.deposit(VAULT_ID, amount100)
      .accounts({
        signer: wallet.publicKey,
        vault: vaultPda,
        userTokenAccount: userAta,
        vaultTokenAccount: vaultTokenPda,
        tokenProgram: TOKEN_PROGRAM_ID,
      }).rpc();
      
    const v = await program.account.collateralVault.fetch(vaultPda);
    
    // We check availableBalance because we know Lock uses it successfully
    const available = v.availableBalance || (v as any).available_balance;
    console.log("Available Balance:", available.toString());
    assert.equal(available.toString(), "100000000");
  });

  it("Lock", async () => {
    await program.methods.lockCollateral(VAULT_ID, amount20)
      .accounts({
        signer: wallet.publicKey,
        vault: vaultPda,
      }).rpc();
      
    const v = await program.account.collateralVault.fetch(vaultPda);
    const locked = v.lockedBalance || (v as any).locked_balance;
    console.log("Locked:", locked.toString());
    assert.equal(locked.toString(), "20000000");
  });
});