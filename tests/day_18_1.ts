import { PublicKey } from '@solana/web3.js';
import { Program, AnchorProvider, Idl, web3 } from '@coral-xyz/anchor';
import fs from 'fs';

const idlFile = fs.readFileSync('../target/idl/day_18.json', 'utf8');
const idl: Idl = JSON.parse(idlFile);

// Set environment variables
process.env.ANCHOR_PROVIDER_URL = 'http://127.0.0.1:8899';
process.env.ANCHOR_WALLET = '/Users/duke.du/.config/solana/id.json';

// Create a provider
const provider = AnchorProvider.env();

// Create the program interface
const program = new Program(idl, provider);

async function getXValueFromStorageAccount(storageAccountPublicKey: PublicKey) {
  try {
    const account = await program.account['myStorage'].fetch(storageAccountPublicKey);
    const xValue = account.x;
    console.log('X value:', xValue.toNumber());
    return xValue;
  } catch (error) {
    console.error('Error fetching storage account data:', error);
    throw error;
  }
}

async function getXValueFromAccount(storageAccountPublicKey: PublicKey) {
  try {
    const info = await provider.connection.getAccountInfo(storageAccountPublicKey);
    console.log('Account info:', info);
  } catch (error) {
    console.error('Error fetching storage account data:', error);
    throw error;
  }
}


// Derive the PDA for myStorage
const seeds = [];
const [myStorage, bump] = web3.PublicKey.findProgramAddressSync(seeds, program.programId);

console.log('myStorage account raw:', myStorage);
console.log('myStorage account:', myStorage.toBase58());

async function main() {
  await getXValueFromStorageAccount(myStorage);
  await getXValueFromAccount(myStorage);
}

main().catch(console.error);