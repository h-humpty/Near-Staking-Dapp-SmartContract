import { useEffect,useState } from "react";
import { Grid,Typography,TableContainer,Table,TableBody,Paper } from "@mui/material"
import StakingData from "./TableBody";
import TableHeadings from "./TableHead";
import { NearContext } from '../provider/NearProvider';
import { useContext } from 'react';
export default function StakingHistory(){
    const {getStakingHistory,account} = useContext(NearContext)
    const [history,setHistory]=useState([])
    useEffect(()=>{
        const getHistory=async()=>{
            const historyResponse=await getStakingHistory('0',10)
            setHistory(historyResponse)
        }
        if(account){
            getHistory()
        }else{
            setHistory([])
        }
    },[account])
    return (
        <Grid container xs={12}>
        <Typography variant="h7" sx={{width:'100%'}}>Staking History</Typography>
        <Grid item xs={12} marginTop="10px">
            <TableContainer component={Paper} sx={{maxWidth : '90%'}}>
                <Table aria-label="simple table" >
                    <TableHeadings />
                    <TableBody>
                        {history?.map((stake)=>{
                            const{stake_id,amount,staked_at,staking_plan}=stake
                            //console.log("stake : ",stake)
                           return <StakingData key={stake_id} stakeId={stake_id} amount={amount} stakedAt={staked_at} stakingPlan = {staking_plan}/>
                        })}
                    </TableBody>
                </Table>
            </TableContainer>
        </Grid>
    </Grid>
    )
}