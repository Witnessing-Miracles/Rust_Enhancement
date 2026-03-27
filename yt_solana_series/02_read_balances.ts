import { clusterApiUrl, Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

const connection = new Connection('https://devnet.helius-rpc.com/?api-key=2c1b78e4-b4f3-46ec-8c12-18cd5b48f7f6')
const address = new PublicKey('k9NVtVp5r8Nn7xG94t5UjRKoqcbJ9GWBn76eEFXUbae')
const balance = await connection.getBalance(address)
const solBalance = balance / LAMPORTS_PER_SOL

console.log(`The balance of ${address} is ${solBalance} SOL.`);
console.log(`✅ Done!`)