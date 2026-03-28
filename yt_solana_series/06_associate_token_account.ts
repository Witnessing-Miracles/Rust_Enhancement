import 'dotenv/config.js'
import {Connection, PublicKey, sendAndConfirmTransaction, Transaction} from "@solana/web3.js";
import { getExplorerLink, getKeypairFromEnvironment } from '@solana-developers/helpers';
import { createMint, getOrCreateAssociatedTokenAccount } from '@solana/spl-token';

const keyPair = process.env.KEYPAIR || null

if (!keyPair) {
    console.log('Please provide a keypair');
    console.log('❗️❗️❗️');
    process.exit(1)
}

const connection = new Connection('https://devnet.helius-rpc.com/?api-key=2c1b78e4-b4f3-46ec-8c12-18cd5b48f7f6')
const owner = getKeypairFromEnvironment('KEYPAIR')
const tokenAccount = new PublicKey('8ZW6m9UNBnBwbKy4YtrtyngS3AG5cLnpBn34um4FAoUz')

const recipientAccount = new PublicKey('7zTWrgyVEMWDAvXqaV459Fu47fxwYXvuhFq4QPK2beoQ')

const associateTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, owner, tokenAccount, recipientAccount
)

console.log(`Associate Token Account: ${associateTokenAccount.address}`)

const link = getExplorerLink('address', associateTokenAccount.address.toString(), "devnet")