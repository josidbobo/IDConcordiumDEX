import {React, useState, useEffect} from 'react';
import { ArrowRight, DollarSign, TrendingUp, PieChart } from 'lucide-react';
import { ConcordiumGRPCClient, ContractAddress, getPastDate, MIN_DATE, Web3StatementBuilder, createConcordiumClient } from '@concordium/web-sdk';
//import { Link, NavLink } from "react-router-dom";
import { CONCORDIUM_NODE_PORT, CONNCORDIUM_NODE_ENDPOINT } from "../components/constants";
import { detectConcordiumProvider, WalletApi } from '@concordium/browser-wallet-api-helpers';
 
import { useNavigate } from 'react-router-dom';  

const DeFiLandingPage = () => {

    const [isVerified, setVerified] = useState(false);
    const [failed, setFailed] = useState(false);

    const navigate = useNavigate();  

  useEffect(() => {  
    // Add your condition here for navigating  
    if(isVerified == true){
    navigate('/exchange');  
    }
  }, [isVerified]); // Dependency array to prevent infinite loops  

  
    const { isConnected, setConnected } = useState(false);

  
      
async function connect() {
    await detectConcordiumProvider()
        .then((provider) => {
            provider.getMostRecentlySelectedAccount()
                .then((account) => (account ? Promise.resolve(account) : provider.connect()))
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
                 // Age verification is successful
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
    <div className="min-h-screen bg-gray-900 text-white">
      <header className="p-4 flex justify-between items-center">
        <div className="flex items-center space-x-2">
          <div className="w-8 h-8 bg-red-600 rounded-sm"></div>
          <span className="text-xl font-bold">LARRY <span className="text-white">MOSH</span></span>
        </div>
        <button onClick={connect} className="bg-red-600 text-white px-4 py-2 rounded">Connect Wallet</button>
      </header>

      <main className="container mx-auto px-4 flex flex-col md:flex-row items-center justify-between mt-16">
        <div className="md:w-1/2">
          <h1 className="text-4xl md:text-5xl font-bold mb-4">
            Decentralized
            <br />
            <span className="text-red-600">Lending</span> And <span className="text-red-600">Borrowing</span>
          </h1>
          <p className="text-gray-400 mb-8">
            The core purpose of this project is to showcase my adept understanding of
            lending and borrowing strategies in the context of blockchain technology.
          </p>
          <button className="bg-red-600 text-white px-6 py-3 rounded-full flex items-center">
            Go to Dashboard <ArrowRight className="ml-2" />
          </button>
        </div>

        <div className="md:w-1/2 mt-8 md:mt-0">
          <div className="relative">
            <div className="absolute top-0 right-0 w-64 h-64 bg-red-600 rounded-full filter blur-3xl opacity-20"></div>
            <img src="/animation.png" alt="DeFi Illustration" className="relative z-10" />
          </div>
        </div>
      </main>

      <div className="fixed bottom-4 right-4 flex space-x-2">
        <button className="w-10 h-10 bg-gray-800 rounded-full flex items-center justify-center">
          <PieChart size={20} />
        </button>
        <button className="w-10 h-10 bg-gray-800 rounded-full flex items-center justify-center">
          <TrendingUp size={20} />
        </button>
      </div>
    </div>
  );
};

export default DeFiLandingPage;