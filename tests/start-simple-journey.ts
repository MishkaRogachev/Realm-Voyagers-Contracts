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
    tilemap: "https://example.com/dungeon-map-1",
    tileset: "https://example.com/dungeon-tileset"
  };
  const startingPosition = { x: 10, y: -13 };

  // PDAs
  const realmPDA = getRealmPDA(realmId, program);
  const dimensionPDA = getDimensionPDA(realmId, dimension.id, program);
  const journeyPDA = getJourneyPDA(realmId, player.publicKey, program);

  it("Create realm, add a dimension and join it as a player", async () => {
    await airdrop(realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
    // TODO: add ability to pay for players from realm master account
    await airdrop(player.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Create the realm
    let tx = await program.methods
      .createRealm(realmId, realmDescription)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Add the dimension
    tx = await program.methods
      .addRealmDimension(realmId, dimension.id, dimension.name)
      .accounts({
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Set the starting point for the realm
    tx = await program.methods
      .setRealmStartingPoint(realmId, dimension.id, startingPosition)
      .accounts({
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Join the realm as a player
    tx = await program.methods
      .startJourney(realmId)
      .accounts({
        player: player.publicKey,
      })
      .signers([player])
      .rpc();
    await confirmTransaction(tx);

    // Fetch & assert journey
    var journeyAccount = await program.account.journey.fetch(journeyPDA);
    expect(journeyAccount.realm.toBase58()).to.equal(realmPDA.toBase58());
    expect(journeyAccount.player.toBase58()).to.equal(player.publicKey.toBase58());
    expect(journeyAccount.dimension.toBase58()).to.equal(dimensionPDA.toBase58());
    expect(journeyAccount.position.x).to.equal(startingPosition.x);
    expect(journeyAccount.position.y).to.equal(startingPosition.y);
  });
});