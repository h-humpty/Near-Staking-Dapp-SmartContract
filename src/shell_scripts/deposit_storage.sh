# near call $CONTRACT_NAME deposit_for_storage '{"ft_contract_id" : "ncd_ft_token.testnet"}' --accountId  $ACCOUNT_ID --deposit 0.00859

echo "Please provide STAKING_CONTRACT_ADDRESS EX: ncd_staking_contract.testnet"
read STAKING_CONTRACT
echo "Please provide your testnet account id EX: johndoe.testnet"
read ACCOUNT
echo "Please provide your FT_CONTRACT_ADDRESS Ex: ncd_ft_token.testnet"
read FT_CONTRACT


near call $STAKING_CONTRACT deposit_for_storage '{"ft_contract_id":"ncd_ft_token.testnet"}' --accountId $ACCOUNT --deposit 0.00859

near call $STAKING_CONTRACT drop_ft '{"account_id":"'"$ACCOUNT"'", "ft_contract_id" : "'"$FT_CONTRACT"'"}' --account_id $ACCOUNT

