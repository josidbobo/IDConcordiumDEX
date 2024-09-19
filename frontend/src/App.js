
import './App.css';
import { BrowserRouter, Routes, Route } from "react-router-dom";

import DeFiLandingPage from './pages/landingpage';
import DexInterface  from './pages/dexInterface';


function App() {
 
  //<Route path='/' element={<Layout />}>
  return (
      <div>
        <BrowserRouter>
          <Routes>
            <Route path="/" element={<DeFiLandingPage />} />
            <Route path="/exchange" element={<DexInterface />} />
          </Routes>
      </BrowserRouter>
      </div>
  );
}

export default App;
