import * as anchor from "@coral-xyz/anchor";

export async function airdrop(publicKey: anchor.web3.PublicKey, lamports: number) {
  let airdropTx = await anchor.getProvider().connection.requestAirdrop(publicKey, lamports);
  await confirmTransaction(airdropTx);
}

export async function confirmTransaction(tx: string) {
  const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
}

export function getRealmPDA(realmMaster: anchor.web3.PublicKey, realmSeed: string, program: anchor.Program<any>) {
  const [pda, _] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("realm"), realmMaster.toBuffer(), Buffer.from(realmSeed)],
    program.programId
  );
  return pda;
}