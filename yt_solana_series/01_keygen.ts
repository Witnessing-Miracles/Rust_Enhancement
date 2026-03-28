import { Keypair } from "@solana/web3.js";

const keypair = Keypair.generate()

console.log(`This is the public key: ${keypair.publicKey.toBase58()}`)
console.log(`This is the private key: ${keypair.secretKey}`)
console.log(`This is the key pair: ${keypair}`)