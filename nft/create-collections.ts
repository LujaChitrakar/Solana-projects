import {
  createNft,
  fetchDigitalAsset,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  airdropIfRequired,
  getExplorerLink,
  getKeypairFromFile,
} from "@solana-developers/helpers";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import {
  generateSigner,
  keypairIdentity,
  percentAmount,
} from "@metaplex-foundation/umi";

const connection = new Connection(clusterApiUrl("devnet"));

const user = await getKeypairFromFile();

await airdropIfRequired(
  connection,
  user.publicKey,
  1 * LAMPORTS_PER_SOL,
  0.5 * LAMPORTS_PER_SOL
);

console.log("Loaded user", user.publicKey);

const umi = createUmi(connection.rpcEndpoint);
umi.use(mplTokenMetadata());

const umiUser = umi.eddsa.createKeypairFromSecretKey(user.secretKey);
umi.use(keypairIdentity(umiUser));

console.log("UMI instance for user");

const collectionMint = generateSigner(umi);
const transaction = await createNft(umi, {
  mint: collectionMint,
  name: "My collections",
  symbol: "MC",
  uri: "https://raw.githubusercontent.com/LujaChitrakar/Solana-projects/refs/heads/main/new-token/metadata.json?token=GHSAT0AAAAAAC3UNGRCHMUDEVEJU4YZRST6Z7UXU3Q",
  sellerFeeBasisPoints: percentAmount(0),
  isCollection: true,
});
await transaction.sendAndConfirm(umi);

const createdCollectionNft = await fetchDigitalAsset(
  umi,
  collectionMint.publicKey
);

console.log(
  `Created collection . Address is ${getExplorerLink(
    "address",
    createdCollectionNft.publicKey,
    "devnet"
  )}`
);
