
import './App.css';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Layout from "./pages/layout";
import HomePage from "./pages/homePage";
import NotFound from "./components/NotFound";


function App() {
 
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route path="/*" element={<NotFound />} />
          <Route index path="/" element={<HomePage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
