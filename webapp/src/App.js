import './App.css';
import Header from './components/Header';
import StakingComponent from './components/StakingComponent';
import NearProvider from './provider/NearProvider';


function App() {
  return (
    <div className="App">
    <NearProvider>
        <Header/>
      <StakingComponent/>
      </NearProvider>
      
    </div>
  );
}

export default App;
