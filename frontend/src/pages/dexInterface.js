import React, { useState, useEffect } from 'react';
import { AlertCircle, ArrowRightLeft, DollarSign, Coins } from 'lucide-react';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';

// Mock functions to simulate blockchain interactions
const mockGetTokenPrice = () => Promise.resolve(0.1);
const mockGetContractLiquidity = () => Promise.resolve(1000000);
const mockBuyTokens = (amount) => Promise.resolve({ success: true, message: `Bought ${amount} tokens` });
const mockSellTokens = (amount) => Promise.resolve({ success: true, message: `Sold ${amount} tokens` });

const DexInterface = () => {
  const [tokenPrice, setTokenPrice] = useState(0);
  const [contractLiquidity, setContractLiquidity] = useState(0);
  const [amount, setAmount] = useState('');
  const [isBuying, setIsBuying] = useState(true);
  const [notification, setNotification] = useState(null);

  useEffect(() => {
    const fetchData = async () => {
      const price = await mockGetTokenPrice();
      const liquidity = await mockGetContractLiquidity();
      setTokenPrice(price);
      setContractLiquidity(liquidity);
    };
    fetchData();
    // In a real implementation, you might want to set up an interval to periodically fetch updated data
  }, []);

  const handleAmountChange = (e) => {
    setAmount(e.target.value);
  };

  const handleToggleMode = () => {
    setIsBuying(!isBuying);
  };

  const handleTransaction = async () => {
    try {
      const result = isBuying
        ? await mockBuyTokens(amount)
        : await mockSellTokens(amount);
      
      setNotification({ type: 'success', message: result.message });
      setAmount('');
      // In a real implementation, you would update the contract liquidity and possibly the token price here
    } catch (error) {
      setNotification({ type: 'error', message: error.message });
    }
  };

  return (
    <div className="max-w-md mx-auto mt-10 p-6 bg-white rounded-lg shadow-lg">
      <h2 className="text-2xl font-bold mb-6 text-center">TechFiesta DEX</h2>
      
      <div className="flex justify-between items-center mb-6">
        <div className="flex items-center">
          <DollarSign className="mr-2 text-blue-500" />
          <span className="font-semibold">Token Price:</span>
        </div>
        <span>{tokenPrice.toFixed(4)} CCD</span>
      </div>
      
      <div className="flex justify-between items-center mb-6">
        <div className="flex items-center">
          <Coins className="mr-2 text-green-500" />
          <span className="font-semibold">Contract Liquidity:</span>
        </div>
        <span>{contractLiquidity.toLocaleString()} Tokens</span>
      </div>
      
      <div className="mb-4">
        <label htmlFor="amount" className="block text-sm font-medium text-gray-700 mb-2">
          Amount of {isBuying ? 'CCD to spend' : 'Tokens to sell'}
        </label>
        <input
          type="number"
          id="amount"
          value={amount}
          onChange={handleAmountChange}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Enter amount"
        />
      </div>
      
      <button
        onClick={handleToggleMode}
        className="w-full mb-4 flex justify-center items-center px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-500"
      >
        <ArrowRightLeft className="mr-2" />
        Switch to {isBuying ? 'Sell' : 'Buy'}
      </button>
      
      <button
        onClick={handleTransaction}
        className="w-full px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        {isBuying ? 'Buy Tokens' : 'Sell Tokens'}
      </button>
      
      {notification && (
        <Alert className={`mt-4 ${notification.type === 'error' ? 'bg-red-100' : 'bg-green-100'}`}>
          <AlertCircle className={notification.type === 'error' ? 'text-red-500' : 'text-green-500'} />
          <AlertTitle>{notification.type === 'error' ? 'Error' : 'Success'}</AlertTitle>
          <AlertDescription>{notification.message}</AlertDescription>
        </Alert>
      )}
    </div>
  );
};

export default DexInterface;
