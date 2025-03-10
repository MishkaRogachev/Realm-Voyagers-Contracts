import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

import { RealmVoyagers } from "../target/types/realm_voyagers";

import * as helper from "./utils/helpers";
import * as steps from "./utils/common-steps";

describe("Hero Management", () => {
  // Program & keypairs
  const program = anchor.workspace.RealmVoyagers as Program<RealmVoyagers>;
  const player = anchor.web3.Keypair.generate();

  // Heroes data
  const heroId = "hero_id_1";
  const heroDescription = { name: "Test Hero 1", graphics: "https://example.com/graphics1", lore: "A test hero" };
  const heroStats = { strength: 10, dexterity: 5, intelligence: 3, charisma: 2, vitality: 8 };
  const updatedHeroDescription = { name: "Updated Hero 1", graphics: "https://example.com/graphics1", lore: "An updated lore" };

  // Listen events
  let listener = null;
  let events = [];

  before(async () => {
    listener = program.addEventListener("heroEvent", (event) => {
      events.push(event);
    });
  });

  after(async () => {
    await program.removeEventListener(listener);
  });

  it("Airdrop to player", async () => await helper.airdrop(player.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL));

  it("Create the hero", async () => await steps.createHero(player, program, heroId, heroDescription, events));
  it("Update the hero", async () => await steps.updateHeroDescription(player, program, heroId, updatedHeroDescription, events));

});