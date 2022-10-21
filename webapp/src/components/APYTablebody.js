import { TableCell, TableRow } from "@mui/material";
import { formatNearAmount, parseNearAmount } from "near-api-js/lib/utils/format";

export default function APYTableBody({apyKey,interestRate,minDuration,minStakingAmount}){
    const amount = formatNearAmount(minStakingAmount)
    const interest = (interestRate/10) + "%";
    const apy = minDuration +" "+"Minutes"
    return(
        <TableRow>
            <TableCell>
            {apy || "N/A"}
            </TableCell>
            <TableCell>
               {amount || "NA" }
            </TableCell>
            <TableCell>
                {minDuration || "NA"}
            </TableCell>
            <TableCell>
                {interest || "NA"}
            </TableCell>
        </TableRow>
    )
}