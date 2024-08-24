import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day17 } from '../target/types/day_17';

describe("day_17", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day17 as Program<Day17>;

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
    console.log('tx: ', tx);
  })

  it.only("should get value", async () => {
    const seeds = []
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
    await program.methods.get().accounts({
      myStorageGet: myStorage
    }).rpc();
  })
});