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
      "https://arweave.net/CdyIXOzLgyWwRiu7wp8M4V7AvKLeolLXopzFEwxYDqU";
    const metadata = {
      name: "bugz bunny coin",
      symbol: "bugz",
      description: "uni metadata test",
      image,
      attributes: [{ trait_type: "carrot", value: "10" }],
      properties: {
        files: [
          {
            uri: "https://www.arweave.net/abcd5678?ext=png",
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
