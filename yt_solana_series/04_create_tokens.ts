import 'dotenv/config.js'
import {Connection, } from "@solana/web3.js";
import { getExplorerLink, getKeypairFromEnvironment } from '@solana-developers/helpers';
import { createMint } from '@solana/spl-token';

const keyPair = process.env.KEYPAIR || null

if (!keyPair) {
    console.log('Please provide a keypair');
    console.log('❗️❗️❗️');
    process.exit(1)
}

const connection = new Connection('https://devnet.helius-rpc.com/?api-key=2c1b78e4-b4f3-46ec-8c12-18cd5b48f7f6')
const owner = getKeypairFromEnvironment('KEYPAIR')

const token = await createMint(connection, owner, owner.publicKey, null, 2)
const link = getExplorerLink('address', token.toString(), "devnet")

console.log(`☑️Done! Create a token: ${link}`)