import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagers } from "../target/types/realm_voyagers";
import { expect } from "chai";
import { airdrop, confirmTransaction, getLocationPDA, getRealmPDA } from "./helpers";

describe("Test realm with locations", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;
  const realmMaster = anchor.web3.Keypair.generate();

  it("Create realm, add some locations, update some and delete", async () => {
    await airdrop(realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Realm & location datas
    const realmId = "realm_with_locations";
    var locations = [
      { id: "rat_castle", name: "Rat Castle", tilemap: "https://example.com/castle-map", tileset: "https://example.com/castle-tileset" },
      { id: "dungeon_1", name: "Synth Dungeon", tilemap: "https://example.com/dungeon-map", tileset: "https://example.com/dungeon-tileset" },
      { id: "spaceship", name: "Witchcraft Spaceship", tilemap: "https://example.com/spaceship-map", tileset: "https://example.com/spaceship-tileset" },
    ];

    // Realm PDA
    const realmPDA = getRealmPDA(realmId, program);
    const locationPDAs = locations.map((location) => getLocationPDA(realmId, location.id, program));

    // Add listener for events
    let events = [];
    let eventsCount = 0;
    let listener = program.addEventListener("locationEvent", (event) => {
      events.push(event);
    });

    // Create first realm
    let tx = await program.methods
      .createRealm(realmId, "Test Realm", "A test realm")
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Add some Locations
    for (let i = 0; i < locations.length; i++) {
      const location = locations[i];
      const locationPDA = locationPDAs[i];

      const tx = await program.methods
        .addRealmLocation(realmId, location.id, location.name, location.tileset, location.tilemap)
        .accounts({
          master: realmMaster.publicKey,
        })
        .signers([realmMaster])
        .rpc();
      await confirmTransaction(tx);

      // Fetch location
      var locationAccount = await program.account.realmLocation.fetch(locationPDA);

      // Assertions for the first realm
      expect(locationAccount.name).to.equal(location.name);
      expect(locationAccount.tilemap).to.equal(location.tilemap);
      expect(locationAccount.tileset).to.equal(location.tileset);
      expect(locationAccount.realm.toBase58()).to.equal(realmPDA.toBase58());

      // Verify an add location event is emitted
      eventsCount++;
      expect(events.length).to.equal(eventsCount);
      expect(events[eventsCount - 1].eventType.locationAdded.name).to.equal(location.name);
      expect(events[eventsCount - 1].eventType.locationAdded.tilemap).to.equal(location.tilemap);
      expect(events[eventsCount - 1].eventType.locationAdded.tileset).to.equal(location.tileset);

      // Verify the realm has the location public key
      const realmAccount = await program.account.realm.fetch(realmPDA);
      expect(realmAccount.locations[i].toBase58()).to.equal(locationPDA.toBase58());
    }

    // Try to update unexisting location
    try {
      tx = await program.methods
        .updateRealmLocation(realmId, "unexisting_location", "New Name", "New Tileset", "New Tilemap")
        .accounts({
          master: realmMaster.publicKey,
        })
        .signers([realmMaster])
        .rpc();
      await confirmTransaction(tx);
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("AccountNotInitialized");
    }

    // Try to update location with wrong owner
    try {
      const pest = anchor.web3.Keypair.generate();
      tx = await program.methods
        .updateRealmLocation(realmId, locations[0].id, "New Name", "New Tileset", "New Tilemap")
        .accounts({
          master: pest.publicKey,
        })
        .signers([pest])
        .rpc();
      await confirmTransaction(tx);
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }

    // Verify no location event is emitted
    expect(events.length).to.equal(eventsCount);

    // Update dungeon location
    locations[1] = {
      id: "dungeon_1",
      name: "Updated Synth Dungeon",
      tilemap: "https://example.com/dungeon-map-updated",
      tileset: "https://example.com/dungeon-tileset-v2"
    };

    tx = await program.methods
      .updateRealmLocation(realmId, locations[1].id, locations[1].name, locations[1].tileset, locations[1].tilemap)
      .accounts({
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Verify an update location event is emitted
    eventsCount++;
    expect(events.length).to.equal(eventsCount);
    expect(events[eventsCount - 1].eventType.locationUpdated.name).to.equal(locations[1].name);
    expect(events[eventsCount - 1].eventType.locationUpdated.tilemap).to.equal(locations[1].tilemap);
    expect(events[eventsCount - 1].eventType.locationUpdated.tileset).to.equal(locations[1].tileset);

    // Delete spaceship location
    tx = await program.methods
      .removeRealmLocation(realmId, locations[2].id)
      .accounts({
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Verify a remove location event is emitted
    eventsCount++;
    expect(events.length).to.equal(eventsCount);
    expect(events[eventsCount - 1].eventType.locationRemoved).not;

    // Verify the realm location is removed
    const realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.locations.length).to.equal(2);
    expect(realmAccount.locations).to.not.include(locationPDAs[2]);

    // First location should be the same
    var locationAccount = await program.account.realmLocation.fetch(locationPDAs[0]);
    expect(locationAccount.name).to.equal(locations[0].name);
    expect(locationAccount.tilemap).to.equal(locations[0].tilemap);
    expect(locationAccount.tileset).to.equal(locations[0].tileset);
    expect(locationAccount.realm.toBase58()).to.equal(realmPDA.toBase58());

    // Second location should be updated
    locationAccount = await program.account.realmLocation.fetch(locationPDAs[1]);
    expect(locationAccount.name).to.equal(locations[1].name);
    expect(locationAccount.tilemap).to.equal(locations[1].tilemap);
    expect(locationAccount.tileset).to.equal(locations[1].tileset);
    expect(locationAccount.realm.toBase58()).to.equal(realmPDA.toBase58());

    // Third location should be deleted
    try {
      locationAccount = await program.account.realmLocation.fetch(locationPDAs[2]);
      expect.fail("Location account should have been deleted");
    } catch (err) {
      expect(err.message).to.include("Account does not exist");
    }

    // Delete realm
    tx = await program.methods
      .deleteRealm(realmId)
      .accounts({ master: realmMaster.publicKey })
      .remainingAccounts([
        { pubkey: locationPDAs[0], isWritable: true, isSigner: false },
        { pubkey: locationPDAs[1], isWritable: true, isSigner: false }
      ])
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Verify the realm is deleted
    const realmInfo = await provider.connection.getAccountInfo(realmPDA);
    expect(realmInfo).to.be.null; // Ensure the account no longer exists

    // Verify realm loctions are deleted as well
    for (const locationPDA of locationPDAs) {
      try {
        locationAccount = await program.account.realmLocation.fetch(locationPDA);
        expect.fail("Location account should have been deleted");
      } catch (err) {
        expect(err.message).to.include("Account does not exist");
      }
    }

    // Remove listener
    await program.removeEventListener(listener);
  });
});
