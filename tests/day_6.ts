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

  it("should succed test for loop!", async () => {
    const tx = await program.methods.forLoops().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test fixed array!", async () => {
    const tx = await program.methods.fixedArray().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test dynamic array!", async () => {
    const tx = await program.methods.dynamicArray().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test HashMap!", async () => {
    const tx = await program.methods.mappingTest().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test struct!", async () => {
    const tx = await program.methods.structTest().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test constant!", async () => {
    const tx = await program.methods.constantTest().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed test usize and casting!", async () => {
    const tx = await program.methods.usizeTest().rpc();
    console.log("You tx signature:", tx);
  })
})