import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SecretHitler } from "../target/types/secret_hitler";
import { Clock, startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("Eliminate inactive player", () => {
  test("Turning forward time eliminates inactive player", async () => {
    const host = anchor.web3.Keypair.generate();

    const context = await startAnchor(
      ".",
      [],
      [
        {
          address: host.publicKey,
          info: {
            lamports: 100 * LAMPORTS_PER_SOL, // 1 SOL equivalent
            data: Buffer.alloc(0),
            owner: SYSTEM_PROGRAM_ID,
            executable: false,
          },
        },
      ],
    );
    const client = context.banksClient;

    const provider = new BankrunProvider(context);
    anchor.setProvider(provider);

    const program = anchor.workspace.SecretHitler as Program<SecretHitler>;

    let players: Keypair[] = [];

    let [gameData, gameDataBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("secret_hitler"), host.publicKey.toBuffer()],
      program.programId,
    );

    try {
      let tx = await program.methods
        .initializeGame(5, new anchor.BN(100), null, null)
        .accountsPartial({
          host: host.publicKey,
          gameData,
        })
        .signers([host])
        .rpc({ skipPreflight: true });
      console.log("Init transaction", tx);

      console.log("Confirmed", tx);
    } catch (e) {
      console.log("Game already exists: ", e);
    }

    for (let i = 0; i < 5; i++) {
      console.log(`Join game ${i}`);
      let player = Keypair.generate();
      players.push(player);

      const ixs = [
        SystemProgram.transfer({
          fromPubkey: host.publicKey,
          toPubkey: player.publicKey,
          lamports: 1 * LAMPORTS_PER_SOL,
        }),
      ];
      const tx_1 = new Transaction();
      const blockhash = context.lastBlockhash;
      tx_1.recentBlockhash = blockhash;
      tx_1.add(...ixs);
      tx_1.sign(host);
      await client.processTransaction(tx_1);

      let tx = await program.methods
        .joinGame()
        .accountsPartial({
          player: player.publicKey,
          gameData,
        })
        .signers([player])
        .rpc();

      console.log("join game instruction", tx);
    }

    try {
      let tx = await program.methods
        .startGame()
        .accountsPartial({
          host: host.publicKey,
          gameData,
        })
        .signers([host])
        .rpc({ skipPreflight: true });
      console.log("Start transaction", tx);

      console.log("Confirmed", tx);
    } catch (e) {
      console.log("Game already started: ", e);
    }

    await client.getAccount(gameData).then((info) => {
      const decoded = program.coder.accounts.decode(
        "secret_hitler",
        Buffer.from(info.data),
      );
      console.log("Game account info", JSON.stringify(decoded));
      expect(decoded).toBeDefined();
      expect(parseInt(Object.keys(decoded.gameState)[0])).toEqual(
        "chancellorNomination",
      );
    });

    const timestamp = Math.floor(Date.now() / 1000);

    // Turn forward the clo/ck for 11 minutes
    const currentClock = await client.getClock();
    context.setClock(
      new Clock(
        currentClock.slot,
        currentClock.epochStartTimestamp,
        currentClock.epoch,
        currentClock.leaderScheduleEpoch,
        BigInt(timestamp) + BigInt(60 * 2),
      ),
    );

    let tx = await program.methods
      .eliminateInactivePlayer()
      .accountsPartial({
        player: host.publicKey,
        gameData,
      })
      .rpc();

    // // Get the account again and check if the energy is updated.
    // // Its 99 because the last chop also costs on energy again.
    // await client.getAccount(playerPDA).then((info) => {
    //   const decoded = program.coder.accounts.decode(
    //     "playerData",
    //     Buffer.from(info.data),
    //   );
    //   console.log("Player account info", JSON.stringify(decoded));
    //   expect(decoded).toBeDefined();
    //   expect(parseInt(decoded.energy)).toEqual(99);
    // });
  }, 10000);
});
