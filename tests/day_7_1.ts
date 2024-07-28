import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day71 } from '../target/types/day_7_1';

describe("day_7_1", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day7_1 as Program<Day71>;

  it("should succeed test option!", async () => {
    const tx = await program.methods.optionTest().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test ? operator!", async () => {
    const tx = await program.methods.encodeAndDecode().rpc();
    console.log("You tx signature:", tx);
  })
})