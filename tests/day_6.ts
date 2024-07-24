import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day6 } from '../target/types/day_6';

describe("day_6", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day6 as Program<Day6>;

  it("should succeed age checker!", async () => {
    const tx = await program.methods.ageChecker(
      new anchor.BN(36)
    ).rpc();
    console.log("You tx signature:", tx);
  })
})