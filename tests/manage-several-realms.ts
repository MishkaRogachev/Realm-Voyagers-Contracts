import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagers } from "../target/types/realm_voyagers";
import { expect } from "chai";
import { airdrop, confirmTransaction, getRealmPDA } from "./helpers";

describe("Manage several realms", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;
  const realmMaster = anchor.web3.Keypair.generate();

  it("Create realm, add some locations and delete", async () => {
    await airdrop(realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Realm datas
    const firstRealmId = "realm_id_1";
    const secondRealmId = "realm_id_2";
    const firstRealmName = "Test Realm 1";
    const secondRealmName = "Test Realm 2";
    const firstRealmDescription = "A test realm";
    const secondRealmDescription = "Another test realm";

    // Realm PDAs
    const firstRealmPDA = getRealmPDA(firstRealmId, program);
    const secondRealmPDA = getRealmPDA(secondRealmId, program);

    // Add listener for events
    let events = [];
    let listener = program.addEventListener("realmEvent", (event) => {
      events.push(event);
    });

    // Create first realm
    let tx = await program.methods
      .createRealm(firstRealmId, firstRealmName, firstRealmDescription)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Fetch first realm account
    var realmAccount = await program.account.realm.fetch(firstRealmPDA);

    // Assertions for the first realm
    expect(realmAccount.name).to.equal(firstRealmName);
    expect(realmAccount.description).to.equal(firstRealmDescription);
    expect(realmAccount.createdAt).to.be.not.null;

    // Verify the first realm event is emitted
    expect(events.length).to.equal(1);
    expect(events[0].eventType.realmCreated.name).to.equal(firstRealmName);
    expect(events[0].eventType.realmCreated.description).to.equal(firstRealmDescription);
    expect(events[0].realmPubkey.toBase58()).to.equal(firstRealmPDA.toBase58());

    // Create second realm
    tx = await program.methods
      .createRealm(secondRealmId, secondRealmName, secondRealmDescription)
      .accounts({ master: realmMaster.publicKey})
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Fetch second realm account
    realmAccount = await program.account.realm.fetch(secondRealmPDA);

    // Assertions for the second realm
    expect(realmAccount.name).to.equal(secondRealmName);
    expect(realmAccount.description).to.equal(secondRealmDescription);
    expect(realmAccount.createdAt).to.be.not.null;

    // Verify the second realm event is emitted
    expect(events.length).to.equal(2);
    expect(events[1].eventType.realmCreated.name).to.equal(secondRealmName);
    expect(events[1].eventType.realmCreated.description).to.equal(secondRealmDescription);
    expect(events[1].realmPubkey.toBase58()).to.equal(secondRealmPDA.toBase58());

    // Update first realm
    const updatedName = "Updated Realm";
    const updatedDescription = "An updated description";
    tx = await program.methods
      .updateRealm(firstRealmId, updatedName, updatedDescription)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    realmAccount = await program.account.realm.fetch(firstRealmPDA);
    expect(realmAccount.name).to.equal(updatedName);
    expect(realmAccount.description).to.equal(updatedDescription);

    // Verify the update event is emitted
    expect(events.length).to.equal(3);
    expect(events[2].eventType.realmUpdated.name).to.equal(updatedName);
    expect(events[2].eventType.realmUpdated.description).to.equal(updatedDescription);
    expect(events[2].realmPubkey.toBase58()).to.equal(firstRealmPDA.toBase58());

    // Verify second realm is still the same
    realmAccount = await program.account.realm.fetch(secondRealmPDA);
    expect(realmAccount.name).to.equal(secondRealmName);
    expect(realmAccount.description).to.equal(secondRealmDescription);

    // Delete the first realm
    tx = await program.methods
      .deleteRealm(firstRealmId)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Verify the first realm is deleted
    let realmInfo = await provider.connection.getAccountInfo(firstRealmPDA);
    expect(realmInfo).to.be.null; // Ensure the account no longer exists

    // Verify the delete event is emitted
    expect(events.length).to.equal(4);
    expect(events[3].eventType.deleted).not.to.be.null;
    expect(events[3].realmPubkey.toBase58()).to.equal(firstRealmPDA.toBase58());

    // Verify second realm is still the same
    realmInfo = await provider.connection.getAccountInfo(secondRealmPDA);
    expect(realmInfo).to.be.not.null;

    // Delete the second realm
    tx = await program.methods
      .deleteRealm(secondRealmId)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    // Verify the second realm is deleted
    realmInfo = await provider.connection.getAccountInfo(secondRealmPDA);
    expect(realmInfo).to.be.null; // Ensure the account no longer exists

    expect(events.length).to.equal(5);
    expect(events[4].eventType.deleted).not.to.be.null;
    expect(events[4].realmPubkey.toBase58()).to.equal(secondRealmPDA.toBase58());

    // Remove listener
    await program.removeEventListener(listener);
  });
});
