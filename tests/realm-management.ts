import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagers } from "../target/types/realm_voyagers";
import { expect } from "chai";

async function airdrop(publicKey: anchor.web3.PublicKey, lamports: number) {
  let airdropTx = await anchor.getProvider().connection.requestAirdrop(publicKey, lamports);
  await confirmTransaction(airdropTx);
}

async function confirmTransaction(tx: string) {
  const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
}

describe("Test realm management operations ", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;

  const realmMaster = anchor.web3.Keypair.generate();

  it("Create realm, add some locations and delete", async () => {
    await airdrop(realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Realm data & PDA prediction
    const realmSeed = "seed";
    const realmName = "Test Realm";
    const realmDescription = "A test realm for the unit tests";
    const [realmPDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("realm"), realmMaster.publicKey.toBuffer(), Buffer.from(realmSeed)],
      program.programId
    );

    // Add log listener for events
    let createEvents = [];
    program.addEventListener("realmCreated", (event) => {
      createEvents.push(event);
    });

    let updateEvents = [];
    program.addEventListener("realmUpdated", (event) => {
      updateEvents.push(event);
    });

    let deleteEvents = [];
    program.addEventListener("realmDeleted", (event) => {
      deleteEvents.push(event);
    });

    // Create realm
    let tx = await program.methods
      .createRealm(realmSeed, realmName, realmDescription)
      .accounts({
        realmMaster: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();

    await confirmTransaction(tx);

    // Fetch realm account
    var realmAccount = await program.account.realm.fetch(realmPDA);

    // Assertions
    expect(realmAccount.realmMaster.toBase58()).to.equal(realmMaster.publicKey.toBase58());
    expect(realmAccount.name).to.equal(realmName);
    expect(realmAccount.description).to.equal(realmDescription);
    expect(realmAccount.createdAt).to.be.not.null;

    // Verify the event is emitted
    expect(createEvents.length).to.equal(1);
    expect(createEvents[0].name).to.equal(realmName);
    expect(createEvents[0].description).to.equal(realmDescription);

    // Update realm
    const updatedName = "Updated Realm";
    const updatedDescription = "An updated description";
    tx = await program.methods
      .updateRealm(updatedName, updatedDescription)
      .accounts({
        realm: realmPDA,
        realmMaster: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();

    await confirmTransaction(tx);

    realmAccount = await program.account.realm.fetch(realmPDA);
    expect(realmAccount.name).to.equal(updatedName);
    expect(realmAccount.description).to.equal(updatedDescription);

    // Verify the event is emitted
    expect(updateEvents.length).to.equal(1);
    expect(updateEvents[0].name).to.equal(updatedName);
    expect(updateEvents[0].description).to.equal(updatedDescription);


    // // Add Locations
    // const locations = [
    //   { name: "Rat Castle", tilemapUrl: "https://example.com/castle-map", tilesetUrl: "https://example.com/castle-tileset" },
    //   { name: "Synth Dungeon", tilemapUrl: "https://example.com/dungeon-map", tilesetUrl: "https://example.com/dungeon-tileset" },
    //   { name: "Witchcraft Spaceship", tilemapUrl: "https://example.com/spaceship-map", tilesetUrl: "https://example.com/spaceship-tileset" },
    // ];

    // for (const location of locations) {
    //   const [locationPDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
    //     [Buffer.from("location"), realmPDA.toBuffer(), Buffer.from(location.name)],
    //     program.programId
    //   );

    //   const locationTx = await program.methods
    //   .addRealmLocation(location.name, location.tilemapUrl, location.tilesetUrl)
    //   .accounts({
    //     realm: realmPDA,
    //     master: realmMaster.publicKey,
    //   })
    //   .signers([realmMaster])
    //   .rpc();

    //   console.log(`Location "${location.name}" added, transaction:`, locationTx);

    //   // Fetch and verify the location account
    //   const locationAccount = await program.account.location.fetch(locationPDA);
    //   expect(locationAccount.realm.toBase58()).to.equal(realmPDA.toBase58());
    //   expect(locationAccount.name).to.equal(location.name);
    //   expect(locationAccount.tilemapUrl).to.equal(location.tilemapUrl);
    //   expect(locationAccount.tilesetUrl).to.equal(location.tilesetUrl);

    // Delete the Realm
    tx = await program.methods
      .deleteRealm()
      .accounts({
        realm: realmPDA,
        realmMaster: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();

    await confirmTransaction(tx);

    // Verify the Realm is deleted
    const realmInfo = await provider.connection.getAccountInfo(realmPDA);
    expect(realmInfo).to.be.null; // Ensure the account no longer exists

    // Verify the event is emitted
    expect(deleteEvents.length).to.equal(1);
    expect(deleteEvents[0].realmPubkey.toBase58()).to.equal(realmPDA.toBase58());

  });
});
