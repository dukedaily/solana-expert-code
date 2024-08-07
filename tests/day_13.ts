import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day13 } from '../target/types/day_13';
import { Connection, PublicKey } from '@solana/web3.js';

describe("day_13", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day13 as Program<Day13>;

  it("should succeed to listen for events!", async () => {
    const listner = program.addEventListener("myEvent",
      (event, slot) => {
        console.log(`slot ${slot}, event value: ${event.value}`)
      }
    )

    const listener2 = program.addEventListener('mySecondEvent',
      (event, slot) => {
        console.log(`slot ${slot}, event value: ${event.value}, message: ${event.message}`)
      }
    )

    await program.methods.initialize().rpc();
    await new Promise((resolve) => setTimeout(resolve, 5000));

    program.removeEventListener(listner);
    program.removeEventListener(listener2);
  })

  it.only("should succeed to list all the txs!", async () => {
    const connection = new Connection('https://api.mainnet-beta.solana.com');
    const getTxs = async (address, limit) => {
      const pubKey = new PublicKey(address);
      let txList = await connection.getSignaturesForAddress(pubKey, {
        limit
      });
      let signatureList = txList.map((tx) => tx.signature);
      console.log('signatureList: ', signatureList.length);

      for await (const sig of signatureList) {
        console.log(await connection.getParsedTransaction(sig, {
          maxSupportedTransactionVersion: 0
        }));
      }
    }

    let myAddress = '8Qmw55X5cUZdwupBFQrN3ZBNSQ6tbwLVgu8yjD9Df2X5'
    getTxs(myAddress, 10);
  })
})