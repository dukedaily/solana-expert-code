import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import fs from 'fs'
let idl = JSON.parse(fs.readFileSync('target/idl/day_5.json', 'utf-8'))

describe("day_5", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  // const programID = "HNdiq7jWsmQ33GPmRd95NR3Momr8pcUVx3GB7MuJMhU";
  const program = new Program(idl, anchor.getProvider());
  it.only("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});