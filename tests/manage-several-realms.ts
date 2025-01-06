import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagers } from "../target/types/realm_voyagers";

import * as helper from "./helpers";
import * as steps from "./common-steps";

describe("Manage several realms", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Program & keypairs
  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;
  const realmMaster = anchor.web3.Keypair.generate();

  // Realm datas
  const firstRealmId = "realm_id_1";
  const secondRealmId = "realm_id_2";
  const firstRealmDescription = { name: "Test Realm 1", details: "A test realm", logo: "https://example.com/logo1" };
  const updatedDescription = { name: "Updated Realm 1", details: "An updated description", logo: "https://example.com/logo1" };
  const secondRealmDescription = { name: "Test Realm 2", details: "Another test realm", logo: "https://example.com/logo2" };

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
  it("Create the second realm", async () => await steps.createRealm(realmMaster, program, secondRealmId, secondRealmDescription, events));
  it("Update first realm", async () => await steps.updateRealmDescription(realmMaster, program, firstRealmId, updatedDescription, events));
  it("Delete the first realm", async () => await steps.deleteRealm(realmMaster, program, firstRealmId, events));
  it("Delete the second realm", async () => await steps.deleteRealm(realmMaster, program, secondRealmId, events));
});
