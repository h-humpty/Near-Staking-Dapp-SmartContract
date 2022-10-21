use crate::*;


impl Contract{
    pub(crate) fn add_fts(
        approved_ft_token_ids: Vec<FT>,
        set: &mut LookupMap<AccountId, FT>,
        ft_accounts: &mut UnorderedSet<AccountId>,
        ft_apy: Vec<APY>,
    ) {
        for mut ft in approved_ft_token_ids {
            let  apy_map: HashMap<APYKey, APY> = Contract::insert_apy(ft_apy.clone());
            ft.apy_against_duration = Some(apy_map);
            set.insert(&ft.account_id, &ft);
            ft_accounts.insert(&ft.account_id);
        }
    }

    pub(crate) fn insert_apy(ft_apy: Vec<APY>)-> HashMap<APYKey,APY>{
        let mut temp :HashMap<APYKey,APY> = HashMap::new();

        for apy in &ft_apy {
            // let apy_key = ft.symbol.clone() + &apy.plan_name+&apy.min_duration.to_string();
            temp.insert(apy.apy_key.clone(), apy.clone());
        }
        temp
    }

    pub(crate) fn is_owner(&self) -> bool {
        &env::predecessor_account_id() == &self.owner_id
    }
    pub(crate) fn assert_owner(&self) {
        assert!(self.is_owner(), "Owner's method");
    }
}
