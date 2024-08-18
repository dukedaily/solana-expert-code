import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14 } from '../target/types/day_14';

describe("day_14", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day14 as Program<Day14>;

  it("should succeed to call single signer!", async () => {
    const tx = await program.methods.singleSigner().accounts(
      {
        signer1: program.provider.publicKey,
      }
    ).rpc();
    console.log('Signer1 raw: ', program.provider.publicKey);
    console.log('Signer1: ', program.provider.publicKey.toBase58());
  })


  it("should succeed to call multi signer!", async () => {
    let newKeypair = anchor.web3.Keypair.generate();
    console.log('New keypair: ', newKeypair.publicKey.toBase58());

    const tx = await program.methods.multipleSigner2().
      accounts({
        signer1: program.provider.publicKey,
        signer2: newKeypair.publicKey,
      }).
      signers([newKeypair]).rpc();
    console.log('Signer1: ', program.provider.publicKey.toBase58());
    console.log('Signer2: ', newKeypair.publicKey.toBase58());
  })

  it("should succeed to call by owner!", async () => {
    const tx = await program.methods.restrictToOwner().accounts({
      signerAccount: program.provider.publicKey,
    }).rpc();
  })

  it("should fail to call by non owner!", async () => {
    let newKeypair = anchor.web3.Keypair.generate();
    console.log('New keypair: ', newKeypair.publicKey.toBase58());
    const tx = await program.methods.restrictToOwner().accounts({
      signerAccount: newKeypair.publicKey,
    }).signers([newKeypair]).rpc();
  })
})