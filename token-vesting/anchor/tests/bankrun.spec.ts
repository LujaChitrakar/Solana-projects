import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import {
  BanksClient,
  Clock,
  ProgramTestContext,
  startAnchor,
} from "solana-bankrun";
import IDL from "../target/idl/tokenvesting.json";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { BankrunProvider } from "anchor-bankrun";
import { Tokenvesting } from "../target/types/tokenvesting";
import { Program, BN } from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { createMint, mintTo } from "spl-token-bankrun";
import { resolve } from "path";

describe("Vesting Smart contract Test", () => {
  const companyName = "company name";
  let beneficiary: Keypair;
  let context: ProgramTestContext;
  let provider: BankrunProvider;
  let program: Program<Tokenvesting>;
  let banksClient: BanksClient;
  let employer: Keypair;
  let mint: PublicKey;
  let beneficiary_provider: BankrunProvider;
  let program2: Program<Tokenvesting>;
  let tokenVestingAccountKey: PublicKey;
  let treasuryTokenAccountKey: PublicKey;
  let employeeAccountKey: PublicKey;

  beforeAll(async () => {
    beneficiary = new anchor.web3.Keypair();

    context = await startAnchor(
      "",
      [{ name: "tokenvesting", programId: new PublicKey(IDL.address) }],
      [
        {
          address: beneficiary.publicKey,
          info: {
            lamports: 1_000_000_000_000,
            data: Buffer.alloc(0),
            owner: SYSTEM_PROGRAM_ID,
            executable: false,
          },
        },
      ]
    );

    provider = new BankrunProvider(context);
    anchor.setProvider(provider);

    program = new Program<Tokenvesting>(IDL as Tokenvesting);
    banksClient = context.banksClient;

    employer = provider.wallet.payer;

    mint = await createMint(banksClient, employer, employer.publicKey, null, 2);

    beneficiary_provider = new BankrunProvider(context);
    beneficiary_provider.wallet = new NodeWallet(beneficiary);

    program2 = new Program<Tokenvesting>(
      IDL as Tokenvesting,
      beneficiary_provider
    );

    [tokenVestingAccountKey] = PublicKey.findProgramAddressSync(
      [Buffer.from(companyName)],
      program.programId
    );

    [treasuryTokenAccountKey] = PublicKey.findProgramAddressSync(
      [Buffer.from("vesting_treasury"), Buffer.from(companyName)],
      program.programId
    );

    [employeeAccountKey] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("employee_vesting"),
        beneficiary.publicKey.toBuffer(),
        tokenVestingAccountKey.toBuffer(),
      ],
      program.programId
    );
  });

  /**-- Writing Tests */

  it("Should create a vesting account", async () => {
    const tx = await program.methods
      .createVestingAccount(companyName)
      .accounts({
        signer: employer.publicKey,
        mint,
        token_program: TOKEN_PROGRAM_ID,
      })
      .rpc({ commitment: "confirmed" });

    const vestingAccountData = await program.account.vestingAccount.fetch(
      tokenVestingAccountKey,
      "confirmed"
    );

    console.log("Vesting account data: ", vestingAccountData, null, 2);
    console.log("Create vesting account", tx);
  });

  it("Should fund the treasury token account ", async () => {
    const amount = 10_000 * 10 ** 9;
    const mintTx = await mintTo(
      banksClient,
      employer,
      mint,
      treasuryTokenAccountKey,
      employer,
      amount
    );
    console.log("Mint Treasury Token Account", mintTx);
  });

  it("should create an employee vesting account", async () => {
    const mintDecimals = 2;

    // ENSURE this calculation results in a non-zero value
    const totalAmountBaseUnits = new BN(10_000 * 10 ** mintDecimals); // Example: 1,000,000
    console.log(
      `Calculated totalAmountBaseUnits to be passed: ${totalAmountBaseUnits.toString()}`
    ); // Add log

    const tx = await program.methods
      .createEmployeeAccount(new BN(0), new BN(100), new BN(100), new BN(0))
      .accounts({
        beneficiary: beneficiary.publicKey,
        vestingAccount: tokenVestingAccountKey,
      })
      .rpc({ commitment: "confirmed", skipPreflight: true });

    console.log("Employee vesting account created:", tx);
    console.log("Employee Account:", employeeAccountKey.toBase58());
  });

  // it("should claim tokens", async () => {
  //   const currentClock = await banksClient.getClock();

  //   console.log("Clock before setting:", currentClock.unixTimestamp.toString());
  //   const targetTimestamp = 101n;
  //   context.setClock(
  //     new Clock(
  //       currentClock.slot,
  //       currentClock.epochStartTimestamp,
  //       currentClock.epoch,
  //       currentClock.leaderScheduleEpoch,
  //       targetTimestamp
  //     )
  //   );

  //   console.log("Employee account", employeeAccountKey.toBase58());

  //   const tx3 = await program2.methods
  //     .claimTokens(companyName)
  //     .accounts({
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     })
  //     .rpc({ commitment: "confirmed" });

  //   console.log("Claim Tokens transaction signature", tx3);
  // });
});
