import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

import { RealmVoyagers } from "../target/types/realm_voyagers";

import * as helper from "./utils/helpers";
import * as steps from "./utils/common-steps";

describe("Realm ownership flows", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Program & keypairs
  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;
  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();

  // Realm data
  const realmId = "realm_id_123";
  const realmDescription = { name: "Test Realm", details: "A test realm details", logo: "https://example.com/logo123" };
  const updatedRealmDescription = { name: "Test Realm 1", details: "An updated test realm details", logo: "https://example.com/logo123" };

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

  it("Airdrop to Alice", async () => await helper.airdrop(alice.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL));
  it("Alice creates a realm", async () => await steps.createRealm(alice, program, realmId, realmDescription, events));
  it("Airdrop to Bob", async () => await helper.airdrop(bob.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL));

  it("Bob tries to update the realm, and it fails", async () => {
    try {
      await program.methods
        .updateRealmDescription(realmId, updatedRealmDescription)
        .accounts({ master: bob.publicKey })
        .signers([bob])
        .rpc();
      expect.fail("Bob should not be authorized to update the realm");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }
  });

  it("Bob tries to add himself as a realm master, and it fails", async () => {
    try {
      await program.methods
        .addRealmMaster(realmId, bob.publicKey)
        .accounts({ master: bob.publicKey })
        .signers([bob])
        .rpc();
      expect.fail("Bob should not be authorized to add himself as realm master");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }
  });

  it("Alice adds Bob as a realm master", async () => await steps.addRealmMaster(alice, program, realmId, bob.publicKey, events));
  it("Bob updates the realm successfully", async () => await steps.updateRealmDescription(bob, program, realmId, updatedRealmDescription, events));

  it("Bob tries to transfer ownership to himself, and it fails", async () => {
    try {
      await program.methods
        .transferRealmOwnership(realmId, bob.publicKey)
        .accounts({ master: bob.publicKey })
        .signers([bob])
        .rpc();
      expect.fail("Bob should not be authorized to transfer realm ownership");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }

    // Ensure that the realm description has not been updated
    const realmPDA = helper.getRealmPDA(realmId, program);
    const realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.masters.length).to.equal(2);
    expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(alice.publicKey.toBase58());
    expect(realmAccount.masters[0].role).to.deep.equal({ owner: {} });
    expect(realmAccount.masters[1].pubkey.toBase58()).to.equal(bob.publicKey.toBase58());
    expect(realmAccount.masters[1].role).to.deep.equal({ admin: {} });
  });

  it("Alice transfers ownership to Bob", async () => await steps.transferRealmOwnership(alice, program, realmId, bob.publicKey, events));
  it("Bob removes Alice as a realm master", async () => await steps.removeRealmMaster(bob, program, realmId, alice.publicKey, events));

  it("Alice tries to update the realm, and it fails", async () => {
    try {
      await program.methods
        .updateRealmDescription(realmId, updatedRealmDescription)
        .accounts({ master: alice.publicKey })
        .signers([alice])
        .rpc();
      expect.fail("Alice should not be authorized to update the realm");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }
  });

  it("Bob deletes the realm", async () => await steps.deleteRealm(bob, program, realmId, [], events));
});
