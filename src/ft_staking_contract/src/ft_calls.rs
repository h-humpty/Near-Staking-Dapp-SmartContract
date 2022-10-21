use core::panic;

use crate::*;

use near_sdk::collections::UnorderedMap;
use near_sdk::env::{attached_deposit, log};
use near_sdk::json_types::U128;
use near_sdk::{ext_contract, log, Balance, Gas, PromiseOrValue, PromiseResult};

const BASE_GAS: Gas = Gas(5_000_000_000_000);
const STORAGE_DEPOSIT_GAS: Gas = Gas(300_000_000_000_000);

//const THIRTY_DAYS: u64 = 2592000; //30 days in seconds
const ONE_MINUTE: u64 = 60; //30 days in seconds

const STORAGE_DEPOSIT: Balance = 8590000000000000000000;

const ONE_YOCTO: Balance = 100000000000000000000000;
const TEN_THOUSAND: u128 = 10000000000000000000000000000;
const NO_BALANCE: Balance = 0;

pub trait FTActionsReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn ft_unstake(&mut self, stake_id: U128);

    fn did_promise_succeded() -> bool;

    fn unstake_callback(&mut self, stake_id: StakeId, staker_id: AccountId);

    fn claim_reward(&mut self, stake_id: StakeId);

    fn claim_reward_callback(
        &mut self,
        stake_id: StakeId,
        claim_history: Option<ClaimHistory>,
        claim_count: u64,
    );

    fn deposit_for_storage(&mut self, ft_contract_id: AccountId);

    fn drop_ft(&mut self, account_id: AccountId, ft_contract_id: AccountId);

    fn drop_ft_callback(&mut self, account_id: AccountId, ft_contract_id: AccountId);
}

#[ext_contract(ext_ft)]
trait FTCallbackReceiver {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn storage_deposit(account_id: Option<AccountId>);
}

#[ext_contract(this_contract)]
trait FTActionsSender {
    fn unstake_callback(&mut self, stake_id: StakeId, staker_id: AccountId);

    fn claim_reward_callback(
        &mut self,
        stake_id: StakeId,
        claim_history: Option<ClaimHistory>,
        claim_count: u64,
    );

    fn drop_ft_callback(&mut self, account_id: AccountId, ft_contract_id: AccountId);
}

