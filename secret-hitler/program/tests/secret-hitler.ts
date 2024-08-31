import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SecretHitler } from "../target/types/secret_hitler";
import { LAMPORTS_PER_SOL, PublicKey, Signer } from "@solana/web3.js";
import { assert } from "chai";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

describe("secret-hitler", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SecretHitler as Program<SecretHitler>;
  const payer = provider.wallet as anchor.Wallet;

  let maxPlayers = 6;
  let turnDuration = new anchor.BN(120);

  let depositAmount = new anchor.BN(1 * LAMPORTS_PER_SOL);
  let betAmount = new anchor.BN(0.5 * LAMPORTS_PER_SOL);

  let host = anchor.web3.Keypair.generate();

  let player_1 = anchor.web3.Keypair.generate();
  let player_2 = anchor.web3.Keypair.generate();
  let player_3 = anchor.web3.Keypair.generate();
  let player_4 = anchor.web3.Keypair.generate();
  let players = [player_1, player_2, player_3, player_4];

  let [gameData, gameDataBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("secret_hitler"), host.publicKey.toBuffer()],
    program.programId,
  );
  let [hostData, hostDataBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("player_data"),
      gameData.toBuffer(),
      host.publicKey.toBuffer(),
    ],
    program.programId,
  );
  let [depositVault, depositBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("deposit_vault"), gameData.toBuffer()],
      program.programId,
    );
  let [betVault, betbump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bet_vault"), gameData.toBuffer()],
    program.programId,
  );

  players.forEach(async (player) => {
    await airdrop(provider.connection, player.publicKey);
  });

  let [gameData_1, gameDataBump_1] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("secret_hitler"), player_1.publicKey.toBuffer()],
      program.programId,
    );
  let playerData_1 = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("player_data"),
      gameData_1.toBuffer(),
      player_1.publicKey.toBuffer(),
    ],
    program.programId,
  )[0];

  it("Init host game with deposit and bet included", async () => {
    await airdrop(provider.connection, host.publicKey);

    await program.methods
      .initializeGame(maxPlayers, turnDuration, depositAmount, betAmount)
      .accountsPartial({
        host: host.publicKey,
        gameData,
        playerData: hostData,
        betVault,
        depositVault,
      })
      .signers([host])
      .rpc()
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "1",
      "expected player count is 1 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      gameAfter.bump.toString(),
      gameDataBump.toString(),
      "wrong bump detected",
    );
  });
  it("Init player_1 game without deposit and bet", async () => {
    await program.methods
      .initializeGame(maxPlayers, turnDuration, null, null)
      .accountsPartial({
        host: player_1.publicKey,
        gameData: gameData_1,
        playerData: playerData_1,
        betVault: null,
        depositVault: null,
      })
      .signers([player_1])
      .rpc({ skipPreflight: true })
      .then(confirmTx);
    let gameAfter = await program.account.gameData.fetch(gameData_1);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "1",
      "expected player count is 1 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      gameAfter.bump.toString(),
      gameDataBump_1.toString(),
      "wrong bump detected",
    );
  });
  it("Join host game", async () => {
    await Promise.all(
      players.map(async (player) => {
        let playerData = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("player_data"),
            gameData.toBuffer(),
            player.publicKey.toBuffer(),
          ],
          program.programId,
        )[0];
        await program.methods
          .joinGame()
          .accountsPartial({
            player: player.publicKey,
            playerData,
            gameData,
            depositVault,
            betVault,
          })
          .signers([player])
          .rpc()
          .then(confirmTx);
      }),
    );
    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "5",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
  });
  it("Join initialized player_1 game", async () => {
    let player = players[2].publicKey;
    let playerData = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("player_data"), gameData_1.toBuffer(), player.toBuffer()],
      program.programId,
    )[0];
    await program.methods
      .joinGame()
      .accountsPartial({
        player,
        playerData,
        gameData: gameData_1,
        depositVault: null,
        betVault: null,
      })
      .signers([players[2]])
      .rpc()
      .then(confirmTx);
    let gameAfter = await program.account.gameData.fetch(gameData_1);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "2",
      "expected player count is 2 but got " + gameAfter.activePlayers.length,
    );
  });
  it("leave game player_1 game", async () => {
    let player = players[2].publicKey;
    let playerData = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("player_data"), gameData_1.toBuffer(), player.toBuffer()],
      program.programId,
    )[0];
    await program.methods
      .leaveGame()
      .accountsPartial({
        player: player_3.publicKey,
        playerData,
        gameData: gameData_1,
        depositVault: null,
        betVault: null,
      })
      .signers([players[2]])
      .rpc()
      .then(confirmTx);
    let gameAfter = await program.account.gameData.fetch(gameData_1);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "1",
      "expected player count is 1 but got " + gameAfter.activePlayers.length,
    );
  });
  it("Start host game", async () => {
    await airdrop(provider.connection, host.publicKey);

    await program.methods
      .startGame()
      .accountsPartial({
        host: host.publicKey,
        gameData,
      })
      .signers([host])
      .rpc()
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "5",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "chancellorNomination",
      "wrong game state detected",
    );
  });
});

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    "confirmed",
  );
};
async function airdrop(
  connection: anchor.web3.Connection,
  address: PublicKey,
  amount = 10 * LAMPORTS_PER_SOL,
) {
  await connection.requestAirdrop(address, amount).then(confirmTx);
}
