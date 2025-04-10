import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BanksClient, ProgramTestContext, startAnchor } from "solana-bankrun";
import IDL from "../target/idl/tokenvesting.json";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { BankrunProvider } from "anchor-bankrun";
import { Tokenvesting } from "../target/types/tokenvesting";
import { Program } from "@coral-xyz/anchor";
import { createMint } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

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

    //@ts-expect-error - Type error in spl-token-bankrun dependency
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
});
