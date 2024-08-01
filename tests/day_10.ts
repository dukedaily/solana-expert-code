import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day10 } from '../target/types/day_10';

describe("day_10", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day10 as Program<Day10>;

  it("should succeed on internal functions!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("You tx signature:", tx);
  })
})