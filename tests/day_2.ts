import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day_2";

describe("day_2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize2(
      new anchor.BN(777),
      new anchor.BN(888),
      "hello world"
    ).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Initializes with vector", async () => {
    const tx = await program.methods.array([new anchor.BN(777),
    new anchor.BN(888)]).rpc();

    console.log("Your transaction signature", tx);
  });

  // when overflow-checks = false
  it.only("should not overflow on 100 - 200", async () => {
    const tx = await program.methods.overflowUnsafe(
      new anchor.BN(100),
      new anchor.BN(200)).rpc();

    // output:
    // Program log: Instruction: OverflowUnsafe
    // Program log: x_unsafe: 18446744073709551516
  });

  // when overflow-checks = true
  it.only("should overflow on 100 - 200", async () => {
    const tx = await program.methods.overflowUnsafe(
      new anchor.BN(100),
      new anchor.BN(200)).rpc();
  });
  // output:
  //   should overflow on 100 - 200:
  //   Error: Simulation failed.
  // Message: Transaction simulation failed: Error processing Instruction 0: Program failed to complete.

  it.only("should panic when overflow", async () => {
    const tx = await program.methods.overflowSafe(
      new anchor.BN(0),
      new anchor.BN(1)).rpc();

    // output:
    // Program log: Instruction: OverflowSafe
    // Program log: panicked at programs/day_2/src/lib.rs:30:44:
    // called `Option::unwrap()` on a `None` value
  });
});
