import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day12 } from '../target/types/day_12';

describe("day_12", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day12 as Program<Day12>;
  const stakeHistoryPubKey = new anchor.web3.PublicKey("SysvarStakeHistory1111111111111111111111111");

  it("should succeed to get epoch schedule!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("You tx signature:", tx);
  })

  it("should succeed to get rent info!", async () => {
    const tx = await program.methods.rentTest().rpc();
    console.log("You tx signature:", tx);
  })

  it.only("should succeed to get stake history info!", async () => {
    const tx = await program.methods.
      getStakeHistory().
      accounts({
        stakeHistory: stakeHistoryPubKey
      }).rpc();
    console.log("You tx signature:", tx);
  })
})