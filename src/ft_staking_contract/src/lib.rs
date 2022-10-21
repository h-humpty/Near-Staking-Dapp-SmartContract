use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};
use std::cmp::min;
use std::collections::HashMap;

pub type APYKey = String; //6 months =  6months
pub type StakeId = U128;

mod ft_calls;
mod internal;

/* #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    approved_ft_token_ids: UnorderedSet<AccountId>,
    approved_fts: LookupMap<AccountId, FT>,
    staking_history: LookupMap<AccountId, Vec<Stake>>,
    staking_nonce: u128,
    claim_history: LookupMap<StakeId, ClaimHistory>,
} */

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub approved_ft_token_ids: UnorderedSet<AccountId>,
    pub approved_fts: LookupMap<AccountId, FT>,
    pub staking_history: LookupMap<AccountId, Vec<Stake>>,
    pub staking_nonce: u128,
    pub claim_history: LookupMap<StakeId, ClaimHistory>,
    pub registered_members : LookupMap<AccountId, Vec<AccountId>>,
    depositted_for_storage : LookupMap<AccountId,UnorderedSet<AccountId>>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ContractV2 {
    pub owner_id: AccountId,
    pub approved_ft_token_ids: UnorderedSet<AccountId>,
    pub approved_fts: LookupMap<AccountId, FT>,
    staking_history: LookupMap<AccountId, Vec<Stake>>,
    staking_nonce: u128,
    claim_history: LookupMap<StakeId, ClaimHistory>,
    registered_members : LookupMap<AccountId, UnorderedSet<AccountId>>,
    depositted_for_storage : LookupMap<AccountId,UnorderedSet<AccountId>>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Stake {
    stake_id: StakeId,
    ft_symbol: String,
    ft_account_id: AccountId,
    decimal: u8,
    amount: U128,
    duration: u64,  //in seconds
    staked_at: u64, //UNIX time : 1652793005
    staked_by: AccountId,
    staking_plan: String, //6months
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FT {
    pub account_id: AccountId,
    pub symbol: String,
    pub apy_against_duration: Option<HashMap<APYKey, APY>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct APY {
    pub apy_key: APYKey,
    pub min_staking_amount: U128,
    pub min_duration: u8,   //Ex 3 for 3 months
    pub interest_rate: u16, // Ex: 10% = 1000
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeArgs {
    ft_symbol: String,
    ft_account_id: AccountId,
    decimal: u8,
    duration: u64, //duration in milliseconds Ex 30 days = 2629800
    staked_by: AccountId,
    staking_plan: String, //Ex 6months
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ClaimHistory {
    last_claimed_at: u64,
    claim_count: u8,
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys {
    ApproveFungibleTokens,
    AmountStaked,
    ClaimHistory,
    RegisteredMembers,
    DeposittedForStorage
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, approved_ft_token_ids: Vec<FT>, ft_apy: Vec<APY>) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut this = Self {
            owner_id: owner_id.into(),
            approved_ft_token_ids: UnorderedSet::new(StorageKeys::ApproveFungibleTokens),
            approved_fts: LookupMap::new(StorageKeys::ApproveFungibleTokens),
            staking_history: LookupMap::new(StorageKeys::AmountStaked),
            claim_history: LookupMap::new(StorageKeys::ClaimHistory),
            staking_nonce: 0,
            registered_members : LookupMap::new(StorageKeys::RegisteredMembers),
            depositted_for_storage : LookupMap::new(StorageKeys::DeposittedForStorage)
        };

        Contract::add_fts(
            approved_ft_token_ids,
            &mut this.approved_fts,
            &mut this.approved_ft_token_ids,
            ft_apy,
        );

        this
    }

    #[init(ignore_state)]
    pub fn migrate()->ContractV2{
        let prev: Contract = env::state_read().expect("ERR_NOT_INITIALIZED");

        assert_eq!(
            env::predecessor_account_id(),
            prev.owner_id,
            "Only Owner can call this function"
        );

        let this = ContractV2{
            owner_id : prev.owner_id,
            approved_ft_token_ids : prev.approved_ft_token_ids,
            approved_fts : prev.approved_fts,
            staking_history : prev.staking_history,
            claim_history : prev.claim_history,
            staking_nonce : prev.staking_nonce,
            registered_members : LookupMap::new(StorageKeys::RegisteredMembers),
            depositted_for_storage : LookupMap::new(StorageKeys::DeposittedForStorage)
        };

        this

    }

    pub fn get_staking_history(
        self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Stake> {
        let mut stake_history = vec![];
        if self.staking_history.get(&account_id).is_none() {
            return stake_history;
        }
        let owner_stakes = self.staking_history.get(&account_id).unwrap();
        let start = u128::from(from_index.unwrap_or(U128(0)));
        let end = min(
            start + (limit.unwrap_or(0) as u128),
            owner_stakes.len().try_into().unwrap(),
        );

        for i in start..end {
            stake_history.push(owner_stakes[i as usize].clone());
        }

        self.staking_history.get(&account_id).unwrap()
    }

    pub fn get_claim_history(self, stake_id: StakeId) -> Option<ClaimHistory> {
        self.claim_history.get(&stake_id)
    }

    pub fn is_registered(&self,ft_contract_id:AccountId, account_id: AccountId)->bool{
        let registered_members = self.registered_members.get(&ft_contract_id);
            if registered_members.unwrap().contains(&account_id){
                true
            }else{
                false
            }
    }

    pub fn all_registered_memebers(&self, ft_contract_id:AccountId)->Vec<AccountId>{
        self.registered_members.get(&ft_contract_id).unwrap().to_vec()
    }

    pub fn all_storage_deposit (&self, ft_contract_id:AccountId)->Vec<AccountId>{
        self.depositted_for_storage.get(&ft_contract_id).unwrap().to_vec()
    }

   /*  pub fn insert_test (&mut self, ft_contract_id:AccountId, account_id: AccountId){
        let mut members: UnorderedSet<AccountId> = self.registered_members.get(&ft_contract_id).unwrap();
        members.insert(&account_id);
        self.registered_members.insert(&ft_contract_id, &members);
    } */

    pub fn has_depositted_for_storage(&self,account_id: AccountId,ft_contract_id:AccountId)->bool{
        if self.depositted_for_storage.get(&ft_contract_id).is_some(){
            if self.depositted_for_storage.get(&ft_contract_id).unwrap().contains(&account_id){
                true
            }else{
                false
            }
        }else{
            false
        }
    }

/*     pub fn remove_from_drop(&mut self, account_id: AccountId,ft_contract:AccountId){
        self.assert_owner();
        self.registered_members.get(&ft_contract).unwrap().remove(&account_id);
    } */

    pub fn get_apy(&self, ft_contract_id:AccountId)->Option<FT>{
        self.approved_fts.get(&ft_contract_id)
    }
}

#[cfg(test)]
mod staking_tests {

    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext,log,Gas};
    use std::convert::TryInto;

    const ALICE:&str = "alice.testnet";

    const FT_CONTRACT:&str = "ft.testnet";
    //const CURRENT_ACOUNT_ID: AccountId = "contract.testnet".to_string();
    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(AccountId::try_from(ALICE.to_string()).unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn test_contract_instantiation() {
        let context = get_context(false);

        let apy_dur:HashMap<APYKey, APY> = HashMap::new();

        let amount1:U128 = U128::from(500_000_000_000_000_0000_000_000_000);
        let amount2:U128 = U128::from(1000_000_000_000_000_0000_000_000_000);
        let amount3:U128 = U128::from(2000_000_000_000_000_0000_000_000_000);
        let duration1 : String = "3months".to_string();
        let duration2 : String = "6months".to_string();
        let duration3 : String = "12months".to_string();
        let apy1 : APY = APY{
            apy_key : duration1,
            min_staking_amount : amount1,
            min_duration : 3,
            interest_rate: 250,
        };
        let apy2 : APY = APY{
            apy_key : duration2,
            min_staking_amount : amount2,
            min_duration : 6,
            interest_rate: 500,
        };
        let apy3 : APY = APY{
            apy_key : duration3,
            min_staking_amount : amount3,
            min_duration : 12,
            interest_rate: 1000,
        };
        let mut fts : Vec<FT> = Vec::new();

        let ft = FT{
            account_id : AccountId::try_from(FT_CONTRACT.to_string()).unwrap(),
            symbol : "FT".to_string(),
            apy_against_duration: None
        };
        fts.push(ft);

        let mut apys : Vec<APY> = Vec::new();
        apys.push(apy1);
        apys.push(apy2);
        apys.push(apy3);

        let contract = Contract::new(AccountId::try_from(ALICE.to_string()).unwrap(),fts,apys);

        log!("{:?}", contract.approved_fts);
    }

    #[test]
    fn test_registered_members_insertion(){
        let context = get_context(false);
        testing_env!(context.clone());


        let amount1:U128 = U128::from(500_000_000_000_000_0000_000_000_000);
        let amount2:U128 = U128::from(1000_000_000_000_000_0000_000_000_000);
        let amount3:U128 = U128::from(2000_000_000_000_000_0000_000_000_000);
        let duration1 : String = "3months".to_string();
        let duration2 : String = "6months".to_string();
        let duration3 : String = "12months".to_string();
        let apy1 : APY = APY{
            apy_key : duration1,
            min_staking_amount : amount1,
            min_duration : 3,
            interest_rate: 250,
        };
        let apy2 : APY = APY{
            apy_key : duration2,
            min_staking_amount : amount2,
            min_duration : 6,
            interest_rate: 500,
        };
        let apy3 : APY = APY{
            apy_key : duration3,
            min_staking_amount : amount3,
            min_duration : 12,
            interest_rate: 1000,
        };
        let mut fts : Vec<FT> = Vec::new();

        let ft = FT{
            account_id : AccountId::try_from(FT_CONTRACT.to_string()).unwrap(),
            symbol : "FT".to_string(),
            apy_against_duration: None
        };
        fts.push(ft.clone());

        let mut apys : Vec<APY> = Vec::new();
        apys.push(apy1);
        apys.push(apy2);
        apys.push(apy3);

        let mut contract = Contract::new(AccountId::try_from(ALICE.to_string()).unwrap(),fts,apys);

        let mut registered_members : Vec<AccountId> = Vec::new();
        registered_members.push(AccountId::try_from(ALICE.clone().to_string()).unwrap());
        contract.registered_members.insert(&ft.account_id, &registered_members);

       // contract.all_registered_memebers(ft.account_id);

    }

    #[test]
    fn test_depositted_for_storage_member_insertion(){

        let context = get_context(false);
        testing_env!(context.clone());


        let amount1:U128 = U128::from(500_000_000_000_000_0000_000_000_000);
        let amount2:U128 = U128::from(1000_000_000_000_000_0000_000_000_000);
        let amount3:U128 = U128::from(2000_000_000_000_000_0000_000_000_000);
        let duration1 : String = "3months".to_string();
        let duration2 : String = "6months".to_string();
        let duration3 : String = "12months".to_string();
        let apy1 : APY = APY{
            apy_key : duration1,
            min_staking_amount : amount1,
            min_duration : 3,
            interest_rate: 250,
        };
        let apy2 : APY = APY{
            apy_key : duration2,
            min_staking_amount : amount2,
            min_duration : 6,
            interest_rate: 500,
        };
        let apy3 : APY = APY{
            apy_key : duration3,
            min_staking_amount : amount3,
            min_duration : 12,
            interest_rate: 1000,
        };
        let mut fts : Vec<FT> = Vec::new();

        let ft = FT{
            account_id : AccountId::try_from(FT_CONTRACT.to_string()).unwrap(),
            symbol : "FT".to_string(),
            apy_against_duration: None
        };
        fts.push(ft.clone());

        let mut apys : Vec<APY> = Vec::new();
        apys.push(apy1);
        apys.push(apy2);
        apys.push(apy3);

        let mut contract = Contract::new(AccountId::try_from(ALICE.to_string()).unwrap(),fts,apys);

        let mut members : UnorderedSet<AccountId> = UnorderedSet::new(b"s");
        members.insert(&AccountId::try_from(ALICE.clone().to_string()).unwrap());
        contract.depositted_for_storage.insert(&ft.account_id, &members);

    }

    #[test]
    fn test_staking_history_insertion(){
        let context = get_context(false);
        testing_env!(context.clone());


        let amount1:U128 = U128::from(500_000_000_000_000_0000_000_000_000);
        let amount2:U128 = U128::from(1000_000_000_000_000_0000_000_000_000);
        let amount3:U128 = U128::from(2000_000_000_000_000_0000_000_000_000);
        let duration1 : String = "3months".to_string();
        let duration2 : String = "6months".to_string();
        let duration3 : String = "12months".to_string();
        let apy1 : APY = APY{
            apy_key : duration1,
            min_staking_amount : amount1,
            min_duration : 3,
            interest_rate: 250,
        };
        let apy2 : APY = APY{
            apy_key : duration2,
            min_staking_amount : amount2,
            min_duration : 6,
            interest_rate: 500,
        };
        let apy3 : APY = APY{
            apy_key : duration3,
            min_staking_amount : amount3,
            min_duration : 12,
            interest_rate: 1000,
        };
        let mut fts : Vec<FT> = Vec::new();

        let ft = FT{
            account_id : AccountId::try_from(FT_CONTRACT.to_string()).unwrap(),
            symbol : "FT".to_string(),
            apy_against_duration: None
        };
        fts.push(ft.clone());

        let mut apys : Vec<APY> = Vec::new();
        apys.push(apy1.clone());
        apys.push(apy2.clone());
        apys.push(apy3.clone());

        let mut contract = get_contract();

        let mut staking_history : Vec<Stake> = Vec::new();
        let stake = Stake{
            stake_id : U128::from(1),
            ft_symbol : ft.symbol,
            ft_account_id : ft.account_id.clone(),
            decimal : 24,
            amount : U128::from(500000000000000000000000000),
            duration : 180,
            staked_at : env::block_timestamp()/1000000000,
            staked_by : context.signer_account_id.clone().try_into().unwrap(),
            staking_plan : apy1.clone().apy_key

        };

        staking_history.push(stake);

        contract.staking_history.insert(&ft.account_id, &staking_history);
    }

    fn get_contract()->Contract{
        let context = get_context(false);
        testing_env!(context.clone());


        let amount1:U128 = U128::from(500_000_000_000_000_0000_000_000_000);
        let amount2:U128 = U128::from(1000_000_000_000_000_0000_000_000_000);
        let amount3:U128 = U128::from(2000_000_000_000_000_0000_000_000_000);
        let duration1 : String = "3months".to_string();
        let duration2 : String = "6months".to_string();
        let duration3 : String = "12months".to_string();
        let apy1 : APY = APY{
            apy_key : duration1,
            min_staking_amount : amount1,
            min_duration : 3,
            interest_rate: 250,
        };
        let apy2 : APY = APY{
            apy_key : duration2,
            min_staking_amount : amount2,
            min_duration : 6,
            interest_rate: 500,
        };
        let apy3 : APY = APY{
            apy_key : duration3,
            min_staking_amount : amount3,
            min_duration : 12,
            interest_rate: 1000,
        };
        let mut fts : Vec<FT> = Vec::new();

        let ft = FT{
            account_id : AccountId::try_from(FT_CONTRACT.to_string()).unwrap(),
            symbol : "FT".to_string(),
            apy_against_duration: None
        };
        fts.push(ft.clone());

        let mut apys : Vec<APY> = Vec::new();
        apys.push(apy1.clone());
        apys.push(apy2.clone());
        apys.push(apy3.clone());

        let  contract = Contract::new(AccountId::try_from(ALICE.to_string()).unwrap(),fts,apys);
        contract
    }
    #[test]
    fn test_claim_history_insertion(){
        let mut contract = get_contract();

        let claim_history = ClaimHistory{
            last_claimed_at : env::block_timestamp(),
            claim_count  :1
        };

        contract.claim_history.insert(&U128::from(1), &claim_history);
    }
}
