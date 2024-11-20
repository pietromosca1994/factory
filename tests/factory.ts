import * as anchor from "@coral-xyz/anchor";
import { Program, web3} from "@coral-xyz/anchor";
import { Factory } from "../target/types/factory";
import { PublicKey, Keypair, LAMPORTS_PER_SOL, Connection} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress} from "@solana/spl-token";
import { assert } from "chai";
import { token } from "@coral-xyz/anchor/dist/cjs/utils";

describe("factory", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  console.log(`Connection: ${provider.connection.rpcEndpoint}`)

  const program = anchor.workspace.Factory as Program<Factory>;

  // Metaplex Constants
  const TOKEN_METADATA_PROGRAM_ID = new web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  const SYSVAR_INSTRUCTIONS_ID = new PublicKey("Sysvar1nstructions1111111111111111111111111");

  // Create keypairs for the payer and mint accounts
  const payer = provider;

  const id = generateRandomString(10)
  
  const [tokenRegistryPDA, tokenRegistryBump] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('token_registry'),
    ],
    program.programId
  )


  it("Initializes the program", async () => {

    // derive the tokenRegistry account
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

  it("Creates an NFT", async () => {

    // await checkAccounts(provider.connection);

    // Airdrop SOL to payer for testing
    // const airdropSignature = await provider.connection.requestAirdrop(payer.publicKey, 5e9);
    // await provider.connection.confirmTransaction(airdropSignature, "confirmed");
    // const balance = await provider.connection.getAccountInfo(payer.publicKey)
    // console.log(`Balance for ${payer.publicKey}: ${balance.lamports/LAMPORTS_PER_SOL}`)
    const mint = Keypair.generate();

    // Create the InitTokenParams struct
    const token_params = {
        id: id,
        uri: "https://arweave.net/mOOBHcZUTm3DQ_srGltVfWGwqKcMI0_6wpolJ-rxlVA", // Replace with your metadata URI
    };

    // Derive the mint PDA
    const [mintPDA, mintBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("mint"), 
        Buffer.from(token_params.id)
      ],
      program.programId
    );

    // Derive the associated token address amount for the mint
    const [tokenAccount, tokenAccountBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        provider.publicKey.toBuffer(), 
        TOKEN_2022_PROGRAM_ID.toBuffer(),
        mintPDA.toBuffer()
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    // Derive the metadata account PDA
    const [metadataPDA, metadataBump] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintPDA.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    // Derive the master edition PDA
    const [masterEditionPDA, masterEditionPDABump] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintPDA.toBuffer(),
        Buffer.from('edition'),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    console.log("\nAddresses")
    console.log(`provider account:              ${provider.publicKey}`)
    console.log(`payer account:                 ${payer.publicKey}`)
    console.log(`mint account:                  ${mint.publicKey}`)
    console.log(`token account PDA account:     ${tokenAccount}`)
    console.log(`mint PDA account:              ${mintPDA}`)
    console.log(`master edition PDA account:    ${masterEditionPDA}`)
    console.log(`metadata PDA account:          ${metadataPDA}`)
    console.log(`token registry PDA account:    ${tokenRegistryPDA}`)

    console.log("\nPrograms")
    console.log(`token program:                 ${TOKEN_PROGRAM_ID}`)
    console.log(`token 2022 program:            ${TOKEN_2022_PROGRAM_ID}`)
    console.log(`token metadata program:        ${TOKEN_METADATA_PROGRAM_ID}`)
    
    // Call the mint_nft function
    await program.methods
        .mintNft(token_params) // Call the function with the InitTokenParams struct
        .accounts({
            signer: provider.publicKey,
            mint: mint,
            tokenAccount: tokenAccount,
            metadata: metadataPDA,
            masterEdition: masterEditionPDA,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
            sysvarInstructions: SYSVAR_INSTRUCTIONS_ID,
            tokenRegistry: tokenRegistryPDA
        })
        // .signers([provider])
        .rpc();

    let tokenAmount = await provider.connection.getTokenAccountBalance(tokenAccount);
    assert(tokenAmount.value.uiAmount==1, "The account holds more than 1 token");
  });

  it("The token_registry is updated", async () => {
    const tokenRegistryAccount = await program.account.tokenRegistry.fetch(tokenRegistryPDA);
    assert(tokenRegistryAccount.tokens.at(-1).id==id)
  });

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