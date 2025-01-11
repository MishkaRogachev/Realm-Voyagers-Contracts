import * as anchor from "@coral-xyz/anchor";

export function getRealmPDA(realmId: string, program: anchor.Program<any>): anchor.web3.PublicKey{
  const [pda, _] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("realm"), Buffer.from(realmId)],
    program.programId
  );
  return pda;
}

export function getDimensionPDA(realmId: string, dimensionId: string, program: anchor.Program<any>) {
  const [pda, _] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("dimension"), Buffer.from(realmId), Buffer.from(dimensionId)],
    program.programId
  );
  return pda;
}

export function getJourneyPDA(realmId: string, player: anchor.web3.PublicKey, program: anchor.Program<any>) {
  const [pda, _] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("journey"), Buffer.from(realmId), player.toBuffer()],
    program.programId
  );
  return pda;
}

export function getHeroPDA(player: anchor.web3.PublicKey, heroId: string, program: anchor.Program<any>) {
  const [pda, _] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("hero"),  player.toBuffer(), Buffer.from(heroId)],
    program.programId
  );
  return pda;
}

//seeds = [HERO_SEED, player.key().as_ref(), hero_id.as_bytes()],

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
