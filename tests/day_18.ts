import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day18 } from '../target/types/day_18';

describe("day_18", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day18 as Program<Day18>;

  it("should initialized!", async () => {
    const seeds = []
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
    console.log('myStorage account raw: ', myStorage);
    console.log('myStorage account: ', myStorage.toBase58());

    await program.methods.initialize().accounts({
      myStorageAccount: myStorage
    }).rpc();

    const value = new anchor.BN(200)
    const tx = await program.methods.set(value).accounts({
      myStorageSet: myStorage
    }).rpc();

    const v = await program.account.myStorage.fetch(myStorage);
    console.log('fetch value: ', v.x.toNumber());
  })
});