import { FaChevronRight } from "react-icons/fa";
import YourBorrows from "../components/YourBorrows";
import useWallet from "../hooks/useWallet";
import MultiStepProgressBar from "../components/MultiStepProgressBar";
import Footer from "../components/Footer";
import { Link } from "react-router-dom";
import Header from "../components/Header";
import "animate.css";

export default function HomePage() {
  const { signerAddress } = useWallet();

  console.log("Signer address: ", signerAddress);

  return (
    <>
      <section className="bg-gray-900">
        {/* <Header className="bg-gradient-to-r from-black via-black via-black to-black"/> */}
        <section className="w-full flex-1 md:h-screen">
          <div className="bg-gradient-to-tr from-[#0D1321] via-[#0D1321] to-red-950 pt-24 md:pt-16 w-full h-full px-4 flex flex-col items-center justify-center md:grid md:grid-cols-12 text-white">
            <div className="flex flex-col md:mt-0 items-start px-4 ssm:px-8 justify-center md:col-span-6 space-y-4">
              <div className="animate__animated animate__backInDown  text-gray-300 text-xl ss:text-2xl ssm:text-4xl xl:text-5xl font-medium ">
                <p>Decentralized</p>
                <p>
                  <span className="text-red-800">Exchange</span> on{" "}
                  <span className="text-red-800">Concordium</span>
                </p>
              </div>
              <p className="text-gray-300 text-sm ssm:text-lg lg:text-xl leading-relaxed">
                This project aims to showcase a proof-of-concept on Decentralised Exchange service on the Concordium Blockchain
                it's also to refine my undestanding of DEX strategies using CIS2 tokens in the context
                of blockchain technology.
                {/* A system that enables individuals to exchange digital
                assets without intermediaries offering greater accessibility,
                efficiency, and security. */}
              </p>

              <button
                onClick={() => console.log("Connect Wallet")}
                className="flex items-center space-x-2 ssm:space-x-4 bg-red-800 rounded-full hover:border-orange-900 py-3 px-4 ssm:px-8 text-sm ssm:text-base text-white "
              >
                <p>Connect Wallet</p>
                <FaChevronRight className="text-orange-800 w-5 h-5 bg-white rounded-full p-1" />
              </button>
            </div>
            <div className="md:col-span-6 ">
              <img
                src="./animation.png"
                className="object-cover w-full w-12/12 md:w-10/12"
              />
            </div>
          </div>
        </section>
      </section>
      <section className="bg-gray-900 h-full w-full flex flex-col  text-white py-8  items-center justify-center">
        <h1 className="text-center text-4xl sm:text-5xl text-gray-400 pb-5">
          About the App
        </h1>
        <p className="w-9/12 sm:w-8/12 text-gray-400 text-sm sm:text-base text-center leading-relaxed">
        The application functions as a decentralized exchange, offering a platform for users to 
        engage in the trading of digital assets without the need for a central authority or intermediary. 
        It leverages a peer-to-peer network, allowing participants to directly facilitate trades and exchanges. 
        This decentralized structure empowers users by enabling secure, transparent, and trustless transactions, all maintained by the collective 
        actions of the network’s participants.
          <p className="py-4">
          The development of this decentralized exchange drew heavily on insights and 
          guidance from the Concordium Development Libraries.
          </p>
        </p>
      </section>
      <section className="bg-gray-900 h-full w-full text-white py-8  items-center justify-center">
        <h1 className="text-center text-4xl sm:text-5xl text-gray-400">
          How It Works
        </h1>
        <MultiStepProgressBar />
      </section>
      <Footer />
    </>
  );
}