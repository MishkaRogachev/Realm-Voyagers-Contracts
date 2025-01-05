import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagers } from "../target/types/realm_voyagers";
import { expect } from "chai";
import { airdrop, confirmTransaction, getRealmPDA } from "./helpers";

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

  // Realm PDA
  const realmPDA = getRealmPDA(realmId, program);

  it("Alice creates a realm", async () => {
    await airdrop(alice.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    let tx = await program.methods
      .createRealm(realmId, realmDescription)
      .accounts({
          master: alice.publicKey,
      })
      .signers([alice])
      .rpc();
    await confirmTransaction(tx);
  });

  it("Bob tries to update the realm, and it fails", async () => {
    await airdrop(bob.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    try {
      let tx = await program.methods
        .updateRealmDescription(realmId, updatedRealmDescription)
        .accounts({ master: bob.publicKey })
        .signers([bob])
        .rpc();
      await confirmTransaction(tx);
      expect.fail("Bob should not be authorized to update the realm");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }
  });

  it("Bob tries to add himself as a realm master, and it fails", async () => {
    try {
      let tx = await program.methods
        .addRealmMaster(realmId, bob.publicKey)
        .accounts({ master: bob.publicKey })
        .signers([bob])
        .rpc();
      await confirmTransaction(tx);
      expect.fail("Bob should not be authorized to add himself as realm master");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }
  });

  it("Alice adds Bob as a realm master", async () => {
    let tx = await program.methods
      .addRealmMaster(realmId, bob.publicKey)
      .accounts({ master: alice.publicKey })
      .signers([alice])
      .rpc();
    await confirmTransaction(tx);

    const realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.masters.length).to.equal(2);
    expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(alice.publicKey.toBase58());
    expect(realmAccount.masters[0].role).to.deep.equal({ owner: {} });
    expect(realmAccount.masters[1].pubkey.toBase58()).to.equal(bob.publicKey.toBase58());
    expect(realmAccount.masters[1].role).to.deep.equal({ admin: {} });
  });

  it("Bob tries to update the realm successfully", async () => {
    let tx = await program.methods
      .updateRealmDescription(realmId, updatedRealmDescription)
      .accounts({ master: bob.publicKey })
      .signers([bob])
      .rpc();
    await confirmTransaction(tx);

    const realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.description).to.deep.equal(updatedRealmDescription);
  });

  it("Bob tries to transfer ownership to himself, and it fails", async () => {
    try {
      let tx = await program.methods
        .transferRealmOwnership(realmId, bob.publicKey)
        .accounts({ master: bob.publicKey })
        .signers([bob])
        .rpc();
      await confirmTransaction(tx);
      expect.fail("Bob should not be authorized to transfer realm ownership");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }

    const realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.masters.length).to.equal(2);
    expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(alice.publicKey.toBase58());
    expect(realmAccount.masters[0].role).to.deep.equal({ owner: {} });
    expect(realmAccount.masters[1].pubkey.toBase58()).to.equal(bob.publicKey.toBase58());
    expect(realmAccount.masters[1].role).to.deep.equal({ admin: {} });
  });

  it("Alice transfers ownership to Bob", async () => {
    let tx = await program.methods
      .transferRealmOwnership(realmId, bob.publicKey)
      .accounts({ master: alice.publicKey })
      .signers([alice])
      .rpc();
    await confirmTransaction(tx);

    const realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.masters.length).to.equal(2);
    expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(alice.publicKey.toBase58());
    expect(realmAccount.masters[0].role).to.deep.equal({ admin: {} });
    expect(realmAccount.masters[1].pubkey.toBase58()).to.equal(bob.publicKey.toBase58());
    expect(realmAccount.masters[1].role).to.deep.equal({ owner: {} });
  });

  it("Bob removes Alice as a realm master", async () => {
    let tx = await program.methods
      .removeRealmMaster(realmId, alice.publicKey)
      .accounts({ master: bob.publicKey })
      .signers([bob])
      .rpc();
    await confirmTransaction(tx);

    const realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.masters.length).to.equal(1);
    expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(bob.publicKey.toBase58());
    expect(realmAccount.masters[0].role).to.deep.equal({ owner: {} });
  });

  it("Alice tries to update the realm, and it fails", async () => {
    try {
      let tx = await program.methods
        .updateRealmDescription(realmId, updatedRealmDescription)
        .accounts({ master: alice.publicKey })
        .signers([alice])
        .rpc();
      await confirmTransaction(tx);
      expect.fail("Alice should not be authorized to update the realm");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("UnauthorizedRealmMaster");
    }
  });

  it("Bob deletes the realm", async () => {
    let tx = await program.methods
      .deleteRealm(realmId)
      .accounts({ master: bob.publicKey })
      .signers([bob])
      .rpc();
    await confirmTransaction(tx);

    try {
      const _ = await program.account.realm.fetch(realmPDA);
      expect.fail("Realm account should have been deleted");
    } catch (err) {
      expect(err.message).to.include("Account does not exist");
    }
  });
});
