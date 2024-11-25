#
## Introduction
Sample Rust program for Solana based on [Anchor Framework](https://solana.com/docs/programs/anchor)
Here some useful resources:
- [Program Structure](https://solana.com/docs/programs/anchor/program-structure)
- [Program Examples](https://solana.com/docs/programs/examples)
- [Anchor by Example](https://examples.anchor-lang.com/docs/onchain-voting)


1. Install the required toolchain as described in [Installation](https://solana.com/docs/intro/installation)
2. Install required dependencies  
```bash
cargo add <crate_name>
```
3. Update cargo modules  
```bash
cd programs/<program_name>
cargo update
```

## Testing
### Local Environment
1. Start a [Local Solana Cluster](https://solana.com/developers/guides/getstarted/solana-test-validator) (ref: [Test the Program](https://solana.com/docs/programs/anchor)):
1.1 Start the cluster  
``` bash 
solana-test-validator
```
1.2
Add addtional programs 
```bash
solana program dump -um metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s ./tests/metaplex_token_metadata_program.so
solana program dump -um TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA ./tests/spl_token_program.so
solana program dump -um CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d ./tests/mpl_core_program.so
solana program dump -um TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb ./tests/token_2022_program.so
solana program dump -um ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL ./tests/associated_token_program.so
```

Add the programs to ```Anchor.toml```
```
[[test.genesis]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"  
program = "tests/metaplex_token_metadata_program.so"
```

In a different prompt 
``` bash 
solana config set --url http://127.0.0.1:8899
```
Cloning programs
```bash 
solana-test-validator \
--bpf-program --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s ./tests/metaplex_token_metadata_program.so \
--bpf-program --bpf-program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA ./tests/spl_token_program.so \
--bpf-program --bpf-program CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d ./tests/mpl_core_program.so \
--bpf-program --bpf-program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb ./tests/token_2022_program.so \
--bpf-program --bpf-program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL ./tests/associated_token_program.so \
--reset
```

2. Deploy 
- 2.1 Configure the Anchor Project for Devnet deployment in ```Anchor.toml```
```toml
[clusters]
localnet = "http://127.0.0.1:8899"
```
- 2.2 Build the Anchor Program
``` bash
anchor build
```
- 2.3 Deploy the program locally 
``` bash
anchor deploy --provider.cluster localnet
```

3. Test
``` bash 
anchor test --skip-local-validator
```
## Deployment
### Devnet Deployment
1. Configure the Anchor Project for Devnet deployment in ```Anchor.toml``` (ref [Deploy to Devnet](https://solana.com/docs/programs/anchor))
``` bash 
[provider]
cluster = "Mainnet"
wallet = "~/.config/solana/id.json"
```
The Solana wallet should have enough SOL founds (ref: [How to get Solana devnet SOL (including airdrops and faucets)](https://solana.com/developers/guides/getstarted/solana-token-airdrop-and-faucets)). For Airdrop  ([Solana Faucet](https://faucet.solana.com))
``` bash
solana config set --url <your RPC url>
solana airdrop 2
```

## Verify the Build  
[How to Verify a Program](https://solana.com/developers/guides/advanced/verified-builds)


## Rust Enviornment
- Check rust tree (for crates dependencies)
```bash
cargo tree -e features
```

## References
- [Account Constraints](https://www.anchor-lang.com/docs/account-constraints)
- [Account Types](https://www.anchor-lang.com/docs/account-types)
- [Anchor.toml](https://www.anchor-lang.com/docs/manifest)
- [Metaplex Token Metadata](https://developers.metaplex.com/token-metadata)
- [Account Space Chart](https://www.anchor-lang.com/docs/space)