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
  const exampleArea = {
    "name": "Example Area",
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
    "tileset": "https://example.com/tileset.png",
    "tilemap": "https://example.com/tilemap.json"
  }

  var dimensions = [
    { id: "rat_castle", name: "Rat Castle", areas: [exampleArea] },
    { id: "dungeon_1", name: "Synth Dungeon", areas: [] },
    { id: "spaceship", name: "Witchcraft Spaceship", areas: [] },
  ];

  // PDAs
  const realmPDA = getRealmPDA(realmId, program);
  const dimensionPDAs = dimensions.map((dimension) => getDimensionPDA(realmId, dimension.id, program));

  // Add listener for events
  let events = [];
  let eventsCount = 0;
  let listener = program.addEventListener("realmDimensionEvent", (event) => {
    events.push(event);
  });

  it("Create the realm", async () => {
    await airdrop(realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    let tx = await program.methods
      .createRealm(realmId, realmDescription)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);
  });

  it("Add some realm dimensions", async () => {
    for (let i = 0; i < dimensions.length; i++) {
      const dimension = dimensions[i];
      const dimensionPDA = dimensionPDAs[i];

      const tx = await program.methods
        .addRealmDimension(realmId, dimension.id, dimension.name, dimension.areas)
        .accounts({
          master: realmMaster.publicKey,
        })
        .signers([realmMaster])
        .rpc();
      await confirmTransaction(tx);

      var dimensionAccount = await program.account.realmDimension.fetch(dimensionPDA);
      expect(dimensionAccount.name).to.equal(dimension.name);
      expect(dimensionAccount.realm.toBase58()).to.equal(realmPDA.toBase58());
      expect(dimensionAccount.areas).to.deep.equal(dimension.areas);

      eventsCount++;
      expect(events.length).to.equal(eventsCount);
      expect(events[eventsCount - 1].eventType.dimensionAdded.name).to.equal(dimension.name);

      const realmAccount = await program.account.realm.fetch(realmPDA);
      expect(realmAccount.dimensions[i].toBase58()).to.equal(dimensionPDA.toBase58());
    }
  });

  it("Try to update unexisting dimension", async () => {
    try {
      const tx = await program.methods
        .updateRealmDimension(realmId, "unexisting_dimension", "New Name", [])
        .accounts({
          master: realmMaster.publicKey,
        })
        .signers([realmMaster])
        .rpc();
      await confirmTransaction(tx);
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("AccountNotInitialized");
    }
  });

  it("Try to remove unexisting dimension", async () => {
    try {
      const tx = await program.methods
        .removeRealmDimension(realmId, "unexisting_dimension")
        .accounts({
          master: realmMaster.publicKey,
        })
        .signers([realmMaster])
        .rpc();
      await confirmTransaction(tx);
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("AccountNotInitialized");
    }
    expect(events.length).to.equal(eventsCount);
  });

  it("Update dungeon dimension", async () => {
    dimensions[1] = {
      id: "dungeon_1",
      name: "Updated Synth Dungeon",
      areas: [exampleArea]
    };

    const tx = await program.methods
      .updateRealmDimension(realmId, dimensions[1].id, dimensions[1].name, dimensions[1].areas)
      .accounts({
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    eventsCount++;
    expect(events.length).to.equal(eventsCount);
    expect(events[eventsCount - 1].eventType.dimensionUpdated.name).to.equal(dimensions[1].name);

    const dimensionAccount = await program.account.realmDimension.fetch(dimensionPDAs[1]);
    expect(dimensionAccount.name).to.equal(dimensions[1].name);
    expect(dimensionAccount.realm.toBase58()).to.equal(realmPDA.toBase58());
    expect(dimensionAccount.areas).to.deep.equal(dimensions[1].areas);
  });

  it("Delete spaceship dimension", async () => {
    const tx = await program.methods
      .removeRealmDimension(realmId, dimensions[2].id)
      .accounts({
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();

    eventsCount++;
    expect(events.length).to.equal(eventsCount);
    expect(events[eventsCount - 1].eventType.dimensionRemoved).not;

    try {
      await program.account.realmDimension.fetch(dimensionPDAs[2]);
      expect.fail("dimension account should have been deleted");
    } catch (err) {
      expect(err.message).to.include("Account does not exist");
    }
  });

  it("Delete realm", async () => {
    const tx = await program.methods
      .deleteRealm(realmId)
      .accounts({ master: realmMaster.publicKey })
      .remainingAccounts([
        { pubkey: dimensionPDAs[0], isWritable: true, isSigner: false },
        { pubkey: dimensionPDAs[1], isWritable: true, isSigner: false }
      ])
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    const realmInfo = await provider.connection.getAccountInfo(realmPDA);
    expect(realmInfo).to.be.null;

    // Verify realm loctions are deleted as well
    for (const dimensionPDA of dimensionPDAs) {
      try {
        await program.account.realmDimension.fetch(dimensionPDA);
        expect.fail("dimension account should have been deleted");
      } catch (err) {
        expect(err.message).to.include("Account does not exist");
      }
    }

    // Remove listener
    await program.removeEventListener(listener);
  });
});
