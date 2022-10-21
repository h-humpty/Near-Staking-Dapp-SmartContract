import { Grid, TableBody, Typography ,TableContainer,Table,Paper} from "@mui/material";
import { useContext, useEffect, useState } from "react";
import { NearContext } from "../provider/NearProvider";
import APYTableBody from "./APYTablebody";
import APYTableHead from "./APYTableHead";

export default function APYComponent(){
    const {getAPY,account} = useContext(NearContext)
    const [ftApy, setFtApy] = useState([])

    useEffect(()=>{
        const ftApys = async()=>{
            const response  = await getAPY()
            console.log("apys",response.apy_against_duration)
       let tempArr=[]
Object.values(response.apy_against_duration).map(item=>{
    tempArr=[...tempArr,item]
  
})
setFtApy(tempArr)
//console.log('tempArr', ftApy)

        }
        if (account){
            ftApys()
        }
    })
    return (
        <Grid container xs={12}>
            <Typography variant="h7" sx={{width:'100%'}}>APY Rates</Typography>
            <Grid item xs={12} marginTop="20px">
            <TableContainer component={Paper} sx={{maxWidth : '90%', marginTop :'10px'}}>
            <Table aria-label="simple table" size="small" >
                <APYTableHead/>
                <TableBody>
                    {ftApy?.map((apy)=>{
                        const {apy_key,interest_rate,min_duration,min_staking_amount} = apy
                        return  <APYTableBody key={apy_key} apyKey={apy_key} interestRate = {interest_rate} minDuration = {min_duration} minStakingAmount = {min_staking_amount}/>
                    })}
                </TableBody>
            </Table>
            </TableContainer>
            </Grid>
        </Grid>
    )
}