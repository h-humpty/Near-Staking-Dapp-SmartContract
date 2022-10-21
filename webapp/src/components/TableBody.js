import { Button, TableCell,TableRow } from "@mui/material"
import { formatNearAmount } from "near-api-js/lib/utils/format";
import { NearContext } from '../provider/NearProvider';
import { useContext } from 'react';
export default function StakingData({stakeId,amount,stakedAt,stakingPlan}){
    const {claimReward,unstakeTokens} = useContext(NearContext)
    const options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' }
    const date=new Date(stakedAt*1000).toLocaleDateString("en-US", options)

    const RewardClaimer=async()=>{
        try {
            await claimReward(stakeId)
        window.location.reload()

        } catch (error) {
            alert("Reward can be claimed after staking for 1 minute")
        }
    }

    const TokenUnStaker=async()=>{
        try {
            await unstakeTokens(stakeId)
        window.location.reload()

        } catch (error) {
            alert("Cannot withdraw before locked time")
        }
    }

    return (
        <TableRow>
        <TableCell>
            {stakeId || "N/A"}
        </TableCell>
        <TableCell>
            {formatNearAmount(amount) || "N/A"}
        </TableCell>
        <TableCell>
            {date || "N/A"}
        </TableCell>
        <TableCell>
            {stakingPlan || "N/A"}
        </TableCell>
        <TableCell>
            <Button onClick={()=>RewardClaimer()}>Claim Reward</Button>
            <Button onClick={TokenUnStaker}>Unstake</Button>
        </TableCell>
    </TableRow>
    
    )
}