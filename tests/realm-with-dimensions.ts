import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

import { RealmVoyagers } from "../target/types/realm_voyagers";

import * as helper from "./utils/helpers";
import * as steps from "./utils/common-steps";

describe("Test realm with dimensions", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Program & keypairs
  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;
  const realmMaster = anchor.web3.Keypair.generate();

  // Realm data
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

  // Listen events
  let listeners = [];
  let events = [];

  before(async () => {
    listeners.push(program.addEventListener("realmEvent", (event) => {
      events.push(event);
    }));
    listeners.push(program.addEventListener("realmDimensionEvent", (event) => {
      events.push(event);
    }));
  });

  after(async () => {
    for (const listener of listeners) {
      await program.removeEventListener(listener);
    }
  });

  it("Airdrop to realm master", async () => await helper.airdrop(realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL));
  it("Create the realm", async () => await steps.createRealm(realmMaster, program, realmId, realmDescription, events));

  for (const dimension of dimensions) {
    it("Add realm dimension: " + dimension.name, async () => steps.addRealmDimension(realmMaster, program, realmId, dimension, events));
  }

  it("Try to update unexisting dimension", async () => {
    try {
      await program.methods
        .updateRealmDimension(realmId, "unexisting_dimension", "New Name", [])
        .accounts({
          master: realmMaster.publicKey,
        })
        .signers([realmMaster])
        .rpc();
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("AccountNotInitialized");
    }
  });

  it("Try to remove unexisting dimension", async () => {
    try {
      await program.methods
        .removeRealmDimension(realmId, "unexisting_dimension")
        .accounts({
          master: realmMaster.publicKey,
        })
        .signers([realmMaster])
        .rpc();
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("AccountNotInitialized");
    }
  });

  it("Update dungeon dimension", async () => steps.updateRealmDimension(realmMaster, program, realmId, dimensions[1], events));
  it("Remove spaceship dimension", async () => steps.removeRealmDimension(realmMaster, program, realmId, dimensions[2].id, events));
  it("Delete realm", async () => steps.deleteRealm(realmMaster, program, realmId, [dimensions[0].id, dimensions[1].id], events));

  it("Verify realm dimensions are deleted as well", async () => {
    for (const dimension of dimensions) {
      try {
        const dimensionPDA = helper.getDimensionPDA(realmId, dimension.id, program);
        await program.account.realmDimension.fetch(dimensionPDA);
        expect.fail("dimension account should have been deleted");
      } catch (err) {
        expect(err.message).to.include("Account does not exist");
      }
    }
  });
});
