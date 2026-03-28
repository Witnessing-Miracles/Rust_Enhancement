import 'dotenv/config.js'
import {Connection, PublicKey} from "@solana/web3.js";
import { getExplorerLink, getKeypairFromEnvironment } from '@solana-developers/helpers';
import { transfer, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';

const keyPair = process.env.KEYPAIR || null

if (!keyPair) {
    console.log('Please provide a keypair');
    console.log('❗️❗️❗️');
    process.exit(1)
}

const connection = new Connection('https://devnet.helius-rpc.com/?api-key=2c1b78e4-b4f3-46ec-8c12-18cd5b48f7f6')
const owner = getKeypairFromEnvironment('KEYPAIR')
const tokenMint = new PublicKey('8ZW6m9UNBnBwbKy4YtrtyngS3AG5cLnpBn34um4FAoUz')

const recipientAccount = new PublicKey('7zTWrgyVEMWDAvXqaV459Fu47fxwYXvuhFq4QPK2beoQ')

// 创建owner的Token账户
const ownerTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, owner, tokenMint, owner.publicKey
)

console.log(`Owner Token Account: ${ownerTokenAccount.address}`)

// 铸造Token给owner（这样才有Token可以转）
await mintTo(
    connection,
    owner,
    tokenMint,
    ownerTokenAccount.address,
    owner,
    50000000000  // 铸50个Token
)

console.log(`✅ Minted 50 tokens`)

// 创建recipient的Token账户
const recipientTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, owner, tokenMint, recipientAccount
)

// 转账
const signature = await transfer(
    connection,
    owner,
    ownerTokenAccount.address,
    recipientTokenAccount.address,
    owner.publicKey,
    36000000000
)

console.log(`✅ Transferred 18 tokens! ${signature}`)

const link = getExplorerLink('transaction', signature.toString(), "devnet")
console.log(`Check it out: ${link}`)