import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import wallet from "./wallet/wba-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("HczKmbytGoFfGhVGBEnCjAw85MfbKwhRuUKFifJL6HVW");

const toKeypair = Keypair.generate();
// Recipient address
const to = new PublicKey(toKeypair.publicKey);

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const fromWalletAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    // Get the token account of the toWallet address, and if it does not exist, create it
    const toWalletAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      toKeypair.publicKey
    );
    // Transfer the new token to the "toTokenAccount" we just created

    const tx = await transfer(
      connection,
      keypair,
      fromWalletAta.address,
      toWalletAta.address,
      keypair,
      10
    );

    console.log("tx id: ", tx);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
