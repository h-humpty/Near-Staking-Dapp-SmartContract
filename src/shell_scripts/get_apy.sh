echo "Please provide STAKING_CONTRACT_ADDRESS EX: ncd_staking_contract.testnet"
read STAKING_CONTRACT

echo "Please provide your FT_CONTRACT_ADDRESS Ex: ncd_ft_token.testnet"
read FT_CONTRACT

near view $STAKING_CONTRACT get_apy '{"ft_contract_id" : "'"$FT_CONTRACT"'"}'