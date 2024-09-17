// import useWallet from "./hooks/useWallet";
import { useEffect, useState } from "react";
import { Outlet } from "react-router-dom";
import useWallet from "../components/useWallet";
import Header from "../components/Header";
import { ConcordiumGRPCClient, ContractAddress, getPastDate, MIN_DATE, Web3StatementBuilder, createConcordiumClient } from '@concordium/web-sdk';
import { Link, NavLink } from "react-router-dom";
import { CONCORDIUM_NODE_PORT, CONNCORDIUM_NODE_ENDPOINT } from "../components/constants";
import { detectConcordiumProvider, WalletApi } from '@concordium/browser-wallet-api-helpers';


function Layout() {
  const [isVerified, setVerified] = useState(false);
  const [failed, setFailed] = useState(false);


  const { isConnected, setConnected } = useState(false);

  useEffect(() => {
    if (isVerified) {
        document.body.style.backgroundColor = 'white';
    } else {
        document.body.style.backgroundColor = '#016a3a';
    }
}, [isVerified]);


//   useEffect(() => {
//     const loadDefi = async () => {
//       if (signerAddress) {
//         await loadUserSupplies(signerAddress);
//       }
//     };

//     loadDefi();
//   }, [signerAddress]);

  
//   const [state, setState] = useState<{
//     grpcClient: ConcordiumGRPCClient;
//     provider?: WalletApi;
//     account?: string;
//     marketplaceContractAddress?: ContractAddress;
// }>({
//     marketplaceContractAddress,
//     grpcClient: createConcordiumClient(CONNCORDIUM_NODE_ENDPOINT, Number(CONCORDIUM_NODE_PORT)),
// });

async function connect() {
    await detectConcordiumProvider()
        .then((provider) => {
            provider
                
                .then((account) => (account ? Promise.resolve(account) : WalletApi.connect()))
                .then(async (account) => {
                    await verifyUser();
                    //setState({ ...state, provider, account });
                })
                .catch(() => {
                    alert('Please allow wallet connection');
                });
            provider.on('accountDisconnected', () => {
               // setState({ ...state, account: undefined });
            });
            provider.on('accountChanged', (account) => {
                //setState({ ...state, account });
            });
            provider.on('chainChanged', () => {
                //setState({ ...state, account: undefined, provider: undefined });
            });
            
        })
        .catch(() => {
            console.error(`could not find provider`);
            alert('Please download Concordium Wallet');
        });
}


  
  async function verifyUser() {
    const provider = await detectConcordiumProvider();
    try {
        await provider.requestAccounts();
        const statementBuilder = new Web3StatementBuilder().addForIdentityCredentials([0, 1, 2, 3, 4, 5], (b) =>
            b.addRange('dob', MIN_DATE, getPastDate(18, 1))
        );
        const statement = statementBuilder.getStatements();
        const challenge = 'BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB';

        provider.requestVerifiablePresentation(challenge, statement)
            .then(() => {
                setVerified(true);
                setFailed(false);
                 // Assuming age verification is successful
            })
            .catch(() => {
                setFailed(true);
            });
    } catch (error) {
        console.error(error);
        alert('Please connect');
    }
}


  return (
    <div className="bg-gray-800 h-full dark:bg-dark-100 flex min-h-screen flex-col">
      <div className="">
        <Header getAccount={connect} />
      </div>
      <Outlet />
    </div>
  );
}

export default Layout;