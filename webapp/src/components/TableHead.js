import { TableHead, TableRow, TableCell, Typography } from "@mui/material"
export default function TableHeadings() {
    return (
        <TableHead>
            <TableRow>
                <TableCell>
                    <Typography variant="h7">
                        Stake ID
                    </Typography>
                </TableCell>
                <TableCell alig>
                    <Typography variant="h7">
                        Amount Staked
                    </Typography>
                </TableCell>
                <TableCell>
                    <Typography variant="h7">Staked On</Typography>

                </TableCell>
                <TableCell>
                    <Typography variant="h7">Staked For</Typography>

                </TableCell>
                <TableCell>
                    <Typography variant="h7">
                        Actions
                    </Typography>
                </TableCell>
            </TableRow>
        </TableHead>
    )
}