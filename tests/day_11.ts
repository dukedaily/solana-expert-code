import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day11 } from '../target/types/day_11';

describe("day_11", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day11 as Program<Day11>;

  it("should succeed to get timestamp!", async () => {
    const tx = await program.methods.getTimestamp().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed to get day of week!", async () => {
    const tx = await program.methods.getDayOfWeek().rpc();
    console.log("You tx signature:", tx);
  })
})