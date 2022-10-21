import { TableHead, TableRow, TableCell, Typography } from "@mui/material"
export default function APYTableHead(){

    return(
        <TableHead>
        <TableRow>
            <TableCell>
                <Typography>Staking Plan</Typography>
            </TableCell>
            <TableCell>
                <Typography>Min Tokens</Typography>
            </TableCell>
            <TableCell>
                <Typography>Lock Period (Mins)</Typography>
            </TableCell>
            <TableCell>
                <Typography>APY</Typography>
            </TableCell>
        </TableRow>
    </TableHead>
    )

 
}