import {  Paper, TextField } from "@mui/material";
export default function StakingInformation({tokens,setTokens}){
    return(
        
        <Paper component='form' elevation={0}>
            <TextField onChange={(e)=>setTokens(e.target.value)} fullWidth label="Tokens to Stake" id="fullWidth" sx={{maxWidth : '50%'}}/>
        </Paper>
    )
}