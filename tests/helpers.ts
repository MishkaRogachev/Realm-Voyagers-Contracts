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

export function getRealmPDA(realmId: string, program: anchor.Program<any>): anchor.web3.PublicKey{
  const [pda, _] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("realm"), Buffer.from(realmId)],
    program.programId
  );
  return pda;
}

export function getLocationPDA(realmId: string, locationId: string, program: anchor.Program<any>) {
  const [pda, _] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("location"), Buffer.from(realmId), Buffer.from(locationId)],
    program.programId
  );
  return pda;
}