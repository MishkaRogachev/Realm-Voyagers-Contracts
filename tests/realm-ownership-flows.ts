import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagers } from "../target/types/realm_voyagers";
import { expect } from "chai";
import { airdrop, confirmTransaction, getRealmPDA } from "./helpers";

describe("Manage several realms", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;

  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();

  it("Alice creates a realm and transfers ownership to bob", async () => {
    await airdrop(alice.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Realm data
    const realmId = "realm_id_123";
    const realmName = "Test Realm";
    const realmDescription = "A test realm";
    const updatedRealmName = "Updated Realm Name";
    const updatedRealmDescription = "Updated Realm Description";

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
      expect(err.message).to.include("UnauthorizedRealmMaster");
    }

    // Fetch & check realm account after failed update
    realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.name).to.equal(updatedRealmName);
    expect(realmAccount.description).to.equal(updatedRealmDescription);
    realmAccount.masters.length = 1;
    realmAccount.masters[0].pubkey = alice.publicKey;
    realmAccount.masters[0].role = { owner: {} };

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
      expect(err.message).to.include("UnauthorizedRealmMaster");
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
    realmAccount.masters.length = 2;
    realmAccount.masters[0].pubkey = alice.publicKey;
    realmAccount.masters[0].role = { owner: {} };
    realmAccount.masters[1].pubkey = bob.publicKey;
    realmAccount.masters[1].role = { admin: {} };
  });
});