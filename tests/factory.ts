import * as anchor from "@coral-xyz/anchor";
import { Program, web3} from "@coral-xyz/anchor";
import { Factory } from "../target/types/factory";
import { PublicKey, Keypair, LAMPORTS_PER_SOL, Connection} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress} from "@solana/spl-token";
import { assert } from "chai";

describe("factory", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  console.log(`Connection: ${provider.connection.rpcEndpoint}`)

  const program = anchor.workspace.Factory as Program<Factory>;

  // Metaplex Constants
  const TOKEN_METADATA_PROGRAM_ID = new web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

  // Create keypairs for the payer and mint accounts
  const payer = provider;
  const mint = Keypair.generate();

  it("Create an asset", async () => {

    await checkAccounts(provider.connection);

    // Airdrop SOL to payer for testing
    // const airdropSignature = await provider.connection.requestAirdrop(payer.publicKey, 5e9);
    // await provider.connection.confirmTransaction(airdropSignature, "confirmed");
    // const balance = await provider.connection.getAccountInfo(payer.publicKey)
    // console.log(`Balance for ${payer.publicKey}: ${balance.lamports/LAMPORTS_PER_SOL}`)

    // Create the InitTokenParams struct
      const token_params = {
        id: '1234',
        uri: "https://example.com/metadata.json", // Replace with your metadata URI
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
    const associatedTokenAccount = anchor.utils.token.associatedAddress({
      mint: mintPDA,
      owner: payer.publicKey,
    });

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
    console.log(`Token Associated Account:      ${associatedTokenAccount}`)
    console.log(`Token Program:                 ${TOKEN_PROGRAM_ID}`)
    console.log(`Token 2022 Program:            ${TOKEN_2022_PROGRAM_ID}`)
    console.log(`Token Metadata Program:        ${TOKEN_METADATA_PROGRAM_ID}`)
    console.log(`System Program:                ${anchor.web3.SystemProgram.programId}`)
    console.log(`mint PDA account:              ${mintPDA}`)
    console.log(`master edition PDA account:    ${masterEditionPDA}`)
    console.log(`metadata PDA account:          ${metadataPDA}`)

    console.log("\nBumps")
    console.log(`master edition PDA Bump:       ${masterEditionPDABump}`)
    console.log(`metadata PDA Bump:             ${metadataBump}`)
    // console.log(`metadataPDA account: ${metadataPDA}`)
    
    // Call the mint_nft function
    await program.methods
        .mintNft(token_params) // Call the function with the InitTokenParams struct
        .accounts({
            signer: provider.publicKey,
            mint: mint,
            metadata: metadataPDA,
            masterEdition: masterEditionPDA,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        })
        .rpc();
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
