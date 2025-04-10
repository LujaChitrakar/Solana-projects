import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { Tokenlottery } from "../target/types/tokenlottery";

describe("tokenlottery", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.Tokenlottery as Program<Tokenlottery>;

  it("Should init config", async () => {
    const initConfigTx = await program.methods
      .intializeConfig(
        new anchor.BN(0),
        new anchor.BN(1844281085),
        new anchor.BN(10000)
      )
      .instruction();

    const blockhasWithContext = await provider.connection.getLatestBlockhash();

    const tx = new anchor.web3.Transaction({
      feePayer: provider.wallet.publicKey,
      blockhash: blockhasWithContext.blockhash,
      lastValidBlockHeight: blockhasWithContext.lastValidBlockHeight,
    }).add(initConfigTx);

    const signature = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      tx,
      [wallet.payer]
    );
    console.log("Your transaction signature is :", signature);
  });
});
