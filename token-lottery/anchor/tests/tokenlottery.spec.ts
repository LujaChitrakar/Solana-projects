import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { Tokenlottery } from "../target/types/tokenlottery";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("tokenlottery", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.Tokenlottery as Program<Tokenlottery>;

  it("Should init", async () => {
    /** initializing config */
    const initConfigIx = await program.methods
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
    }).add(initConfigIx);

    const signature = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      tx,
      [wallet.payer],
      { skipPreflight: true }
    );
    console.log("Your transaction signature is :", signature);

    /** initializing lottery */

    const initLotteryIx = await program.methods
      .initializeLottery()
      .accounts({ tokenProgram: TOKEN_PROGRAM_ID })
      .instruction();

    const initLotteryTx = new anchor.web3.Transaction({
      feePayer: provider.wallet.publicKey,
      blockhash: blockhasWithContext.blockhash,
      lastValidBlockHeight: blockhasWithContext.lastValidBlockHeight,
    }).add(initLotteryIx);

    const initLotterySignature = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      initLotteryTx,
      [wallet.payer]
    );
    console.log(
      "Your init Lottery  transaction  signature is :",
      initLotterySignature
    );
  });
});
