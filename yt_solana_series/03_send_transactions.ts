import 'dotenv/config.js'
import { Connection,
         LAMPORTS_PER_SOL,
         PublicKey,
         sendAndConfirmTransaction, 
         SystemProgram, Transaction } from "@solana/web3.js";
import { getKeypairFromEnvironment } from '@solana-developers/helpers';

const keyPair = process.env.KEYPAIR || null

if (!keyPair) {
    console.log('Please provide a keypair');
    console.log('❗️');
    process.exit(1)
}

const connection = new Connection('https://devnet.helius-rpc.com/?api-key=2c1b78e4-b4f3-46ec-8c12-18cd5b48f7f6')
const sender = getKeypairFromEnvironment('KEYPAIR')
const receiverAddress = new PublicKey('k9NVtVp5r8Nn7xG94t5UjRKoqcbJ9GWBn76eEFXUbae')

const amount = 3.88
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