#[near_bindgen]
impl FTActionsReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let a: u128 = 0;
        let staking_id = self
            .staking_nonce
            .checked_add(1)
            .expect("Exceeded u128 capacity");

        log!("{} staking id", staking_id);
        let staked_at = env::block_timestamp() / 1000000000;
        let StakeArgs {
            ft_symbol,
            ft_account_id,
            decimal,
            duration,
            staked_by,
            staking_plan,
        } = near_sdk::serde_json::from_str(&msg).expect("Invalid Staking Arguments");

        let stake = Stake {
            stake_id: U128::from(staking_id),
            ft_symbol,
            ft_account_id,
            decimal,
            amount,
            duration,
            staked_at: staked_at,
            staked_by,
            staking_plan,
        };

        //fetch apy details from FT
        let ft = self.approved_fts.get(&stake.clone().ft_account_id).unwrap();
        let apy_map = ft.apy_against_duration.unwrap();
        let apy = apy_map.get(&stake.clone().staking_plan);
        //fetch minimum staking amount from APY
        let threshold: u128 = u128::from(apy.unwrap().clone().min_staking_amount);

        let calling_contrat = env::predecessor_account_id();
        assert!(
            self.approved_ft_token_ids.contains(&calling_contrat),
            "Only approved FT can be staked"
        );
        assert!(
            u128::from(amount) >= threshold,
            "Cannot stake less than {} tokens",
            threshold
        );
        assert!(
            stake.duration / ONE_MINUTE >= apy.unwrap().min_duration.into(),
            "Invalid Duration"
        );

        if let Some(mut staking_history) = self.staking_history.get(&sender_id) {
            log!("In IF");
            staking_history.push(stake);
            self.staking_history.insert(&sender_id, &staking_history);
        } else {
            log!("In ELSE");
            let mut staking_history: Vec<Stake> = Vec::new();
            staking_history.push(stake);
            self.staking_history.insert(&sender_id, &staking_history);
        }

        log!(
            "{:?} staked by {} with staking_id {}",
            amount,
            sender_id,
            staking_id
        );
        self.staking_nonce = staking_id;
        near_sdk::PromiseOrValue::Value(U128::from(a))
    }

    fn ft_unstake(&mut self, stake_id: StakeId) {
        // let stake_id = u128::from(stake_id);
        let staker_id: AccountId = env::predecessor_account_id().try_into().unwrap();
        let stake_history = self.staking_history.get(&staker_id);

        // assert!(self.whitelist_addresses.contains(&staker_id), "Only whitelisted members can unstake tokens");

        let stake = stake_history
            .unwrap()
            .into_iter()
            .find(|i| i.stake_id == stake_id)
            .expect("No staking data with this id found for caller");

        let current_time = env::block_timestamp() / 1000000000;
        let staked_at = stake.staked_at;
        let duration = stake.duration;
        let amount = stake.amount;
        let staked_by = stake.staked_by;
        let ft_contract: AccountId = stake.ft_account_id.try_into().unwrap();
        let memo: Option<String> = Some("Unstaking with reward".to_string());

        assert_eq!(
            staked_by.to_string(),
            staker_id.to_string(),
            "Only owner of the tokens can unstake"
        );

        assert!(
            current_time >= staked_at + duration,
            "Cannot withdraw before locked time"
        );
        ext_ft::ft_transfer(
            staker_id.clone(),
            U128::from(amount),
            memo,
            ft_contract,
            1,
            BASE_GAS,
        )
        .then(this_contract::unstake_callback(
            stake_id,
            staker_id,
            env::current_account_id(),
            0,
            BASE_GAS,
        ));

        //remove staking info from vector
    }

    fn did_promise_succeded() -> bool {
        if env::promise_results_count() != 1 {
            log!("Expected a result on the callback");
            return false;
        }
        match env::promise_result(0) {
            PromiseResult::Successful(_) => true,
            _ => false,
        }
    }

    fn unstake_callback(&mut self, stake_id: StakeId, staker_id: AccountId) {
        if Self::did_promise_succeded() {
            let mut staking_history = self.staking_history.get(&staker_id).unwrap();
            let index = &staking_history.iter().position(|i| i.stake_id == stake_id);

            let _ = &staking_history.remove(index.unwrap());

            self.staking_history.insert(&staker_id, &staking_history);

            log!(
                "Staking ID {} removed from {}",
                u128::from(stake_id),
                index.unwrap()
            );
        }
    }

    fn claim_reward(&mut self, stake_id: StakeId) {
        let staker_id: AccountId = env::predecessor_account_id().try_into().unwrap();
        let stake_history = self
            .staking_history
            .get(&staker_id)
            .expect("This user has not staked yet.");

        let stake = stake_history
            .into_iter()
            .find(|i| i.stake_id == stake_id)
            .expect("No staking data with this id found for caller");

        let current_time = env::block_timestamp() / 1000000000;
        //let current_time = 1653764399;
        let staked_at = stake.staked_at;
        // let duration = stake.duration / THIRTY_DAYS;
        let amount = u128::from(stake.amount);
        let staked_by = stake.staked_by;
        // let symbol = stake.ft_symbol;
        // let decimal = stake.decimal;

        let claim_history = self.claim_history.get(&stake_id);

        assert_eq!(
            staked_by.to_string(),
            staker_id.to_string(),
            "Only owner of the tokens can claim reward"
        );
        //  assert!(self.whitelist_addresses.contains(&staker_id), "Only whitelisted members can claim reward tokens");

        let difference: u64;
        if claim_history.is_none() {
            difference = (current_time - staked_at) / ONE_MINUTE;
            //difference = 1;
            log!("{}", difference);
            assert!(
                difference >= 1,
                "Reward can be claimed after staking for 1 minute"
            );
        } else {
            let claimed_at = claim_history.clone().unwrap().last_claimed_at;
            difference = (current_time - claimed_at) / ONE_MINUTE;
            //difference = 2;
            log!(
                "Current Time : {} Claimed at : {} Difference {}",
                current_time,
                claimed_at,
                difference
            );
            assert!(
                difference >= 1,
                "Reward can be claimed after 1 minute of the last claim"
            );
        }

        //get FT details
        let ft = self.approved_fts.get(&stake.ft_account_id).unwrap();
        let apy_map = ft.apy_against_duration.unwrap();
        let apy = apy_map.get(&stake.staking_plan).unwrap();

        //log!("{:?}", apy);
        //let ap = apy.interest_rate / apy.min_duration as u16;
        let ap = apy.interest_rate;

        let interest = (amount * (ap as u128)) / 100;
        log!("interest {}", interest);
        let mut actual_amount = (interest / 10) * difference as u128; //calculae the reward according to the number of months passed since the user staked
        log!("actual_amount {}", actual_amount);
        actual_amount = actual_amount / apy.min_duration as u128;
        log!("Actual amount for transfer {}", actual_amount);

        let memo: Option<String> = Some("Reward tokens".to_string());
        ext_ft::ft_transfer(
            staker_id.clone(),
            U128::from(actual_amount),
            memo,
            stake.ft_account_id,
            1,
            BASE_GAS,
        )
        .then(this_contract::claim_reward_callback(
            stake_id,
            claim_history.clone(),
            difference,
            env::current_account_id(),
            0,
            BASE_GAS,
        ));
    }

    fn claim_reward_callback(
        &mut self,
        stake_id: StakeId,
        claim_history: Option<ClaimHistory>,
        claim_count: u64,
    ) {
        if Self::did_promise_succeded() {
            let claim: ClaimHistory;
            let current_time = env::block_timestamp() / 1000000000;
            if claim_history.is_none() {
                let count = claim_count as u8;
                claim = ClaimHistory {
                    last_claimed_at: current_time,
                    claim_count: count,
                }
            } else {
                claim = ClaimHistory {
                    last_claimed_at: current_time,
                    claim_count: claim_history.unwrap().claim_count + 1,
                }
            }
            self.claim_history.insert(&stake_id, &claim);
        }
    }

    #[payable]
    fn deposit_for_storage(&mut self, ft_contract_id: AccountId) {
        let caller_id = env::predecessor_account_id();
        let balance = env::attached_deposit();
        assert_eq!(
            attached_deposit(),
            STORAGE_DEPOSIT,
            "must attach 0.00859 NEAR"
        );
        ext_ft::storage_deposit(
            Some(caller_id.clone()),
            ft_contract_id.clone(),
            balance,
            BASE_GAS,
        );

        if let Some(mut depositted_for_storage) = self.depositted_for_storage.get(&ft_contract_id) {
            depositted_for_storage.insert(&caller_id);
            self.depositted_for_storage
                .insert(&ft_contract_id, &depositted_for_storage);
        } else {
            let mut members: UnorderedSet<AccountId> = UnorderedSet::new(b"s");
            members.insert(&caller_id);
            self.depositted_for_storage
                .insert(&ft_contract_id, &members);
        }
        /*
        if self.depositted_for_storage.get(&ft_contract_id).is_some(){
            self.depositted_for_storage.get(&ft_contract_id).unwrap().insert(&caller_id);
        }else{
            let mut members: UnorderedSet<AccountId> = UnorderedSet::new(b"s");
            members.insert(&caller_id);
            self.depositted_for_storage.insert(&ft_contract_id, &members);

        } */
        log!("true")
    }

    fn drop_ft(&mut self, account_id: AccountId, ft_contract_id: AccountId) {
        let memo: String = "Airdrop".to_string();

         if let Some(registered_members) = self.registered_members.get(&ft_contract_id){
            if registered_members.contains(&account_id){
                panic!("already claimed drop");
            }
        }
        ext_ft::ft_transfer(
            account_id.clone(),
            U128::from(TEN_THOUSAND),
            Some(memo),
            ft_contract_id.clone(),
            1,
            BASE_GAS,
        )
        .then(this_contract::drop_ft_callback(
            account_id,
            ft_contract_id,
            env::current_account_id(),
            NO_BALANCE,
            BASE_GAS,
        ));
    }
    fn drop_ft_callback(&mut self, account_id: AccountId, ft_contract_id: AccountId) {
        //  log!("acct_id {}",account_id);
        if Self::did_promise_succeded() {
            if let Some(mut members) = self.registered_members.get(&ft_contract_id) {
                members.push(account_id);
                self.registered_members.insert(&ft_contract_id, &members);
                // log!("in if acct_id =  {} reg_mem = {:?}",account_id,members);
            } else {
                let mut members: Vec<AccountId> = Vec::new();
                members.push(account_id);
                self.registered_members.insert(&ft_contract_id, &members);
            }
        }
    }
}
