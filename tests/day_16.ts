import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day16 } from '../target/types/day_16';

describe("day_16", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day16 as Program<Day16>;

  it("should initialized!", async () => {
    const seeds = []
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
    console.log('myStorage account raw: ', myStorage);
    console.log('myStorage account: ', myStorage.toBase58());

    await program.methods.initialize().accounts({
      myStorageAccount: myStorage
    }).rpc();
  })

  // it("should fail two initialize twice!", async () => {
  //   const seeds = []
  //   const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

  //   await program.methods.initialize().accounts({
  //     myStorageAccount: myStorage
  //   }).rpc();

  // await program.methods.initialize().accounts({
  //   myStorageAccount: myStorage
  // }).rpc();
  // })
})