import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";

import { RealmVoyagers } from "../target/types/realm_voyagers";
import * as helper from "./helpers";

export async function createRealm(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  description: any,
  events: any[]
  ) {
  let tx = await program.methods
    .createRealm(realmId, description)
    .accounts({ master: realmMaster.publicKey })
    .signers([realmMaster])
    .rpc();
  await helper.confirmTransaction(tx);

  const realmPDA = helper.getRealmPDA(realmId, program);
  const realmAccount = await program.account.realm.fetch(realmPDA);

  expect(realmAccount.description).to.deep.equal(description);
  expect(realmAccount.createdAt).to.be.not.null
  expect(realmAccount.updatedAt.eq(realmAccount.createdAt)).to.be.true;

  expect(events.length).to.be.above(0);
  expect(events[events.length - 1].eventType.realmCreated.description).to.deep.equal(description);
  expect(events[events.length - 1].realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function updateRealmDescription(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  updatedDescription: any,
  events: any[] | undefined = undefined
) {
  let tx = await program.methods
    .updateRealmDescription(realmId, updatedDescription)
    .accounts({ master: realmMaster.publicKey })
    .signers([realmMaster])
    .rpc();
  await helper.confirmTransaction(tx);

  const realmPDA = helper.getRealmPDA(realmId, program);
  const realmAccount = await program.account.realm.fetch(realmPDA);

  expect(realmAccount.description).to.deep.equal(updatedDescription);
  expect(realmAccount.updatedAt).not.to.be.equal(realmAccount.createdAt);

  expect(events.length).to.be.above(0);
  expect(events[events.length - 1].eventType.realmDescriptionUpdated.description).to.deep.equal(updatedDescription);
  expect(events[events.length - 1].realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function deleteRealm(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  events: any[] | undefined = undefined
) {
  let tx = await program.methods
    .deleteRealm(realmId)
    .accounts({ master: realmMaster.publicKey })
    .signers([realmMaster])
    .rpc();
  await helper.confirmTransaction(tx);

  const realmPDA = helper.getRealmPDA(realmId, program);
  let realmInfo = await anchor.getProvider().connection.getAccountInfo(realmPDA);
  expect(realmInfo).to.be.null;

  expect(events.length).to.be.above(0);
  expect(events[events.length - 1].eventType.deleted).not.to.be.null;
  expect(events[events.length - 1].realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}