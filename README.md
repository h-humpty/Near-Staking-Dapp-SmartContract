# Staking-DApp

Staking application consists of two smart contracts and React a web application.The purpose of this application is to allow users to earn rewards by staking their savings in terms of fungible tokens resulting in the multiplication of their savings.

# Prerequisites

In order to successfully compile the codes and run the application on local server you will need:
- [Node v18](https://nodejs.org/en/download/current/)
- [NEAR CLI](https://docs.near.org/docs/tools/near-cli)
- [RUST](https://www.rust-lang.org/tools/install)
- Add the WASM (WebAssembly) target to the toolchain by executing ` rustup target add wasm32-unknown-unknown` in terminal or powershell 

# Build

In order to generate the wasm files for the contracts, navigate to `/src/shell_scripts` and run the `build.sh` script by typing `./build.sh` in the rtminal or powershell.

# Simulation Tests

In order to execute the simulation tests, navigate to the `src/shell_scripts` and run the `test.sh` script by typing `./test.sh`

# Test Staking Functionality Using CLI

In order to test the whole functionality of the application, please runt the following scripts in sequence
- `deposit_storage.sh` this script allows users to deposit 0.00859 NEAR to th FT contract so, their acocunting can be mainitained on the contract, on successful depoist 10,000 UNCT will be transferred to the specified account for testing purposes.
- `stake.sh` this script enables users to stake tokens for 3 minutes the list of APY can be retrieved by running the `get_apy.sh` ## You will need to change the `duration` and `staking_plan`
- `claim.sh` this script allows users to claim rewards but after staking for atleast 1 minute, users will wait for 1 minute to carry out subsequent claims
- `unstake.sh` this script allows stakers to withdraw thier tokens after the lock period ends.

## Note
 You will need a testnet account in order to interact with the smart contract an account can be created from [here]([wal](https://wallet.testnet.near.org)
 Please use the follwoing addresses for staking contract and fungible token contract respectively
 #### staking contract address = ncd_staking_contract.testnet
 #### fungible token contract address = ncd_ft_token.testnet
