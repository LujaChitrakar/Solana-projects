import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { Voting } from "../target/types/voting";
import { BankrunProvider, startAnchor } from "anchor-bankrun";

const IDL = require("../target/idl/voting.json");

const votingAddress = new PublicKey(
  "coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF"
);

describe("Voting", () => {
  let context;
  let provider;
  let votingProgram: anchor.Program<Voting>;

  beforeAll(async () => {
    context = await startAnchor(
      "",
      [{ name: "voting", programId: votingAddress }],
      []
    );

    provider = new BankrunProvider(context);

    votingProgram = new Program<Voting>(IDL, provider);
  });

  it("Initialize Poll", async () => {
    await votingProgram.methods
      .initializePoll(
        new anchor.BN(1),
        "What is your favorite type of peanut butter?",
        new anchor.BN(0), //poll start time
        new anchor.BN(1843746801) //poll end time
      )
      .rpc();

    const [pollAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      votingAddress
    );

    const poll = await votingProgram.account.poll.fetch(pollAddress);
    console.log(poll);

    expect(poll.pollId.toNumber()).toEqual(1);
    expect(poll.description).toEqual(
      "What is your favorite type of peanut butter?"
    );
    expect(poll.pollStart.toNumber()).toEqual(0);
    expect(poll.pollEnd.toNumber()).toEqual(1843746801);
    expect(poll.candidateAmount.toNumber()).toEqual(0);
  });

  it("Initialize Candidate", async () => {
    await votingProgram.methods
      .initializeCandidate("Smooth", new anchor.BN(1))
      .rpc();
    await votingProgram.methods
      .initializeCandidate("Crunchy", new anchor.BN(1))
      .rpc();

    const [crunchyAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("Crunchy")],
      votingAddress
    );
    const [smoothAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("Smooth")],
      votingAddress
    );

    const crunchyCandidate = await votingProgram.account.candidate.fetch(
      crunchyAddress
    );
    const smoothCandidate = await votingProgram.account.candidate.fetch(
      smoothAddress
    );

    expect(crunchyCandidate.candidateVotes.toNumber()).toEqual(0);
    expect(smoothCandidate.candidateVotes.toNumber()).toEqual(0);
    console.log(crunchyCandidate, smoothCandidate);
  });

  it("Vote", async () => {
    await votingProgram.methods.vote("Smooth", new anchor.BN(1)).rpc();

    const [smoothAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("Smooth")],
      votingAddress
    );
    const smoothCandidate = await votingProgram.account.candidate.fetch(
      smoothAddress
    );
    expect(smoothCandidate.candidateVotes.toNumber()).toEqual(1);
  });
});

// anchor test --skip-local-validator --skip-deploy
