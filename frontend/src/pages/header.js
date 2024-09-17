
import { useEffect, useState, useCallback } from "react";
import { displayToast } from "./Toast";
import logo from "../../public/concordium-logo.png";
// import { useRouter } from 'next/router';

import useWallet from "../hooks/useWallet";
import Sidebar2 from "./Sidebar2";

//   useEffect(() => {
//     const media = window.matchMedia(`(max-width: ${width}px)`);
//     media.addEventListener("change", updateTarget);

//     // Check on mount (callback is not called until a change occurs)
//     if (media.matches) {
//       setTargetReached(true);
//     }

//     return () => media.removeEventListener("change", updateTarget);
//   }, []);

export default function Header(props) {
    const {getAccount} = props;

  const [collapsed, setCollapsed] = useState(true);

  const [showSidebar, setShowSidebar] = useState(false);

  const { chainId } = useWallet();

  useEffect(() => {
    console.log("There is a change in the id", chainId);
  }, [chainId]);

  useEffect(() => {
    console.log("Collapsing: ", collapsed);
  }, [collapsed]);


  return (
    <header className="App-header">
            <div className="leftH">
                <img src={logo} alt='Logo' className='logo'/>
                <div className="headerItem">RAGNAR</div>
                <div className="text-red-800 font-medium">DEX</div>
            </div>
            <div className='rightH'>
                <div className="connectButton" onClick={getAccount}>
                    Connect
                </div>
            </div>
        </header>
  );
}