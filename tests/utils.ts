import {Umi, createGenericFile, generateSigner, percentAmount, Pda, Signer } from '@metaplex-foundation/umi'
import { irysUploader} from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";
import {createAndMint, TokenStandard, mplTokenMetadata, createV1, mintV1} from "@metaplex-foundation/mpl-token-metadata"
import {NonFungibleTokenMeta} from "./types"
import {TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID} from '@solana/spl-token';
import {fromWeb3JsPublicKey} from '@metaplex-foundation/umi-web3js-adapters'
import { findAssociatedTokenPda } from '@metaplex-foundation/mpl-toolbox'
import { Keypair, Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';

export async function uploadLogo(umi: Umi, imagePath: string): Promise<string>{
    umi.use(irysUploader())
    const cluster=umi.rpc.getCluster()
    
    console.log("⬆️ Uploading the logo...");
    let imageUri
    if (cluster=='localnet' || cluster=='custom'){
        imageUri = "https://devnet.irys.xyz/8QqR4QpyyjiF5qXTxExns89KpDEJwkGhNtxaScjgPka9" // example
    } else{
        const image = await readFile(imagePath); // Read the image file
        const imageFile = createGenericFile(image, "logo.png"); // Create a generic file object

        const imageUris = await umi.uploader.upload([imageFile], {
            onProgress: (percent: number) => {
                console.log(`${(percent * 100).toFixed(2)}% uploaded...`);
            },
        });
        imageUri = imageUris.join(""); // Concatenate the uploaded URIs (if multiple parts)
        
        // Correct the URI if it contains Arweave
        if (imageUri.includes('arweave.net')) {
            imageUri = imageUri.split('/').pop()!;
            imageUri = `https://devnet.irys.xyz/${imageUri}`;
        }
    }
    console.log(`✅ Image uri: ${imageUri}`);
    return imageUri
}

export async function uploadNonFungibleTokenMeta(umi: Umi, nonFungibleTokenMeta: NonFungibleTokenMeta, imageUri: string): Promise<string>{
    umi.use(irysUploader())
    const cluster=umi.rpc.getCluster()

    console.log("⬆️ Uploading the metadata...");
    let uri
    if (cluster=='localnet' || cluster=='custom'){
        uri = "https://devnet.irys.xyz/DbPDkeTvm9DTJXxkLEFvHJWDAfu3FortEDHTvsy9sSQB" // example
    } else{
        let attributes
        // create uri
        // ref: https://developers.metaplex.com/core/what-is-an-asset
        // ref: https://developers.metaplex.com/token-metadata/token-standard
        uri = await umi.uploader.uploadJson(
            {
                name: nonFungibleTokenMeta.name,
                description: nonFungibleTokenMeta.description,
                image: imageUri,
                // animation_url
                // external_url
                "attributes": nonFungibleTokenMeta.attributes
                // properties
            }
        )

        // Correct the URI if it contains Arweave
        if (uri.includes('arweave.net')) {
            uri = uri.split('/').pop()!;
            uri = `https://devnet.irys.xyz/${uri}`;
        }
    }

    console.log(`✅ uri: ${uri}`);

    return uri
}

export async function airdrop(address: any, endpoint: string, amount: number) {
    const publicKey=new PublicKey(address)
    const connection = new Connection(endpoint , 'confirmed');
    try {
        // Request an airdrop of 5 SOL (5 * 1_000_000_000 lamports)
        const signature = await connection.requestAirdrop(
            publicKey,
            amount * LAMPORTS_PER_SOL
        );
        await connection.confirmTransaction(signature, 'confirmed');
        console.log('✅ Airdrop successful!');

        const balance = await connection.getAccountInfo(publicKey)
        console.log(`💰 Balance for ${publicKey}: ${balance.lamports/LAMPORTS_PER_SOL} SOL`)
    } catch (error) {
        console.error('❌ Airdrop failed:\n', error);
    }
}

export async function delay(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
}