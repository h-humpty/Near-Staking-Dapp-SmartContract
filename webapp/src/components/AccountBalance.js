import { useEffect,useState } from "react";
import {  Grid, TextField, Typography , Button } from "@mui/material";
import { NearContext } from '../provider/NearProvider';
import { useContext } from 'react';
import { formatNearAmount } from "near-api-js/lib/utils/format";

export default function AccountBalance(){
    const [accountBalance,setAccountBalance]=useState(0)
    const { account, getFTBalance, getAirdrop } = useContext(NearContext)
    useEffect(() => {
        const getBalance = async () => {
            let balance=await getFTBalance()
            //balance = parseInt(balance)
            setAccountBalance(formatNearAmount(balance))
        }
        if(account){
            getBalance()
        }
    },[account,getFTBalance])
    return (
       
        <Grid container spacing={2}>
            <Grid item >
                <Typography variant="h7">Account Balance :</Typography>
            </Grid>
            <Grid item xs={3} container spacing={2}>
                {!account ? (
                
                <Grid item xs={6}>
                <Typography variant="h8">Connect wallet</Typography>
                </Grid>
                ) : (
                <>
                <Grid item xs={6}>
                {/* <Typography variant="h8">{accountBalance ?  parseFloat(accountBalance.replace(/\,/g,''), 10).toFixed(2) : "0"} UNCT</Typography> */}
                <Typography variant="h8">{typeof(accountBalance)==="string" ? parseFloat(accountBalance.replace(/\,/g,''), 10).toFixed(2) : typeof(accountBalance)==="number" ? accountBalance.toFixed(2):"0" } UNCT</Typography>
                                
                </Grid>
                <Grid item xs={6}>
                                <Button variant="outlined" onClick={() => {
                                    getAirdrop()
                }} type="primary">Get UNCT</Button>

                </Grid>
                        </>        
                )}
            </Grid>
        </Grid>
 
        )
}
    
