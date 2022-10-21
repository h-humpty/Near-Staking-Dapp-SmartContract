echo "Please provide STAKING_CONTRACT_ADDRESS EX: ncd_staking_contract.testnet"
read STAKING_CONTRACT
echo "Please provide your testnet account id EX: johndoe.testnet"
read ACCOUNT
echo "Please provide your FT_CONTRACT_ADDRESS Ex: ncd_ft_token.testnet"
read FT_CONTRACT


near call $FT_CONTRACT ft_transfer_call '{"receiver_id": "ncd_staking_contract.testnet",
"amount": "250000000000000000000000000","msg": "{\"ft_symbol\":\"UNCT\",\"ft_account_id\":\"ncd_ft_token.testnet\",\"decimal\":24,\"duration\":180,\"staked_by\":\"'"$ACCOUNT"'\",\"staking_plan\":\"3minutes\"}"}' --accountId $ACCOUNT --depositYocto 1 --gas 300000000000000

near call $FT_CONTRACT ft_transfer_call '{"receiver_id": "ncd_staking_contract.testnet",
  "amount": "500000000000000000000000000","msg": "{\"ft_symbol\":\"UNCT\",\"ft_account_id\":\"ncd_ft_token.testnet\",\"decimal\":24,\"duration\":180,\"staked_by\":\"'"$ACCOUNT"'\",\"staking_plan\":\"3minutes\"}"}' --accountId $ACCOUNT --depositYocto 1 --gas 300000000000000