import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

import { RealmVoyagers } from "../target/types/realm_voyagers";

import * as helper from "./utils/helpers";
import * as steps from "./utils/common-steps";

describe("Realm Management", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Program & keypairs
  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;
  const realmMaster = anchor.web3.Keypair.generate();

  // Realms data
  const firstRealmId = "realm_id_1";
  const secondRealmId = "realm_id_2";
  const firstRealmDescription = { name: "Test Realm 1", details: "A test realm", logo: "https://example.com/logo1" };
  const firstRealmUpdatedDescription = { name: "Updated Realm 1", details: "An updated description", logo: "https://example.com/logo1" };
  const secondRealmDescription = { name: "Test Realm 2", details: "Another test realm", logo: "https://example.com/logo2" };
  const secondRealmUpdatedDescription = { name: "Updated Realm 2", details: "Another updated description", logo: "https://example.com/logo2" };

  // Listen events
  let listener = null;
  let events = [];

  before(async () => {
    listener = program.addEventListener("realmEvent", (event) => {
      events.push(event);
    });
  });

  after(async () => {
    await program.removeEventListener(listener);
  });

  it("Airdrop to realm master", async () => await helper.airdrop(realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL));
  it("Create the first realm", async () => await steps.createRealm(realmMaster, program, firstRealmId, firstRealmDescription, events));

  it("Try to update unexisting realm, and it fails", async () => {
    try {
      await program.methods
        .updateRealmDescription("unexisting_realm", firstRealmUpdatedDescription)
        .accounts({ master: realmMaster.publicKey })
        .signers([realmMaster])
        .rpc();
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("AccountNotInitialized");
    }
  });

  it("Try to delete unexisting realm, and it fails", async () => {
    try {
      await program.methods
        .deleteRealm("unexisting_realm")
        .accounts({ master: realmMaster.publicKey })
        .signers([realmMaster])
        .rpc();
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("AccountNotInitialized");
    }
  });

  it("Create the second realm", async () => await steps.createRealm(realmMaster, program, secondRealmId, secondRealmDescription, events));
  it("Update first realm", async () => await steps.updateRealmDescription(realmMaster, program, firstRealmId, firstRealmUpdatedDescription, events));
  it("Delete the first realm", async () => await steps.deleteRealm(realmMaster, program, firstRealmId, [], events));
  it("Update second realm", async () => await steps.updateRealmDescription(realmMaster, program, secondRealmId, secondRealmUpdatedDescription, events));
  it("Delete the second realm", async () => await steps.deleteRealm(realmMaster, program, secondRealmId, [], events));

  it("Try to update deleted realm, and it fails", async () => {
    try {
      await program.methods
        .updateRealmDescription(firstRealmId, firstRealmUpdatedDescription)
        .accounts({ master: realmMaster.publicKey })
        .signers([realmMaster])
        .rpc();
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("AccountNotInitialized");
    }
  });
});
