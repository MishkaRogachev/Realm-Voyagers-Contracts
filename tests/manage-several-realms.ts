import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagers } from "../target/types/realm_voyagers";
import { expect } from "chai";
import { airdrop, confirmTransaction, getRealmPDA } from "./helpers";

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

  // Realm PDAs
  const firstRealmPDA = getRealmPDA(firstRealmId, program);
  const secondRealmPDA = getRealmPDA(secondRealmId, program);

  // Add listener for events
  let events = [];
  let listener = program.addEventListener("realmEvent", (event) => {
    events.push(event);
  });

  it("Create the first realm", async () => {
    await airdrop(realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    let tx = await program.methods
      .createRealm(firstRealmId, firstRealmDescription)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    const realmAccount = await program.account.realm.fetch(firstRealmPDA);

    expect(realmAccount.description).to.deep.equal(firstRealmDescription);
    expect(realmAccount.createdAt).to.be.not.null
    expect(realmAccount.updatedAt.eq(realmAccount.createdAt)).to.be.true;

    expect(events.length).to.equal(1);
    expect(events[0].eventType.realmCreated.description).to.deep.equal(firstRealmDescription);
    expect(events[0].realmPubkey.toBase58()).to.equal(firstRealmPDA.toBase58());
  });

  it("Create the second realm", async () => {
    let tx = await program.methods
      .createRealm(secondRealmId, secondRealmDescription)
      .accounts({ master: realmMaster.publicKey})
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    const realmAccount = await program.account.realm.fetch(secondRealmPDA);

    expect(realmAccount.description).to.deep.equal(secondRealmDescription);
    expect(realmAccount.createdAt).to.be.not.null;
    expect(realmAccount.updatedAt.eq(realmAccount.createdAt)).to.be.true;

    expect(events.length).to.equal(2);
    expect(events[1].eventType.realmCreated.description).to.deep.equal(secondRealmDescription);
    expect(events[1].realmPubkey.toBase58()).to.equal(secondRealmPDA.toBase58());
  });

  it("Update first realm", async () => {
    let tx = await program.methods
      .updateRealmDescription(firstRealmId, updatedDescription)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    const realmAccount = await program.account.realm.fetch(firstRealmPDA);
    expect(realmAccount.description).to.deep.equal(updatedDescription);
    expect(realmAccount.updatedAt).not.to.be.equal(realmAccount.createdAt);

    expect(events.length).to.equal(3);
    expect(events[2].eventType.realmDescriptionUpdated.description).to.deep.equal(updatedDescription);
    expect(events[2].realmPubkey.toBase58()).to.equal(firstRealmPDA.toBase58());
  });


  it("Delete the first realm", async () => {
    let tx = await program.methods
      .deleteRealm(firstRealmId)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    let realmInfo = await provider.connection.getAccountInfo(firstRealmPDA);
    expect(realmInfo).to.be.null;

    expect(events.length).to.equal(4);
    expect(events[3].eventType.deleted).not.to.be.null;
    expect(events[3].realmPubkey.toBase58()).to.equal(firstRealmPDA.toBase58());
  });

  it("Delete the second realm", async () => {
    let tx = await program.methods
      .deleteRealm(secondRealmId)
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    await confirmTransaction(tx);

    let realmInfo = await provider.connection.getAccountInfo(secondRealmPDA);
    expect(realmInfo).to.be.null;

    expect(events.length).to.equal(5);
    expect(events[4].eventType.deleted).not.to.be.null;
    expect(events[4].realmPubkey.toBase58()).to.equal(secondRealmPDA.toBase58());

    // Remove listener
    await program.removeEventListener(listener);
  });
});
