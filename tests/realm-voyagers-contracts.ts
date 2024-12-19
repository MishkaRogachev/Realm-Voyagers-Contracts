import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagersContracts } from "../target/types/realm_voyagers_contracts";
import { expect } from "chai";

async function airdrop(provider: anchor.AnchorProvider, publicKey: anchor.web3.PublicKey, lamports: number) {
  const airdropSignature = await provider.connection.requestAirdrop(publicKey, lamports);
  const latestBlockHash = await provider.connection.getLatestBlockhash();

  await provider.connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: airdropSignature,
  });

  console.log(`Airdropped ${lamports / anchor.web3.LAMPORTS_PER_SOL} SOL to: ${publicKey.toBase58()}`);
}

describe("realm-voyagers-contracts", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RealmVoyagersContracts as Program<RealmVoyagersContracts>;

  const realmMaster = anchor.web3.Keypair.generate();

  it("Create realm, add some locations and delete", async () => {
    await airdrop(provider, realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Create realm
    const realmName = "Test Realm";
    const [realmPDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("realm"), realmMaster.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .createRealm(realmName)
      .accounts({
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();
    console.log("Transaction signature:", tx);

    // Fetch realm account
    const realmAccount = await program.account.realm.fetch(realmPDA);
    console.log("Realm account:", realmAccount);

    // Assertions
    expect(realmAccount.master.toBase58()).to.equal(realmMaster.publicKey.toBase58());
    expect(realmAccount.name).to.equal(realmName);
    console.log(`Realm created: ${realmAccount.name}`);

    // Add Locations
    const locations = [
      { name: "Rat Castle", tilemapUrl: "https://example.com/castle-map", tilesetUrl: "https://example.com/castle-tileset" },
      { name: "Synth Dungeon", tilemapUrl: "https://example.com/dungeon-map", tilesetUrl: "https://example.com/dungeon-tileset" },
      { name: "Witchcraft Spaceship", tilemapUrl: "https://example.com/spaceship-map", tilesetUrl: "https://example.com/spaceship-tileset" },
    ];

    for (const location of locations) {
      const [locationPDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("location"), realmPDA.toBuffer(), Buffer.from(location.name)],
        program.programId
      );

      const locationTx = await program.methods
      .addRealmLocation(location.name, location.tilemapUrl, location.tilesetUrl)
      .accounts({
        realm: realmPDA,
        master: realmMaster.publicKey,
      })
      .signers([realmMaster])
      .rpc();

      console.log(`Location "${location.name}" added, transaction:`, locationTx);

      // Fetch and verify the location account
      const locationAccount = await program.account.location.fetch(locationPDA);
      expect(locationAccount.realm.toBase58()).to.equal(realmPDA.toBase58());
      expect(locationAccount.name).to.equal(location.name);
      expect(locationAccount.tilemapUrl).to.equal(location.tilemapUrl);
      expect(locationAccount.tilesetUrl).to.equal(location.tilesetUrl);

      // Delete the Realm
      const deleteTx = await program.methods
        .deleteRealm()
        .accounts({
          realm: realmPDA,
          master: realmMaster.publicKey,
        })
        .signers([realmMaster])
        .rpc();

        console.log("Realm and all locations deleted, transaction signature:", deleteTx);

        // TODO: Verify Realm is deleted
    }
  });
});
