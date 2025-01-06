import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagers } from "../target/types/realm_voyagers";
import { expect } from "chai";
import { airdrop, confirmTransaction, getRealmPDA, getDimensionPDA, getJourneyPDA } from "./helpers";

describe("Test simple journey", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Program & keypairs
  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;
  const realmMaster = anchor.web3.Keypair.generate();
  const player = anchor.web3.Keypair.generate();

  // Realm & dimension datas
  const realmId = "journey_realm";
  const realmDescription = { name: "Test Realm", details: "A test realm details", logo: "https://example.com/logo123" };
  const dimension = {
    id: "dungeon_1",
    name: "Dungeon",
    areas: [
      {
        "name": "Test Area 1",
        "area": {
          "topLeft": {
            "x": 0,
            "y": 0
          },
          "bottomRight": {
            "x": 100,
            "y": 100
          }
        },
        "tileset": "https://example.com/tileset_1.png",
        "tilemap": "https://example.com/tilemap.json"
      },
      {
        "name": "Test Area 2",
        "area": {
          "topLeft": {
            "x": 0,
            "y": 100
          },
          "bottomRight": {
            "x": 150,
            "y": 100
          }
        },
        "tileset": "https://example.com/tileset_2.png",
        "tilemap": "https://example.com/tilemap.json"
      },
    ]
  };
  const startingPosition = { x: 10, y: -13 };

  // PDAs
  const realmPDA = getRealmPDA(realmId, program);
  const dimensionPDA = getDimensionPDA(realmId, dimension.id, program);
  const journeyPDA = getJourneyPDA(realmId, player.publicKey, program);

  it("Airdrop to realm master", async () => await airdrop(realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL));

  it("Create the realm", async () => {
    let tx = await program.methods
      .createRealm(realmId, realmDescription)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);
  });

  it("Add the dimension", async () => {
    let tx = await program.methods
      .addRealmDimension(realmId, dimension.id, dimension.name, dimension.areas)
      .accounts({
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);
  });

  it("Set the starting point", async () => {
    let tx = await program.methods
    .setRealmStartingPoint(realmId, dimension.id, startingPosition)
    .accounts({
        master: realmMaster.publicKey,
    })
    .signers([realmMaster])
    .rpc();
    await confirmTransaction(tx);
  });

  // TODO: add ability to pay for players from realm master account
  it("Airdrop to player", async () => await airdrop(player.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL));

  it("Join the realm as a player", async () => {
    const tx = await program.methods
      .startJourney(realmId)
      .accounts({
        player: player.publicKey,
      })
      .signers([player])
      .rpc();
    await confirmTransaction(tx);

    var journeyAccount = await program.account.journey.fetch(journeyPDA);
    expect(journeyAccount.realm.toBase58()).to.equal(realmPDA.toBase58());
    expect(journeyAccount.player.toBase58()).to.equal(player.publicKey.toBase58());
    expect(journeyAccount.dimension.toBase58()).to.equal(dimensionPDA.toBase58());
    expect(journeyAccount.position.x).to.equal(startingPosition.x);
    expect(journeyAccount.position.y).to.equal(startingPosition.y);
  });
});