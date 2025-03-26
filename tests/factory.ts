import * as anchor from "@coral-xyz/anchor";
import { Program, web3} from "@coral-xyz/anchor";
import { Factory } from "../target/types/factory";
import { PublicKey, Keypair, LAMPORTS_PER_SOL, clusterApiUrl, Connection} from "@solana/web3.js";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createSignerFromKeypair, signerIdentity} from '@metaplex-foundation/umi'
import { TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, TokenError, thawAccountInstructionData} from "@solana/spl-token";
import { fetchAsset} from '@metaplex-foundation/mpl-core'
import { assert } from "chai";
import * as borsh from "borsh";

import {SolanaNetwork, TokenConfigs} from './types'
import { airdrop, delay, deserializeWhitelist, fetch_nft_core } from "./utils";

import configs from './config.json';
import wallet from '../2rz6Z257ETCouoWTYTZYu9wV6zGE7pVPNXmm6689S3XS.json'
import { token } from "@coral-xyz/anchor/dist/cjs/utils";

const tokenConfigs: TokenConfigs = configs;
console.log(`Configs\n${tokenConfigs}`)
const privateKeyArray=new Uint8Array(wallet)

var solanaNetwork: SolanaNetwork
if (tokenConfigs.solanaNetwork.toLowerCase() == 'localnet') {
    solanaNetwork = SolanaNetwork.Localnet;
} else if (tokenConfigs.solanaNetwork.toLowerCase() == 'mainnet') {
    solanaNetwork = SolanaNetwork.MainnetBeta;
} else if (tokenConfigs.solanaNetwork.toLowerCase() == 'devnet') {
    solanaNetwork = SolanaNetwork.Devnet;
}

