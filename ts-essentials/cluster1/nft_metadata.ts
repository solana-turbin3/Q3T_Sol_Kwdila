import wallet from "./wallet/wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {
    const image =
      "https://arweave.net/bcacosHOqEn9HvIwoxHczxy20ZNfAXYLOnzVQt80ZxY";
    const metadata = {
      name: "Aladin Magic Rug",
      symbol: "RUGG",
      description: "umi metadata test",
      image,
      attributes: [{ trait_type: "wishes left", value: "2" }],
      properties: {
        files: [
          {
            uri: image,
            type: "image/png",
          },
        ],
      },
    };

    const myUri = await umi.uploader.uploadJson(metadata);

    console.log("Your URI: ", myUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
