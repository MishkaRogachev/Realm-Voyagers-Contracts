import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagers } from "../target/types/realm_voyagers";
import { expect } from "chai";
import { airdrop, confirmTransaction, getDimensionPDA, getRealmPDA } from "./helpers";

describe("Test realm with dimensions", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Program & keypairs
  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;
  const realmMaster = anchor.web3.Keypair.generate();

  // Realm & dimension datas
  const realmId = "realm_with_dimensions";
  const realmDescription = { name: "Test Realm", details: "A test realm details", logo: "https://example.com/logo123" };
  var dimensions = [
    { id: "rat_castle", name: "Rat Castle", tilemap: "https://example.com/castle-map", tileset: "https://example.com/castle-tileset" },
    { id: "dungeon_1", name: "Synth Dungeon", tilemap: "https://example.com/dungeon-map", tileset: "https://example.com/dungeon-tileset" },
    { id: "spaceship", name: "Witchcraft Spaceship", tilemap: "https://example.com/spaceship-map", tileset: "https://example.com/spaceship-tileset" },
  ];

  // PDAs
  const realmPDA = getRealmPDA(realmId, program);
  const dimensionPDAs = dimensions.map((dimension) => getDimensionPDA(realmId, dimension.id, program));

  it("Create realm, add some dimensions, update some and delete", async () => {
    // Add listener for events
    let events = [];
    let eventsCount = 0;
    let listener = program.addEventListener("dimensionEvent", (event) => {
      events.push(event);
    });

    await airdrop(realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Create the realm
    let tx = await program.methods
      .createRealm(realmId, realmDescription)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Add some dimensions
    for (let i = 0; i < dimensions.length; i++) {
      const dimension = dimensions[i];
      const dimensionPDA = dimensionPDAs[i];

      const tx = await program.methods
        .addRealmDimension(realmId, dimension.id, dimension.name, dimension.tileset, dimension.tilemap)
        .accounts({
          master: realmMaster.publicKey,
        })
        .signers([realmMaster])
        .rpc();
      await confirmTransaction(tx);

      // Fetch & assert dimension
      var dimensionAccount = await program.account.realmDimension.fetch(dimensionPDA);
      expect(dimensionAccount.name).to.equal(dimension.name);
      expect(dimensionAccount.tilemap).to.equal(dimension.tilemap);
      expect(dimensionAccount.tileset).to.equal(dimension.tileset);
      expect(dimensionAccount.realm.toBase58()).to.equal(realmPDA.toBase58());

      // Verify an add dimension event is emitted
      eventsCount++;
      expect(events.length).to.equal(eventsCount);
      expect(events[eventsCount - 1].eventType.dimensionAdded.name).to.equal(dimension.name);
      expect(events[eventsCount - 1].eventType.dimensionAdded.tilemap).to.equal(dimension.tilemap);
      expect(events[eventsCount - 1].eventType.dimensionAdded.tileset).to.equal(dimension.tileset);

      // Verify the realm has the dimension public key
      const realmAccount = await program.account.realm.fetch(realmPDA);
      expect(realmAccount.dimensions[i].toBase58()).to.equal(dimensionPDA.toBase58());
    }

    // Try to update unexisting dimension
    try {
      tx = await program.methods
        .updateRealmDimension(realmId, "unexisting_dimension", "New Name", "New Tileset", "New Tilemap")
        .accounts({
          master: realmMaster.publicKey,
        })
        .signers([realmMaster])
        .rpc();
      await confirmTransaction(tx);
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("AccountNotInitialized");
    }

    // Try to update dimension with wrong owner
    try {
      const pest = anchor.web3.Keypair.generate();
      tx = await program.methods
        .updateRealmDimension(realmId, dimensions[0].id, "New Name", "New Tileset", "New Tilemap")
        .accounts({
          master: pest.publicKey,
        })
        .signers([pest])
        .rpc();
      await confirmTransaction(tx);
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }

    // Verify no dimension event is emitted
    expect(events.length).to.equal(eventsCount);

    // Update dungeon dimension
    dimensions[1] = {
      id: "dungeon_1",
      name: "Updated Synth Dungeon",
      tilemap: "https://example.com/dungeon-map-updated",
      tileset: "https://example.com/dungeon-tileset-v2"
    };

    tx = await program.methods
      .updateRealmDimension(realmId, dimensions[1].id, dimensions[1].name, dimensions[1].tileset, dimensions[1].tilemap)
      .accounts({
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Verify an update dimension event is emitted
    eventsCount++;
    expect(events.length).to.equal(eventsCount);
    expect(events[eventsCount - 1].eventType.dimensionUpdated.name).to.equal(dimensions[1].name);
    expect(events[eventsCount - 1].eventType.dimensionUpdated.tilemap).to.equal(dimensions[1].tilemap);
    expect(events[eventsCount - 1].eventType.dimensionUpdated.tileset).to.equal(dimensions[1].tileset);

    // Delete spaceship dimension
    tx = await program.methods
      .removeRealmDimension(realmId, dimensions[2].id)
      .accounts({
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Verify a remove dimension event is emitted
    eventsCount++;
    expect(events.length).to.equal(eventsCount);
    expect(events[eventsCount - 1].eventType.dimensionRemoved).not;

    // Verify the realm dimension is removed
    const realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.dimensions.length).to.equal(2);
    expect(realmAccount.dimensions).to.not.include(dimensionPDAs[2]);

    // First dimension should be the same
    var dimensionAccount = await program.account.realmDimension.fetch(dimensionPDAs[0]);
    expect(dimensionAccount.name).to.equal(dimensions[0].name);
    expect(dimensionAccount.tilemap).to.equal(dimensions[0].tilemap);
    expect(dimensionAccount.tileset).to.equal(dimensions[0].tileset);
    expect(dimensionAccount.realm.toBase58()).to.equal(realmPDA.toBase58());

    // Second dimension should be updated
    dimensionAccount = await program.account.realmDimension.fetch(dimensionPDAs[1]);
    expect(dimensionAccount.name).to.equal(dimensions[1].name);
    expect(dimensionAccount.tilemap).to.equal(dimensions[1].tilemap);
    expect(dimensionAccount.tileset).to.equal(dimensions[1].tileset);
    expect(dimensionAccount.realm.toBase58()).to.equal(realmPDA.toBase58());

    // Third dimension should be deleted
    try {
      dimensionAccount = await program.account.realmDimension.fetch(dimensionPDAs[2]);
      expect.fail("dimension account should have been deleted");
    } catch (err) {
      expect(err.message).to.include("Account does not exist");
    }

    // Delete realm
    tx = await program.methods
      .deleteRealm(realmId)
      .accounts({ master: realmMaster.publicKey })
      .remainingAccounts([
        { pubkey: dimensionPDAs[0], isWritable: true, isSigner: false },
        { pubkey: dimensionPDAs[1], isWritable: true, isSigner: false }
      ])
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Verify the realm is deleted
    const realmInfo = await provider.connection.getAccountInfo(realmPDA);
    expect(realmInfo).to.be.null; // Ensure the account no longer exists

    // Verify realm loctions are deleted as well
    for (const dimensionPDA of dimensionPDAs) {
      try {
        dimensionAccount = await program.account.realmDimension.fetch(dimensionPDA);
        expect.fail("dimension account should have been deleted");
      } catch (err) {
        expect(err.message).to.include("Account does not exist");
      }
    }

    // Remove listener
    await program.removeEventListener(listener);
  });
});
