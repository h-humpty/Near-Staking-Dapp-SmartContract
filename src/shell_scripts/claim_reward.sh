 echo "Please provide STAKING_CONTRACT_ADDRESS EX: ncd_staking_contract.testnet"
read STAKING_CONTRACT
echo "Please provide your testnet account id EX: johndoe.testnet"
read ACCOUNT
echo "Please provide STAKE_ID Ex: "1""
read STAKE_ID

near call $STAKING_CONTRACT claim_reward '{"stake_id": "'"$STAKE_ID"'"}' --accountId $ACCOUNT