import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealmVoyagersContracts } from "../target/types/realm_voyagers_contracts";

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

  it("Create realm", async () => {
    await airdrop(provider, realmMaster.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    const tx = await program.methods
      .createRealm("Test Realm")
      .accounts({ master: realmMaster.publicKey })
      .signers([realmMaster])
      .rpc();
    console.log("Your transaction signature", tx);

    // Check balance after realm creation
    let balance = await provider.connection.getBalance(realmMaster.publicKey);
    console.log(`Hub crated, balance: ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
  });
});
