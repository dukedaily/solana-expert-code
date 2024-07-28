import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day7 } from '../target/types/day_7';

describe("day_7", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day7 as Program<Day7>;

  it("should succeed copy types!", async () => {
    const tx = await program.methods.copyTypes().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test ownership!", async () => {
    const tx = await program.methods.ownershipTest().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test borrowing integer!", async () => {
    const tx = await program.methods.borrowInteger().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test clone test!", async () => {
    const tx = await program.methods.cloneTest().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test struct!", async () => {
    const tx = await program.methods.genericTypeTest().rpc();
    console.log("You tx signature:", tx);
  })

  it.only("should succeed test option!", async () => {
    const tx = await program.methods.optionTest().rpc();
    console.log("You tx signature:", tx);
  })
})