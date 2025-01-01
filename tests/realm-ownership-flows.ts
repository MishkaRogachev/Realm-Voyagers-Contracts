import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagers } from "../target/types/realm_voyagers";
import { expect } from "chai";
import { airdrop, confirmTransaction, getRealmPDA } from "./helpers";

describe("Realm ownership flows", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;
  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();

  it("Alice creates a realm and transfers ownership to bob", async () => {
    await airdrop(alice.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
    await airdrop(bob.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Realm data
    const realmId = "realm_id_123";
    const realmName = "Test Realm";
    const realmDescription = "A test realm";
    const updatedRealmName = "Updated Realm Name";
    const updatedRealmDescription = "Updated Realm Description";
    const updatedRealmName2 = "Updated Realm Name 2";
    const updatedRealmDescription2 = "Updated Realm Description 2";

    // Realm PDA
    const realmPDA = getRealmPDA(realmId, program);

    // Alice creates a realm
    let tx = await program.methods
      .createRealm(realmId, realmName, realmDescription)
      .accounts({
         master: alice.publicKey,
      })
      .signers([alice])
      .rpc();
    await confirmTransaction(tx);

    // Fetch & check realm account after creation
    var realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.name).to.equal(realmName);
    expect(realmAccount.description).to.equal(realmDescription);

    // Alice updates the realm successfully
    tx = await program.methods
    .updateRealm(realmId, updatedRealmName, updatedRealmDescription)
      .accounts({ master: alice.publicKey })
      .signers([alice])
      .rpc();
    await confirmTransaction(tx);

    // Fetch & check realm account after update
    realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.name).to.equal(updatedRealmName);
    expect(realmAccount.description).to.equal(updatedRealmDescription);

    // Bob tries to update the realm, and it fails
    try {
      tx = await program.methods
        .updateRealm(realmId, "Unauthorized Update", "Should Fail")
        .accounts({ master: bob.publicKey })
        .signers([bob])
        .rpc();
      await confirmTransaction(tx);
      expect.fail("Bob should not be authorized to update the realm");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }

    // Fetch & check realm account after failed update
    realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.name).to.equal(updatedRealmName);
    expect(realmAccount.description).to.equal(updatedRealmDescription);
    expect(realmAccount.masters.length).to.equal(1);
    expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(alice.publicKey.toBase58());
    expect(realmAccount.masters[0].role).to.deep.equal({ owner: {} });

    // Bob tries to add himself as a realm master, and it fails
    try {
      tx = await program.methods
        .addRealmMaster(realmId, bob.publicKey)
        .accounts({ master: bob.publicKey })
        .signers([bob])
        .rpc();
      await confirmTransaction(tx);
      expect.fail("Bob should not be authorized to add himself as realm master");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }

    // Alice adds Bob as a realm master
    tx = await program.methods
      .addRealmMaster(realmId, bob.publicKey)
      .accounts({ master: alice.publicKey })
      .signers([alice])
      .rpc();
    await confirmTransaction(tx);

    // Fetch & check realm account after adding Bob as a realm master
    realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.masters.length).to.equal(2);
    expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(alice.publicKey.toBase58());
    expect(realmAccount.masters[0].role).to.deep.equal({ owner: {} });
    expect(realmAccount.masters[1].pubkey.toBase58()).to.equal(bob.publicKey.toBase58());
    expect(realmAccount.masters[1].role).to.deep.equal({ admin: {} });

    // Bob tries to update the realm successfully
    tx = await program.methods
      .updateRealm(realmId, updatedRealmName2, updatedRealmDescription2)
      .accounts({ master: bob.publicKey })
      .signers([bob])
      .rpc();
    await confirmTransaction(tx);

    // Fetch & check realm account after Bob's update
    realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.name).to.equal(updatedRealmName2);
    expect(realmAccount.description).to.equal(updatedRealmDescription2);

    // Bob tries to transfer ownership to himself, and it fails
    try {
      tx = await program.methods
        .transferRealmOwnership(realmId, bob.publicKey)
        .accounts({ master: bob.publicKey })
        .signers([bob])
        .rpc();
      await confirmTransaction(tx);
      expect.fail("Bob should not be authorized to transfer realm ownership");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }

    // Fetch & check realm account after failed ownership transfer
    realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.masters.length).to.equal(2);
    expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(alice.publicKey.toBase58());
    expect(realmAccount.masters[0].role).to.deep.equal({ owner: {} });
    expect(realmAccount.masters[1].pubkey.toBase58()).to.equal(bob.publicKey.toBase58());
    expect(realmAccount.masters[1].role).to.deep.equal({ admin: {} });

    // Alice transfers ownership to Bob
    tx = await program.methods
      .transferRealmOwnership(realmId, bob.publicKey)
      .accounts({ master: alice.publicKey })
      .signers([alice])
      .rpc();
    await confirmTransaction(tx);

    // Fetch & check realm account after ownership transfer
    realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.masters.length).to.equal(2);
    expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(alice.publicKey.toBase58());
    expect(realmAccount.masters[0].role).to.deep.equal({ admin: {} });
    expect(realmAccount.masters[1].pubkey.toBase58()).to.equal(bob.publicKey.toBase58());
    expect(realmAccount.masters[1].role).to.deep.equal({ owner: {} });

    // Bob removes Alice as a realm master
    tx = await program.methods
      .removeRealmMaster(realmId, alice.publicKey)
      .accounts({ master: bob.publicKey })
      .signers([bob])
      .rpc();
    await confirmTransaction(tx);

    // Fetch & check realm account after removing Alice as a realm master
    realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.masters.length).to.equal(1);
    expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(bob.publicKey.toBase58());
    expect(realmAccount.masters[0].role).to.deep.equal({ owner: {} });

    // Alice tries to update the realm, and it fails
    try {
      tx = await program.methods
        .updateRealm(realmId, "Unauthorized Update", "Should Fail")
        .accounts({ master: alice.publicKey })
        .signers([alice])
        .rpc();
      await confirmTransaction(tx);
      expect.fail("Alice should not be authorized to update the realm");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }

    // Bob deletes the realm
    tx = await program.methods
      .deleteRealm(realmId)
      .accounts({ master: bob.publicKey })
      .signers([bob])
      .rpc();
    await confirmTransaction(tx);

    // Try to fetch realm account after deletion
    try {
      realmAccount = await program.account.realm.fetch(realmPDA);
      expect.fail("Realm account should have been deleted");
    } catch (err) {
      expect(err.message).to.include("Account does not exist");
    }
  });
});
