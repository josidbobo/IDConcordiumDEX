import React, { useState, useEffect } from 'react';
import { AlertCircle, ArrowRightLeft, DollarSign, Coins } from 'lucide-react';

// Custom Alert Component
const Alert = ({ type, message, onClose }) => {
  const bgColor = type === 'error' ? 'bg-red-100' : 'bg-green-100';
  const textColor = type === 'error' ? 'text-red-800' : 'text-green-800';
  const iconColor = type === 'error' ? 'text-red-500' : 'text-green-500';

  return (
    <div className={`rounded-md p-4 ${bgColor} ${textColor} mb-4`}>
      <div className="flex">
        <div className="flex-shrink-0">
          <AlertCircle className={`h-5 w-5 ${iconColor}`} />
        </div>
        <div className="ml-3">
          <p className="text-sm font-medium">{message}</p>
        </div>
        <div className="ml-auto pl-3">
          <div className="-mx-1.5 -my-1.5">
            <button
              onClick={onClose}
              className={`inline-flex rounded-md p-1.5 ${bgColor} ${textColor} hover:${bgColor} focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-${type === 'error' ? 'red' : 'green'}-50 focus:ring-${type === 'error' ? 'red' : 'green'}-600`}
            >
              <span className="sr-only">Dismiss</span>
              <svg className="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                <path fillRule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clipRule="evenodd" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

// functions to simulate blockchain interactions
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

  const closeNotification = () => {
    setNotification(null);
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
        <Alert
          type={notification.type}
          message={notification.message}
          onClose={closeNotification}
        />
      )}
    </div>
  );
};

export default DexInterface;