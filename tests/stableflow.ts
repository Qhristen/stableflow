import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { Stableflow } from "../target/types/stableflow";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  createMint,
  createAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  Account,
  getAssociatedTokenAddressSync,
  TOKEN_2022_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";
import { confirmTransaction } from "@solana-developers/helpers";
import { DEVNET_PROGRAM_ID } from "@raydium-io/raydium-sdk";
import { BN } from "bn.js";

describe("stableflow", async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Stableflow as Program<Stableflow>;
  const tokenProgram = TOKEN_PROGRAM_ID;
  // Generate a new keypair for the user
  const user = Keypair.generate();
  // Generate a new keypair for the user

  // Test accounts
  let vaultStatePda: PublicKey;
  let configPda: PublicKey;
  let userPda: PublicKey;
  let mint: PublicKey;
  let userATA: Account;
  let vault: PublicKey;

  // This variable is the base vault account.
  let creator_base_ata: PublicKey;

  // This variable is the token vault account.
  let creator_token_ata: PublicKey;

  // Raydium Observation State PDA
  let observation_state: PublicKey;

  // Raydium Pool PDA
  let pool_state: PublicKey;

  // Raydium Pool vault and lp mint authority PDA
  let authority: PublicKey;

  // Raydium base mint vault & token mint vault
  let token_vault_0: PublicKey;
  let token_vault_1: PublicKey;

  // Raydium lp_mint
  let lp_mint: PublicKey;

  // lp mint ata
  let lp_mint_ata: PublicKey;

  // locked pda
  let locked_liquidity: PublicKey;

  // locked pda
  let locked_lp_vault: PublicKey;

  // Address of the Rent program
  const RENT_PROGRAM = anchor.web3.SYSVAR_RENT_PUBKEY;

  const seed = "seed";
  // Test constants
  const FEE = 30; // 0.3%
  const INITIAL_MINT_AMOUNT = 1_000_000_000;

  before(async () => {
    await airdrop(user.publicKey, 2);
    // Derive PDAs
    configPda = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    )[0];

    userPda = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), user.publicKey.toBuffer()],
      program.programId
    )[0];

    vaultStatePda = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault_state"),
        user.publicKey.toBuffer(),
        Buffer.from(seed),
      ],
      program.programId
    )[0];

    vault = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), vaultStatePda.toBuffer()],
      program.programId
    )[0];

    mint = await createMint(provider.connection, user, user.publicKey, null, 6);
    userATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mint,
      user.publicKey
    );

    // Mint test tokens
    await mintTo(
      provider.connection,
      user,
      mint,
      userATA.address,
      user.publicKey,
      INITIAL_MINT_AMOUNT
    );
  });

  it("Initializes config", async () => {
    //     const balance = await provider.connection.getBalance(user.publicKey);
    // console.log(`Account balance: ${balance} lamports`);
    try {
      const tx = await program.methods
        .config(FEE)
        .accounts({
          admin: user.publicKey,
        })
        .signers([user])
        .rpc()
        .then(confirm)
        .then(log);

      const configState = await program.account.configState.fetch(configPda);
      assert.ok(configState.admin.equals(user.publicKey));
      assert.equal(configState.fee, FEE);
      assert.equal(configState.totalDeposits.toNumber(), 0);
      assert.equal(configState.totalYield.toNumber(), 0);
    } catch (error) {
      console.log(error, "init error");
    }
  });

  it("Initialize User", async () => {
    try {
      await program.methods
        .initializeUser()
        .accounts({
          user: user.publicKey,
        })
        .signers([user])
        .rpc()
        .then(confirm)
        .then(log);
    } catch (error) {
      console.log("user init error", error);
    }
  });

  it("Initialize Vault", async () => {
    try {
      await program.methods
        .initializeVault(seed)
        .accounts({
          user: user.publicKey,
          tokenMint: mint,
          tokenProgram,
        })
        .signers([user])
        .rpc()
        .then(confirm)
        .then(log);
    } catch (error) {
      console.log("vault init error", error);
    }
  });

  it("Deposit", async () => {
    const amount = new BN(100_000_000);
    try {
      await program.methods
        .deposit(amount)
        .accounts({
          config: configPda,
          user: user.publicKey,
          userAccount: userPda,
          tokenMint: mint,
          vault,
          vaultState: vaultStatePda,
          userTokenAccount: userATA.address,
          tokenProgram,
        })
        .signers([user])
        .rpc()
        .then(confirm)
        .then(log);
    } catch (error) {
      console.log("deposit error", error);
    }
  });

  it("Initialize withdrawal", async () => {
    const amount = new BN(50_000_000);
    // const balanceBefore = await getTokenBalance(userATA.address);
    // console.log("Balance before withdrawal:", balanceBefore);

    await program.methods
      .withdraw(amount)
      .accountsPartial({
        userAccount: userPda,
        user: user.publicKey,
        vault,
        tokenMint: mint,
        vaultState: vaultStatePda,
        userTokenAccount: userATA.address,
        tokenProgram,
        systemProgram: SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc()
      .then(confirm)
      .then(log);
  });

    it("Should fail with invalid fee", async () => {
    try {
      const invalidFee = 10100; // 101% fee
      await program.methods
        .config(invalidFee)
        .accounts({
          admin: user.publicKey,
        })
        .signers([user])
        .rpc();
      assert.fail("Expected to fail with InvalidFee error");
    } catch (error) {
      // assert.include(error.message, "Fee is greater than 100%");
    }
  });

  it("Should fail withdrawing more than deposited", async () => {
    const invalidAmount = new BN(3000000000);
    // const balanceBefore = await getTokenBalance(userATA.address);
    // console.log("Balance before withdrawal:", balanceBefore);

    try {
      await program.methods
        .withdraw(invalidAmount)
        .accountsPartial({
          userAccount: userPda,
          user: user.publicKey,
          vault,
          tokenMint: mint,
          vaultState: vaultStatePda,
          userTokenAccount: userATA.address,
          tokenProgram,
          systemProgram: SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();
      assert.fail("Expected to fail with InsufficientBalance error");
    } catch (error) {
      // assert.include(error.message, "Insufficient balance");
    }
  });

  it("Add external protocol", async () => {
    const pool_id = user.publicKey;
    const name = "Raydium";
    // const balanceBefore = await getTokenBalance(userATA.address);
    // console.log("Balance before withdrawal:", balanceBefore);

    await program.methods
      .addExternalProtocol(pool_id, name)
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc()
      .then(confirm)
      .then(log);
  });

  
  const airdrop = async (address: PublicKey, amount: number) => {
    let txn = await provider.connection.requestAirdrop(
      address,
      amount * LAMPORTS_PER_SOL
    );
    await confirmTransaction(provider.connection, txn, "confirmed");
  };

  async function getTokenBalance(tokenAccount: PublicKey): Promise<number> {
    const account = await getAccount(provider.connection, tokenAccount);
    return Number(account.amount) / Math.pow(10, 6); // Assuming 6 decimals
  }

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
      {
        signature,
        ...block,
      },
      "confirmed"
    );
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    // console.log(
    //   `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${provider.connection.rpcEndpoint}`
    // );
    return signature;
  };
});
