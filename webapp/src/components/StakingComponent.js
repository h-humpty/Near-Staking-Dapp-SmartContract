import { Grid, Paper } from "@mui/material";
import AccountBalance from "./AccountBalance";
import APYComponent from "./APYComponent";
import StakingDuration from "./StakingDuration";
import StakingHistory from "./StakingHistory";




export default function StakingForm() {
    return (
        <Paper sx={{ marginTop: '10px' }} elevation={0}>
            <Grid container marginLeft="8px" marginRight="8px">
                {/* Balance item start */}
                <Grid item xs={12} marginTop="10px" >
                    <AccountBalance />
                </Grid>
                {/* Balance item End */}

                <Grid item xs={12} marginTop="10px" marginLeft="10px" marginRight="10px">
                    <APYComponent marginTop="10px"/>
                </Grid>

                {/* Duration Starts */}
                <Grid item xs={12} marginTop="20px">
                    <StakingDuration />
                </Grid>
                {/* Duration Ends */}

                {/* Textfield Start */}

                <Grid item xs={12}>
                   <StakingHistory/>
                </Grid>
            </Grid>
        </Paper>
    )

}
