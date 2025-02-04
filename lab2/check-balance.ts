import "dotenv/config";
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";

import { airdropIfRequired} from "@solana-developers/helpers";

const connection = new Connection(clusterApiUrl("devnet"));

console.log("Connected to devnet");

const publicKey = new PublicKey("47fX22LwxRL6eRYLvUWPedeR5xWnyzmmPapV5R8fTPiQ");

const balanceInLamports = await connection.getBalance(publicKey);

console.log("Done! Balance in lamports is", balanceInLamports);

const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;

await airdropIfRequired(
  connection,
  publicKey,
  1 * LAMPORTS_PER_SOL,
  0.5 * LAMPORTS_PER_SOL
);

console.log("Airdrop done!");

const balanceInLamports2 = await connection.getBalance(publicKey);
console.log("Done! Balance in lamports is", balanceInLamports2);

