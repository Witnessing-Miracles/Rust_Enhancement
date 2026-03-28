import 'dotenv/config.js'
import {Connection, PublicKey, sendAndConfirmTransaction, Transaction} from "@solana/web3.js";
import { getExplorerLink, getKeypairFromEnvironment } from '@solana-developers/helpers';
import { createCreateMetadataAccountV3Instruction } from '@metaplex-foundation/mpl-token-metadata'

const keyPair = process.env.KEYPAIR || null

if (!keyPair) {
    console.log('Please provide a keypair');
    console.log('❗️❗️❗️');
    process.exit(1)
}

const connection = new Connection('https://devnet.helius-rpc.com/?api-key=2c1b78e4-b4f3-46ec-8c12-18cd5b48f7f6')
const owner = getKeypairFromEnvironment('KEYPAIR')
const tokenAccount = new PublicKey('8ZW6m9UNBnBwbKy4YtrtyngS3AG5cLnpBn34um4FAoUz')

const TOKEN_METADATA_PROGRAM_ID = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')

const metadata = {
    name: "PL 1st Token on Solana",
    symbol: "PL1SOL",
    uri: 'https://dappmentors.org/',
    sellerFeeBasisPoints: 0.0001,
    creators: null,
    collection: null,
    uses: null,
}

const metadataTokenSync = PublicKey.findProgramAddressSync(
    [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        tokenAccount.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
)

const metadataPDA = metadataTokenSync[0]
const transaction = new Transaction()

const addMetadataToTokenInstruction = createCreateMetadataAccountV3Instruction(
    {
        metadata: metadataPDA,
        mint: tokenAccount,
        mintAuthority: owner.publicKey,
        payer: owner.publicKey,
        updateAuthority: owner.publicKey,
    },
    {
        createMetadataAccountArgsV3: {
            collectionDetails: null,
            data: metadata,
            isMutable: true,
        },
    }
)

transaction.add(addMetadataToTokenInstruction)

const signature = await sendAndConfirmTransaction(
    connection, transaction, [owner,]
)

let link = getExplorerLink('transaction', signature.toString(), "devnet")
console.log(`✅ Done! Token Metadata added: ${link}`)

link = getExplorerLink('address', tokenAccount.toString(), 'devnet')
console.log(`✅ Look up the token again: ${link}`)