describe("factory", () => {
  // Configure the client to use the local cluster
  let provider
  if (solanaNetwork==SolanaNetwork.Localnet){
    // working
    provider = anchor.AnchorProvider.env();

  } else if (solanaNetwork==SolanaNetwork.Devnet){
    let connection = new Connection(clusterApiUrl('devnet'));
    const keypair = Keypair.fromSecretKey(Uint8Array.from(privateKeyArray));
    const wallet = new anchor.Wallet(keypair);
    provider = new anchor.AnchorProvider(connection, wallet, {
      commitment: "processed",
    });
  }
  anchor.setProvider(provider);
  console.log(`🔗 Connection: ${provider.connection.rpcEndpoint}`)
  console.log('🗝️ Anchor wallet Public Key:', provider.wallet.publicKey.toString());

  // get the program
  const program = anchor.workspace.Factory as Program<Factory>;

  // create umi
  const umi = createUmi(solanaNetwork, "confirmed")
  const keypair = umi.eddsa.createKeypairFromSecretKey(privateKeyArray)
  umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

  // get the cluster 
  const cluster=umi.rpc.getCluster()

  // set the payer 
  const payer = Keypair.fromSecretKey(privateKeyArray); 
  // const payer = umi.payer

  // create battery unique ID
  const id = generateRandomString(10)
  tokenConfigs.tokenMeta.name=id
  
  it("Initializes the program", async () => {
    // airdrop funds
    await airdrop(payer.publicKey, solanaNetwork, 5);

    // derive the tokenRegistry account as PDA with seeds ['token_registry']
    const [tokenRegistryPDA, tokenRegistryBump] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('token_registry'),
      ],
      program.programId
    )

    // invoke the init method
    await program.methods
    .init() // Call the function with the InitTokenParams struct
    .accounts({
        tokenRegistry: tokenRegistryPDA
    })
    .rpc();

    // check that the token registry account has been created
    const accountInfo = await provider.connection.getAccountInfo(tokenRegistryPDA);
    assert(accountInfo.owner.equals(program.programId), `token_registry account owner: ${accountInfo.owner}\n program id: ${program.programId}`);
  });

  it ("Whitelist is working", async () => {
    // generate a third party
    const thirdParty1 = Keypair.generate();
    const thirdParty2 = Keypair.generate();
    let error=false
    
    await program.methods
                .addToWhitelist(thirdParty1.publicKey)
                .accounts({})
                .signers([payer])
                .rpc();
    await delay(1000)

    const [whitelistPDA, assetBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("whitelist")
      ],
      program.programId
    );

    const whitelist = await deserializeWhitelist(provider.connection, whitelistPDA);

    assert(
      whitelist.authorized_users.some((key: PublicKey) => key.toString() === thirdParty1.publicKey.toString()), 
      `❌ User ${thirdParty1.publicKey.toString()} not in whitelist`
    );

    try{
      await program.methods
      .addToWhitelist(thirdParty2.publicKey)
      .accounts({})
      .signers([thirdParty2])
      .rpc();
      await delay(1000)
    } catch {
      error=true
    }
    assert(error==true, `❌ A malicious actor is able to modify the whitelist`)

    await program.methods
    .removeFromWhitelist(thirdParty1.publicKey)
    .accounts({})
    .signers([payer])
    .rpc();
    await delay(1000)

    assert(
      whitelist.authorized_users.some((key: PublicKey) => key.toString() !== thirdParty1.publicKey.toString()), 
      `❌ User ${thirdParty1.publicKey.toString()} still in whitelist`
    );

  })

  it("Creates a dynamic core NFT", async () => {
    // get the asset PDA
    const [assetPDA, assetBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("asset"), 
        Buffer.from(id)
      ],
      program.programId
    );
    
    console.log(tokenConfigs.tokenMeta)
        
    // Call the mint_nft_core function
    await program.methods
    .mintNftCore(tokenConfigs.tokenMeta)
    .accounts({
        signer: payer.publicKey,
        payer: payer.publicKey,
    })
    .signers([payer])
    .rpc();
    await delay(10000)

    console.log(`✅ Token ${tokenConfigs.tokenMeta.name} created and minted successfully!`);
    if (cluster=='localnet' || cluster=='custom'){
        console.log(`🔑 Solana Explorer:   https://explorer.solana.com/address/${assetPDA}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`)
        console.log(`🔑 Solscan:           https://solscan.io/token/${assetPDA}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`)
    } else {
        console.log(`🔑 Solana Explorer:   https://explorer.solana.com/address/${assetPDA}?cluster=${cluster}`)
        console.log(`🔑 Solscan:           https://solscan.io/token/${assetPDA}?cluster=${cluster}`)
    }
    
    // run tests
    // fetch the asset
    const nftCoreAsset = await fetch_nft_core(tokenConfigs.tokenMeta.name, program.programId, umi)

    // check the name
    assert(nftCoreAsset.name == tokenConfigs.tokenMeta.name, "❌ name not corresponding")
    // check the uri
    assert(nftCoreAsset.uri == tokenConfigs.tokenMeta.uri, '❌ uri not corresponding')
    // check the properties
    let propertiesKeys=[]
    for (const property of tokenConfigs.tokenMeta.properties ?? []) {
        propertiesKeys.push(property.key)
    }
    for (const attribute of nftCoreAsset.attributes?.attributeList ?? []) {
        assert(propertiesKeys.includes(attribute.key), `❌ ${attribute} not in ${propertiesKeys}`)
    }
  });

  it("Updates properties of the dynamic core NFT", async() => {
    // get the asset PDA
    const [assetPDA, assetBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("asset"), 
        Buffer.from(id)
      ],
      program.programId
    );

    // update one of the properties 
    const updatePropertyKey=tokenConfigs.tokenMeta.properties[0].key
    const updatePropertyValue='updated'
    tokenConfigs.tokenMeta.properties[0].value=updatePropertyValue
    console.log(tokenConfigs.tokenMeta.properties)

    await program.methods.updatePropertiesNftCore(tokenConfigs.tokenMeta)
    .accounts({
      signer: payer.publicKey,
      payer: payer.publicKey,
    })
    .signers([payer])
    .rpc();
    await delay(10000)

    // run tests
    // fetch the asset
    const nftCoreAsset = await fetchAsset(umi, assetPDA.toString(), {
      skipDerivePlugins: false,
    })

    // check if the property has been updated
    const properties: { [key: string]: string }={}
    for (const attribute of nftCoreAsset.attributes?.attributeList ?? []) {
      properties[attribute.key]=attribute.value
    }
    assert(properties[updatePropertyKey]==updatePropertyValue, `❌ ${updatePropertyKey} not updated is instead ${properties[updatePropertyKey]}`)
  })

  it("A third part should fail to udpate the core NFT", async() => {
    // generate a third party
    const thirdParty = Keypair.generate();
    let error=false
    
    // try to update the token
    try{
      await program.methods.updatePropertiesNftCore(tokenConfigs.tokenMeta)
      .accounts({
        signer: thirdParty.publicKey, // in this case we simulate a malicious actor trying to update the token
        payer: payer.publicKey,
      })
      .signers([thirdParty])
      .rpc();
      await delay(10000)
    } catch {
      error=true
    }

    assert(error==true, `❌ A malicious actor is able to update the token`)
  })

  it ("Updates the dynamic core NFT", async() => {

    const newUri = 'newUri'
    tokenConfigs.tokenMeta.uri=newUri

    // Call the mint_nft_core function
    await program.methods
    .updateNftCore(tokenConfigs.tokenMeta)
    .accounts({
        signer: payer.publicKey,
        payer: payer.publicKey,
    })
    .signers([payer])
    .rpc();
    await delay(10000)

    // run tests
    const nftCoreAsset = await fetch_nft_core(tokenConfigs.tokenMeta.name, program.programId, umi)
    assert(nftCoreAsset.uri == tokenConfigs.tokenMeta.uri, `❌ uri not updated remains ${nftCoreAsset.uri}`)
  })

  it ("Transfers the dynamic core NFT", async() => {
    const thirdParty1 = Keypair.generate();

    // Call the mint_nft_core function
    await program.methods
    .transferNftCore(tokenConfigs.tokenMeta)
    .accounts({
        signer: payer.publicKey,
        payer: payer.publicKey,
        newOwner: thirdParty1.publicKey
    })
    .signers([payer])
    .rpc();
    await delay(10000)

    // run tests
    const nftCoreAsset = await fetch_nft_core(tokenConfigs.tokenMeta.name, program.programId, umi)
    assert(nftCoreAsset.owner.toString() == thirdParty1.publicKey.toString(), `❌ owner not updated remains ${nftCoreAsset.owner.toString()}`)
    assert(nftCoreAsset.updateAuthority.address.toString() == thirdParty1.publicKey.toString(), `❌ update authority not updated remains ${nftCoreAsset.updateAuthority.address.toString()}`)

  })
});

// function to check that external programs are present in the test environment
async function checkAccounts (connection: Connection){
  let accountInfo
  let id: string
  // check that external accounts are available in this session
  try{
    id = 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
    accountInfo = await connection.getAccountInfo(new PublicKey(id));
    console.log(accountInfo)
  } catch (error: unknown) {
    console.warn(`metaplex token metadata program @ ${id} not found\n${error}`)
  }
  
  try{
    id = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
    accountInfo = await connection.getAccountInfo(new PublicKey(id));
    console.log(accountInfo)
  } catch (error: unknown) {
    console.warn(`spl token program @ ${id} not found\n${error}`)
  }
}

function generateRandomString(length: number): string {
  const charset = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let randomString = '';
  for (let i = 0; i < length; i++) {
      const randomIndex = Math.floor(Math.random() * charset.length);
      randomString += charset[randomIndex];
  }
  return randomString;
}

export interface TokenInfo {
  id: string;           // Token ID
  token_mint: PublicKey;  // Token mint address (Pubkey)
}

export interface TokenRegistry {
  tokens: TokenInfo[];  // Array of TokenInfo
  total_tokens: number; // Total number of tokens
}

export interface InitTokenParams {
  id: string;
  uri: string;
}