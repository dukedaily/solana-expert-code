import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day4 } from "../target/types/day_4";
import { assert } from "chai";

describe("day_4", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day4 as Program<Day4>;

  it("Input test too small", async () => {
    try {
      const tx = await program.methods.limitRange(new anchor.BN(9)).rpc();
      console.log("Your transaction signature", tx);
    } catch (_err) {
      // console.log(_err);
      assert.isTrue(_err instanceof anchor.AnchorError);

      const err: anchor.AnchorError = _err;
      const errMsg = "a is too small";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("error number:", err.error.errorCode.number);
    }
  })

  it("Input test too big", async () => {
    try {
      const tx = await program.methods.limitRange(new anchor.BN(101)).rpc();
      console.log("Your transaction signature", tx);
    } catch (_err) {
      assert.isTrue(_err instanceof anchor.AnchorError);
      const err: anchor.AnchorError = _err;
      const errMsg =
        "a is too big";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }
  })

  it("Input test require, too small", async () => {
    try {
      const tx = await program.methods.limitRangeRequire(new anchor.BN(9)).rpc();
      console.log("Your transaction signature", tx);
    } catch (_err) {
      // console.log(_err);
      assert.isTrue(_err instanceof anchor.AnchorError);

      const err: anchor.AnchorError = _err;
      const errMsg = "a is too small";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("error number:", err.error.errorCode.number);
    }
  })

  it("Input test require, too big", async () => {
    try {
      const tx = await program.methods.limitRangeRequire(new anchor.BN(101)).rpc();
      console.log("Your transaction signature", tx);
    } catch (_err) {
      assert.isTrue(_err instanceof anchor.AnchorError);
      const err: anchor.AnchorError = _err;
      const errMsg =
        "a is too big";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }
  })

  it("Error test funcError", async () => {
    try {
      const tx = await program.methods.funcError().rpc();
      console.log("Your transaction signature", tx);
    } catch (_err) {
      assert.isTrue(_err instanceof anchor.AnchorError);
      const err: anchor.AnchorError = _err;
      const errMsg =
        "Always errors";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }
  });

  it("Error test funcOK", async () => {
    try {
      const tx = await program.methods.funcOk().rpc();
      console.log("Your transaction signature", tx);
    } catch (_err) {
      assert.isTrue(_err instanceof anchor.AnchorError);
      const err: anchor.AnchorError = _err;
      const errMsg =
        "Always errors";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }
  });
});
