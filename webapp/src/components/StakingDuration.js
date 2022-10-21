import { useState } from "react";
import { Button, Grid, Typography } from "@mui/material";
import StakingInformation from "./StakeForm";
import { NearContext } from '../provider/NearProvider';
import { useContext } from 'react';
import { parseNearAmount } from "near-api-js/lib/utils/format";

export default function StakingDuration() {
    const [duration,setDuration]=useState(0)
    const [tokens,setTokens]=useState(0)
const {stakeTokens,accountId} = useContext(NearContext)
    return (
        <>
        <Grid container xs={12} marginTop="20px">
            <Typography variant="h7" sx={{width : '100%'}}>Select Duration</Typography>

            <Grid item xs={12} marginTop="20px">
                <Grid container spacing={{xs:2}}>
                <Grid item xs={3} >
                <Button variant={duration===3 ? "contained" : 'text'} onClick={()=>setDuration(3)}>3 Minutes</Button>
            </Grid>
            <Grid item xs={3} >
                <Button variant={duration===6 ? "contained" : 'text'} onClick={()=>setDuration(6)}>6 Minutes</Button>
            </Grid>
            <Grid item xs={3} >
                <Button variant={duration===9 ? "contained" : 'text'} onClick={()=>setDuration(9)}>9 Minutes</Button>
            </Grid>
            <Grid item xs={3}>
                <Button variant={duration===12 ? "contained" : 'text'} onClick={()=>setDuration(12)}>12 Minutes</Button>
            </Grid>
                </Grid> 
            </Grid>

        </Grid>
        <Grid item xs={12} marginTop="20px" marginRight="16px">
                <StakingInformation tokens={tokens} setTokens={setTokens}/>
        </Grid>
        <Button onClick={()=>{
            const msg= {
                ft_symbol : "UNCT",
                ft_account_id:"ncd_ft_token.testnet",
                decimal : 24,
                duration: duration*60,
                staked_by : `${accountId}`,
                staking_plan: `${duration}minutes`
            }
            stakeTokens(parseNearAmount(tokens),JSON.stringify(msg))
        }} sx={{ margin: '10px' }}>Stake</Button>

        </>
    )
}