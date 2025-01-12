import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";

import { RealmVoyagers } from "../../target/types/realm_voyagers";
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
  let event = events[events.length - 1];
  expect(event.eventType.realmCreated.description).to.deep.equal(description);
  expect(event.realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function updateRealmDescription(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  updatedDescription: any,
  events: any[]
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
  let event = events[events.length - 1];
  expect(event.eventType.realmDescriptionUpdated.description).to.deep.equal(updatedDescription);
  expect(event.realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function deleteRealm(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  dimensions: string[],
  events: any[]
) {
  let remainingAccounts = [];
  for (let dimensionId of dimensions) {
    remainingAccounts.push({ pubkey: helper.getDimensionPDA(realmId, dimensionId, program), isSigner: false, isWritable: true });
  }

  let tx = await program.methods
    .deleteRealm(realmId)
    .accounts({ master: realmMaster.publicKey })
    .remainingAccounts(remainingAccounts)
    .signers([realmMaster])
    .rpc();
  await helper.confirmTransaction(tx);

  const realmPDA = helper.getRealmPDA(realmId, program);
  let realmInfo = await anchor.getProvider().connection.getAccountInfo(realmPDA);
  expect(realmInfo).to.be.null;

  expect(events.length).to.be.above(0);
  let event = events[events.length - 1];
  expect(event.eventType.deleted).not.to.be.null;
  expect(event.realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function addRealmMaster(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  newMaster: anchor.web3.PublicKey,
  events: any[]
) {
  let tx = await program.methods
    .addRealmMaster(realmId, newMaster)
    .accounts({ master: realmMaster.publicKey })
    .signers([realmMaster])
    .rpc();
  await helper.confirmTransaction(tx);

  const realmPDA = helper.getRealmPDA(realmId, program);
  const realmAccount = await program.account.realm.fetch(realmPDA);
  expect(realmAccount.masters.length).to.be.above(1);
  expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(realmMaster.publicKey.toBase58());
  expect(realmAccount.masters[0].role).to.deep.equal({ owner: {} });
  expect(realmAccount.masters[realmAccount.masters.length - 1].pubkey.toBase58()).to.equal(newMaster.toBase58());
  expect(realmAccount.masters[realmAccount.masters.length - 1].role).to.deep.equal({ admin: {} });

  expect(events.length).to.be.above(0);
  let event = events[events.length - 1];
  expect(event.eventType.realmMasterAdded).not.to.be.null;
  expect(event.eventType.realmMasterAdded.master.pubkey.toBase58()).to.equal(newMaster.toBase58());
  expect(event.eventType.realmMasterAdded.master.role).to.deep.equal({ admin: {} });
  expect(event.realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function removeRealmMaster(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  masterToRemove: anchor.web3.PublicKey,
  events: any[]
) {
  let tx = await program.methods
    .removeRealmMaster(realmId, masterToRemove)
    .accounts({ master: realmMaster.publicKey })
    .signers([realmMaster])
    .rpc();
  await helper.confirmTransaction(tx);

  const realmPDA = helper.getRealmPDA(realmId, program);
  const realmAccount = await program.account.realm.fetch(realmPDA);
  expect(realmAccount.masters.length).to.be.above(0);
  expect(realmAccount.masters.filter((master) => master.pubkey.toBase58() === masterToRemove.toBase58()).length).to.equal(0);

  expect(events.length).to.be.above(0);
  let event = events[events.length - 1];
  expect(event.eventType.realmMasterRemoved).not.to.be.null;
  expect(event.eventType.realmMasterRemoved.master.pubkey.toBase58()).to.equal(masterToRemove.toBase58());
  expect(event.realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function transferRealmOwnership(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  newOwner: anchor.web3.PublicKey,
  events: any[]
) {
  let tx = await program.methods
    .transferRealmOwnership(realmId, newOwner)
    .accounts({ master: realmMaster.publicKey })
    .signers([realmMaster])
    .rpc();
  await helper.confirmTransaction(tx);

  const realmPDA = helper.getRealmPDA(realmId, program);
  const realmAccount = await program.account.realm.fetch(realmPDA);
  expect(realmAccount.masters.length).to.be.above(0);
  expect(realmAccount.masters[0].pubkey.toBase58()).to.equal(realmMaster.publicKey.toBase58());
  expect(realmAccount.masters[0].role).to.deep.equal({ admin: {} });

  let newOwnerFound = false;
  for (let master of realmAccount.masters) {
    if (master.pubkey.toBase58() === newOwner.toBase58()) {
      expect(master.role).to.deep.equal({ owner: {} });
      newOwnerFound = true;
    }
  }
  expect(newOwnerFound).to.be.true;

  expect(events.length).to.be.above(0);
  let event = events[events.length - 1];
  expect(event.eventType.realmOwnershipTransferred).not.to.be.null;
  expect(event.eventType.realmOwnershipTransferred.newOwner.pubkey.toBase58()).to.equal(newOwner.toBase58());
  expect(event.eventType.realmOwnershipTransferred.newOwner.role).to.deep.equal({ owner: {} });
  expect(event.eventType.realmOwnershipTransferred.oldOwner.pubkey.toBase58()).to.equal(realmMaster.publicKey.toBase58());
  expect(event.eventType.realmOwnershipTransferred.oldOwner.role).to.deep.equal({ admin: {} });
  expect(event.realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function addRealmDimension(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  dimension: any,
  events: any[]
) {
  const dimensionPDA = helper.getDimensionPDA(realmId, dimension.id, program);
  const realmPDA = helper.getRealmPDA(realmId, program);

  const tx = await program.methods
    .addRealmDimension(realmId, dimension.id, dimension.name, dimension.areas)
    .accounts({
      master: realmMaster.publicKey,
    })
    .signers([realmMaster])
    .rpc();
  await helper.confirmTransaction(tx);

  var dimensionAccount = await program.account.realmDimension.fetch(dimensionPDA);
  expect(dimensionAccount.name).to.equal(dimension.name);
  expect(dimensionAccount.realm.toBase58()).to.equal(realmPDA.toBase58());
  expect(dimensionAccount.areas).to.deep.equal(dimension.areas);

  expect(events.length).to.be.above(0);
  let event = events[events.length - 1];
  expect(event.eventType.dimensionAdded.name).to.equal(dimension.name);
  expect(event.dimensionPubkey.toBase58()).to.equal(dimensionPDA.toBase58());
  expect(event.realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function updateRealmDimension(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  dimension: any,
  events: any[]
) {
  const dimensionPDA = helper.getDimensionPDA(realmId, dimension.id, program);
  const realmPDA = helper.getRealmPDA(realmId, program);

  const tx = await program.methods
    .updateRealmDimension(realmId, dimension.id, dimension.name, dimension.areas)
    .accounts({
      master: realmMaster.publicKey,
    })
    .signers([realmMaster])
    .rpc();
  await helper.confirmTransaction(tx);

  var dimensionAccount = await program.account.realmDimension.fetch(dimensionPDA);
  expect(dimensionAccount.name).to.equal(dimension.name);
  expect(dimensionAccount.realm.toBase58()).to.equal(realmPDA.toBase58());
  expect(dimensionAccount.areas).to.deep.equal(dimension.areas);

  expect(events.length).to.be.above(0);
  let event = events[events.length - 1];
  expect(event.eventType.dimensionUpdated.name).to.equal(dimension.name);
  expect(event.dimensionPubkey.toBase58()).to.equal(dimensionPDA.toBase58());
  expect(event.realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function removeRealmDimension(
  realmMaster: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  realmId: string,
  dimensionId: string,
  events: any[]
) {
  const dimensionPDA = helper.getDimensionPDA(realmId, dimensionId, program);
  const realmPDA = helper.getRealmPDA(realmId, program);

  const tx = await program.methods
    .removeRealmDimension(realmId, dimensionId)
    .accounts({
      master: realmMaster.publicKey,
    })
    .signers([realmMaster])
    .rpc();
  await helper.confirmTransaction(tx);

  let dimensionInfo = await anchor.getProvider().connection.getAccountInfo(dimensionPDA);
  expect(dimensionInfo).to.be.null;

  expect(events.length).to.be.above(0);
  let event = events[events.length - 1];
  expect(event.eventType.dimensionRemoved).not.to.be.null;
  expect(event.dimensionPubkey.toBase58()).to.equal(dimensionPDA.toBase58());
  expect(event.realmPubkey.toBase58()).to.equal(realmPDA.toBase58());
}

export async function createHero(
  master: anchor.web3.Keypair,
  program: anchor.Program<RealmVoyagers>,
  heroId: string,
  name: string,
  graphics: string,
  lore: string,
  events: any[]
) {
  const heroPDA = helper.getHeroPDA(master.publicKey, heroId, program);

  let tx = await program.methods
    .createHero(heroId, name, graphics, lore)
    .accounts({ master: master.publicKey })
    .signers([master])
    .rpc();
  await helper.confirmTransaction(tx);

  const heroAccount = await program.account.hero.fetch(heroPDA);
  expect(heroAccount.name).to.deep.equal(name);
  expect(heroAccount.graphics).to.deep.equal(graphics);
  expect(heroAccount.lore).to.deep.equal(lore);
  expect(heroAccount.createdAt).to.be.not.null
  expect(heroAccount.updatedAt.eq(heroAccount.createdAt)).to.be.true;

  expect(events.length).to.be.above(0);
  let event = events[events.length - 1];
  expect(event.eventType.heroCreated.heroPubkey.toBase58()).to.deep.equal(heroPDA.toBase58());
}