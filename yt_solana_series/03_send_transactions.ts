import 'dotenv/config.js'
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import { getKeypairFromEnvironment } from '@solana-developers/helpers';

const keyPair = process.env.KEYPAIR || null

if (!keyPair) {
    console.log('Please provide a keypair');
    console.log('❗️');
    process.exit(1)
}

const connection = new Connection('https://devnet.helius-rpc.com/?api-key=2c1b78e4-b4f3-46ec-8c12-18cd5b48f7f6')
const sender = getKeypairFromEnvironment('KEYPAIR')
const receiverAddress = new PublicKey('CctenqEbn16y6CPiLp4VJg36iDWuPc4GAyy5BibA8F7g')

const amount = 0.88
const transaction = new Transaction()

const txInstruction = SystemProgram.transfer({
    fromPubkey: sender.publicKey,
    toPubkey: receiverAddress,
    lamports: amount * LAMPORTS_PER_SOL,
})

transaction.add(txInstruction)
const signature = await sendAndConfirmTransaction(connection, transaction, [sender])
console.log(`Tx successful, ${amount * LAMPORTS_PER_SOL} lamports sent to ${receiverAddress}`)
console.log(`✅ Transaction signature is ${signature}!`)

// const balance = await connection.getBalance(address.publicKey)
// const solBalance = balance / LAMPORTS_PER_SOL

// console.log(`The balance of ${address.publicKey} is ${solBalance} SOL`)
// console.log(`✅ Done!`)

// const connection = new Connection('https://devnet.helius-rpc.com/?api-key=2c1b78e4-b4f3-46ec-8c12-18cd5b48f7f6')
// const address = new PublicKey('k9NVtVp5r8Nn7xG94t5UjRKoqcbJ9GWBn76eEFXUbae')
// const balance = await connection.getBalance(address)
// const solBalance = balance / LAMPORTS_PER_SOL

// console.log(`The balance of ${address} is ${solBalance} SOL.`);
// console.log(`✅ Done!`)