import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day15 } from '../target/types/day_15';

describe("day_15", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day15 as Program<Day15>;

  const localAddress = 'D99tQMx2iCHqUi2F9d4gvdATyFv8u1A6wr9KugW4JtK4'
  const defaultKeypair = new anchor.web3.PublicKey(localAddress)

  it("should get compoute units!", async () => {
    let bal1 = await program.provider.connection.getBalance(defaultKeypair)
    console.log('before: ', bal1);

    let tx = await program.methods.initialize().rpc();
    let bal2 = await program.provider.connection.getBalance(defaultKeypair)
    console.log('after: ', bal2);
    console.log('diff: ', bal1 - bal2);
  })

  it("should consume more compoute units!", async () => {
    let bal1 = await program.provider.connection.getBalance(defaultKeypair)
    console.log('before: ', bal1);

    let tx = await program.methods.moreCode().rpc();
    let bal2 = await program.provider.connection.getBalance(defaultKeypair)
    console.log('after: ', bal2);
    console.log('diff: ', bal1 - bal2);
  })

  it("should consume even more compoute units!", async () => {
    let tx = await program.methods.saveUnits().rpc();
  })
})