import { createContext, useCallback, useEffect, useState } from "react";
import { connect, keyStores, WalletConnection, utils, Contract } from "near-api-js";
import { parseNearAmount } from "near-api-js/lib/utils/format";
/* import NFT from "../models/NFT";
import Purchasable from "../models/Purchasable";
import viewToMax from "../utils/viewToMax"; */


// --- Production: mainnet ---
// const config = {
//   networkId: "mainnet",
//   contractName: "mtvrs-app.near",
//   nodeUrl: "https://rpc.mainnet.near.org",
//   walletUrl: "https://wallet.mainnet.near.org",
//   helperUrl: "https://helper.mainnet.near.org",
//   explorerUrl: "https://explorer.mainnet.near.org",
// };

// --- Development: testnet ---
const config = {
  networkId: "testnet",
  contractName: "ncd_staking_contract.testnet",
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://explorer.testnet.near.org",
};

const GAS = "300000000000000"; // max gas for any transaction

export const NearContext = createContext({});

export default function NearProvider({ children }) {
  // Internal state
  const [, setNearConnection] = useState(null);
  const [walletConnection, setWalletConnection] = useState(null);
  const [accountId, setAccountId] = useState(null);

  // External state.
  const [isConnecting, setIsConnecting] = useState(true);
  const [isSignedIn, setIsSignedIn] = useState(false);
  const [isError, setIsError] = useState(false);
  const [isLoggingIn] = useState(false);
  const [account, setAccount] = useState(null);
  const [accountNearBalance, setAccountNearBalance] = useState(null);
  const [accountUsername, setAccountUsername] = useState(null);
  const [regInfo, setregInfo] = useState({})

  const loginKey = 'loginKey';



  useEffect(() => {
    const keyStore = new keyStores.BrowserLocalStorageKeyStore();
    const connectNear = async () => {
      try {
        const near = await connect({ ...config, keyStore });
        const walletConnection = new WalletConnection(near);
        setNearConnection(near);
        setWalletConnection(walletConnection);

        if (walletConnection.isSignedIn()) {
          const account = walletConnection.account();
          const name = walletConnection._authData.accountId;
          setAccountId(walletConnection.getAccountId());
          setIsSignedIn(true)
          setAccount(account)
          /*     let abc = isRegistered()
       console.log("response", abc) */
        } else {
          setIsSignedIn(false)

        }
      } catch (e) {
        console.error(e);
        setIsError(true);
      }
      setIsConnecting(false);
    };

    connectNear();
  }, []);

  useEffect(() => {
    if (account != null) {
      hasDeposittedForStorage()

    }

  }, [account])

  const getAccountAndNearBalance = useCallback(async () => {
    if (account == null) {
      setAccountNearBalance(null);
      return;
    }

    let balance;
    try {
      balance = await account.getAccountBalance();
    } catch (e) {
      console.error(e);
      setAccountNearBalance(null);
      return;
    }

    setAccountNearBalance(utils.format.formatNearAmount(balance.available, 2));
  }, [account]);

  useEffect(() => {
    // Fetch the near balance.
    getAccountAndNearBalance();
  }, [getAccountAndNearBalance]);

  const login = useCallback(() => {
    // console.log ("near amount", parseNearAmount("0.00859"))
    walletConnection?.requestSignIn({
      contractId: config.contractName,

    }
    );

  }, [walletConnection]);

  const logout = useCallback(() => {
    walletConnection?.signOut();
    setIsSignedIn(false);
    setAccountId(null);
    setAccount(null);
  }, [walletConnection]);



  /*   const getNftsForAccount = useCallback(async () => {
      if (account == null || accountId == null) {
        throw new Error("account must be defined");
      }
  
      const tokens = await viewToMax(
        account,
        config.contractName,
        "nft_tokens_for_owner",
        { account_id: accountId }
      );
  
      return (tokens ?? []).map(NFT.fromNear);
    }, [account, accountId]); */

  /*   const getMetadataForToken = useCallback(
      async (tokenType) => {
        if (account == null) {
          throw new Error("account must be defined");
        }
  
        const token = await account.viewFunction(
          config.contractName,
          "metadata_get",
          {
            token_type: tokenType,
          }
        );
  
        if (token == null) {
          return null;
        }
  
        const tokenList = await viewToMax(
          account,
          config.contractName,
          "minted_tokens_list",
          { token_type: tokenType },
          "offset"
        );
  
        const tokenMaxSupply = parseInt(token.copies);
        const numOfMintedTokens = tokenList?.length ?? 0;
  
        const isAvailable =
          !isNaN(tokenMaxSupply) && numOfMintedTokens < tokenMaxSupply;
  
        return Purchasable.fromNear(tokenType, token, isAvailable);
      },
      [account]
    ); */

  /*   const purchaseNFT = useCallback(
      async (tokenId, quantity) => {
        if (account == null) {
          throw new Error("Account must be defined");
        }
  
        return account.functionCall({
          contractId: config.contractName,
          methodName: "metamon_purchase",
          args: {
            token_type: tokenId,
          },
          gas: GAS,
          // TODO - Get price from master ali.
          attachedDeposit: parseNearAmount("10"),
        });
      },
      [account]
    ); */

  
  const isRegistered = useCallback(
    async () => {
      console.log("account_id and contract", account.accountId + " " + config.contractName)
      if (account == null) {
        throw new Error("Account must be defined");
      }
      return account.viewFunction({
        contractId: config.contractName,
        methodName: "is_registered",
        args: {
          ft_contract_id: "ncd_ft_token.testnet",
          account_id: account.accountId
        }
      });
      console.log('regInfo****', regInfo)
    },
    [account]
    // console.log("account : ",account)
  );

  const hasDeposittedForStorage = useCallback(
    async () => {
      let token;
      if (account != null) {
        token = await account?.viewFunction(
          config.contractName,
          "has_depositted_for_storage",
          {
            ft_contract_id: "ncd_ft_token.testnet",
            account_id: account.accountId
          }
        );
        console.log("has_depositted_for_storage", token)
        if (token === false) {
          depositForStorage()
        }

      }
      setregInfo(token)

    },
    [account]
  );

  const getStakingHistory = useCallback(
    async (fromIndex,limit)=>{
      if (account == null) {
        throw new Error("Account must be defined");
      }
      let history= await account?.viewFunction(
        config.contractName, "get_staking_history",
        {
          account_id : account.accountId,
          from_index : fromIndex,
          limit : limit
        }
        
        );
        return history
    },[account]
    )
    const getAPY =useCallback(
      
      async()=>{
        if (account == null) {
          throw new Error("Account must be defined");
        }
        const res = await account?.viewFunction(
          config.contractName, "get_apy",
          {
            ft_contract_id : "ncd_ft_token.testnet"
          }
        );
        return res
      },[account]
    )



  const depositForStorage = useCallback(
    async () => {
      if (account == null) {
        throw new Error("Account must be defined");
      }
      try {
        const resp = await account.functionCall({
          contractId: config.contractName,
          methodName: "deposit_for_storage",
          args: {
            ft_contract_id: "ncd_ft_token.testnet"
          }, gas: GAS,
          attachedDeposit: parseNearAmount("0.00859")
        })
        console.log('resp*****', resp)
      } catch (error) {
        console.log('error****', error)
      }

    }, [account]
  );

  const claimReward = useCallback(
    async (stakeId) => {
      if (account == null) {
        throw new Error("Account must be defined");
      }
      return account.functionCall({
        contractId: config.contractName,
        methodName: "claim_reward",
        args: {
          stake_id: stakeId
        },
        gas: GAS
      })
    }, [account]
  )

  const unstakeTokens = useCallback(
    async (stakeId) => {
      if (account == null) {
        throw new Error("Account must be defined");
      }
      return account.functionCall({
        contractId: config.contractName,
        methodName: "ft_unstake",
        args: {
          stake_id: stakeId
        }, gas: GAS
      })
    }, [account]
  )

  const getFTBalance = useCallback(
    async()=>{
      if (account === null){
        throw new Error("Account must be defined");
      }
      const ftContract =  new Contract(account,"ncd_ft_token.testnet",{
        viewMethods : ["ft_balance_of"],
        sender : account.accountId
      })
      let balance;
      balance = await ftContract?.ft_balance_of(
        {account_id : account.accountId}
      );
      return balance
    },[account]
  )

  const getAirdrop =useCallback(
    async()=>{
      try {
        if (account == null){
          throw new Error("Account must be defined");
        }
        const stakingContract =  new Contract(account,"ncd_staking_contract.testnet",{
          changeMethods : ["drop_ft"], sender : account.accountId
        })
        let response;
        response = await stakingContract?.drop_ft(
          {account_id : account.accountId, ft_contract_id : "ncd_ft_token.testnet"}
        );
        window.location.reload()
        return response
      } catch (error) {
        alert("Already claimed drop")
      }
    },[account]
  )

  const stakeTokens = useCallback(
    async(amt,message)=>{
      if (account == null){
        throw new Error("Account must be defined");
      }
      const res = await account.functionCall(
        {
          contractId : "ncd_ft_token.testnet",
          methodName : "ft_transfer_call",
          args:{
            receiver_id : "ncd_staking_contract.testnet",
            amount : amt,
            msg: message
          },gas : GAS,attachedDeposit: utils.format.parseNearAmount(
            "0.000000000000000000000001"
          )
        }
      )

  /*     const ftContract = new Contract(account, "ncd_ft_token.testnet", {
        changeMethods : ["ft_transfer_call"], sender : account.accountId
      })
      let response = await ftContract?.ft_transfer_call(
        {receiver_id : "ncd_staking_contract.testnet",amount: amt,msg : message,attachedDeposit: parseNearAmount("0.000000000000000000000001") }
      );
      return response */
      return res
    },[account]
  )

 /*  const res = await account.functionCall({
    contractId: contractName,
    methodName: change_methods.STAKE_TOKENS,
    args: {
      receiver_id: "staking_bkrt.testnet",
      amount: amount,
      msg: JSON.stringify({
        ft_symbol: "BKRT",
        ft_account_id: "ft_bkrt.testnet",
        decimal: 24,
        duration: duration,
        staked_by: walletAccountId,
        staking_plan: "BKRTPremium6",
      }),
    },

    gas: digits.GAS,
    // // TODO - Get price from master ali.
    attachedDeposit: utils.format.parseNearAmount(
      "0.000000000000000000000001"
    ),
  }); */

 


  /*   const stakeTokens = useCallback(
      async ()=>{
        if(account==null){
          throw new Error("Account must be defined");
        }
        return account.functionCall({
          contractId : ft_contract_id,
          methodName : "ft_transfer_call",
          args : {
            receiver_id : config.contractName,
  
          }
        })
      },[account]
    ) */

  const redeemSurprisePack = useCallback(
    (tokenId) => {
      if (account == null) {
        throw new Error("Account Must be defined");
      }

      return account.functionCall({
        contractId: config.contractName,
        methodName: "metamon_redeem_surprise_pack",
        args: {
          token_id: tokenId,
        },
        gas: GAS,
      });
    },
    [account]
  );


  const evolveToNextGeneration = useCallback(
    (firstTokenId, secondTokenId) => {
      if (
        firstTokenId == null ||
        secondTokenId == null ||
        firstTokenId === "" ||
        secondTokenId === "" ||
        firstTokenId === secondTokenId
      ) {
        throw new Error("Tokens must be unique and defined");
      }

      if (account == null) {
        throw new Error("Accoutn msut be defined");
      }

      return account.functionCall({
        contractId: config.contractName,
        methodName: "metamon_evolve",
        args: {
          token1_id: firstTokenId,
          token2_id: secondTokenId,
        },
        gas: GAS,
      });
    },
    [account]
  );
  return (
    
    <NearContext.Provider
      value={{
        accountUsername,
        isConnecting,
        isLoggingIn,
        isError,
        isSignedIn,
        login,
        logout,
        depositForStorage,
        isRegistered,
        accountNearBalance,
        getAccountAndNearBalance,
        redeemSurprisePack,
        evolveToNextGeneration,
        accountId,
        account,
        getFTBalance,
        getAirdrop,
        stakeTokens,
        getStakingHistory,
        unstakeTokens,
        claimReward,
        getAPY
      }}
    >
      {children}
    </NearContext.Provider>
  );
}