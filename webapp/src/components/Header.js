import * as React from 'react';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
// import {login} from '../provider/NearProvider'
import { NearContext } from '../provider/NearProvider';
import { useContext } from 'react';

export default function ButtonAppBar() {
const {login,isConnecting,accountId,isSignedIn,isRegistered,logout} = useContext(NearContext)
const handleLoginChange =()=>{
   login();
   /* let abc = isRegistered()
   console.log("response", abc) */
  }
  return (
    <Box sx={{ flexGrow: 1, width : '100%' }}>
      <AppBar position="static">
        <Toolbar>
         {/*  <IconButton
            size="large"
            edge="start"
            color="inherit"
            aria-label="menu"
            sx={{ mr: 2 }}
          >
            <MenuIcon />
          </IconButton> */}
          <Typography variant="h6" component="div" sx={{width : "210px"}}>
            UNCT Staking Portal
          </Typography>
          <Box sx={{ display : 'flex' ,justifyContent : 'flex-end' ,width: '100%'}}>
        {isSignedIn ?   <Typography component = "div" >{accountId}</Typography>: <Button color="inherit" sx={{display:"visible"}} onClick={handleLoginChange}>Connect Wallet</Button>}

          </Box>
          {isSignedIn && (
              <Button color="inherit" sx={{whiteSpace:'no-wrap',width:'10%'}} onClick={logout}>
                Log Out
              </Button>
        )}
        </Toolbar>
        {/* <Typography variant="h6" component="div" sx={{justifyContent : 'flex-end'}}>Balance</Typography> */}
      </AppBar>
    </Box>
  );
}
