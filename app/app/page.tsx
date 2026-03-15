'use client';

import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { SystemProgram, Transaction, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { useEffect, useState } from 'react';
import { getBalance, requestWithdraw, claimWithdraw, getProtocolConfig } from '../lib/api';
import WalletButton from '../components/WalletButton';


interface StakeInfo {
  stake_account: string;
  amount_sol: number;
  staked_at: string;
}

interface BalanceData {
  wallet_balance_sol: number;
  total_deposited: number;
  total_withdrawn: number;
  active_stakes: StakeInfo[];
  total_staked_sol: number;
  estimated_yield_sol: number;
}

export default function Home() {
  const { publicKey,connected, sendTransaction } = useWallet();
  const { connection } = useConnection();
  const [balanceData, setBalanceData] = useState<BalanceData | null>(null);
  const [depositAmount, setDepositAmount] = useState('');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  useEffect(() => {
    if (connected && publicKey) {
      fetchBalance();
    }
  }, [connected, publicKey]);

  async function fetchBalance() {
    if (!publicKey) return;
    const data = await getBalance(publicKey.toString());
    setBalanceData(data);
  }


  async function handleDeposit() {
    if (!publicKey || !depositAmount) return;
    setLoading(true);
    setMessage('');
    try {
         
      const config = await getProtocolConfig()
        const PROTOCOL_WALLET = new PublicKey(config.protocol_wallet);
  
        const lamports = parseFloat(depositAmount) * LAMPORTS_PER_SOL;

        // build transfer transaction
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: publicKey,
                toPubkey: PROTOCOL_WALLET,
                lamports,
            })
        );

        const { blockhash } = await connection.getLatestBlockhash();
        transaction.recentBlockhash = blockhash;
        transaction.feePayer = publicKey;

        // user signs with their wallet (Phantom popup)
        const signed = await sendTransaction(transaction, connection);

        // confirm on chain
        await connection.confirmTransaction(signed, 'confirmed');

        // tell backend to record and stake
        const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/deposit`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                from_pubkey: publicKey.toString(),
                amount_sol: parseFloat(depositAmount),
                signature: signed,
            }),
        });

        const data = await res.json();
        setMessage(data.message);
        fetchBalance();
    } catch (e: any) {
        setMessage(e.message || 'Deposit failed');
    }
    setLoading(false);
}
  async function handleWithdraw() {
    if (!publicKey) return;
    setLoading(true);
    setMessage('');
    try {
      const res = await requestWithdraw(publicKey.toString());
      setMessage(res.message);
      fetchBalance();
    } catch (e) {
      setMessage('Withdraw request failed');
    }
    setLoading(false);
  }

  async function handleClaim(stakeAccount: string) {
    if (!publicKey) return;
    setLoading(true);
    setMessage('');
    try {
      const res = await claimWithdraw(publicKey.toString(), stakeAccount);
      setMessage(res.message);
      fetchBalance();
    } catch (e) {
      setMessage('Claim failed');
    }
    setLoading(false);
  }

  return (
    <main className="min-h-screen bg-black text-white font-mono">
      {/* header */}
      <header className="border-b border-zinc-800 px-8 py-4 flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold tracking-widest text-green-400">STABLEBANK</h1>
          <p className="text-xs text-zinc-500 tracking-wider">DECENTRALIZED SAVINGS PROTOCOL</p>
        </div>
        <WalletButton className="!bg-green-400 !text-black !font-bold !rounded-none !text-sm !tracking-wider" />
      </header>

      {!connected ? (
        /* landing */
        <div className="flex flex-col items-center justify-center min-h-[80vh] px-8 text-center">
          <div className="mb-8 text-green-400 text-6xl font-bold tracking-tighter">
            YOUR MONEY.<br />YOUR YIELD.
          </div>
          <p className="text-zinc-400 max-w-md mb-4 text-sm leading-relaxed tracking-wide">
            Deposit SOL. StableBank stakes it natively on Solana.
            You earn 6–9% APY. No banks. No middlemen. No bullshit.
          </p>
          <p className="text-zinc-600 text-xs tracking-widest mb-8">
            CONNECT YOUR WALLET TO GET STARTED
          </p>
          <WalletButton className="!bg-green-400 !text-black !font-bold !rounded-none !px-8 !py-3 !text-sm !tracking-widest" />
        </div>
      ) : (
        /* dashboard */
        <div className="max-w-4xl mx-auto px-8 py-12 space-y-8">

          {/* stats row */}
          <div className="grid grid-cols-3 gap-4">
            <div className="border border-zinc-800 p-6">
              <p className="text-xs text-zinc-500 tracking-widest mb-2">TOTAL DEPOSITED</p>
              <p className="text-3xl font-bold text-white">
                {balanceData ? balanceData.total_deposited.toFixed(4) : '0.0000'}
                <span className="text-sm text-zinc-500 ml-2">SOL</span>
              </p>
            </div>
            <div className="border border-zinc-800 p-6">
              <p className="text-xs text-zinc-500 tracking-widest mb-2">TOTAL STAKED</p>
              <p className="text-3xl font-bold text-white">
                {balanceData ? balanceData.total_staked_sol.toFixed(4) : '0.0000'}
                <span className="text-sm text-zinc-500 ml-2">SOL</span>
              </p>
            </div>
            <div className="border border-zinc-800 p-6">
              <p className="text-xs text-zinc-500 tracking-widest mb-2">YIELD EARNED</p>
              <p className="text-3xl font-bold text-green-400">
                {balanceData ? balanceData.estimated_yield_sol.toFixed(6) : '0.000000'}
                <span className="text-sm text-zinc-500 ml-2">SOL</span>
              </p>
            </div>
          </div>

          {/* deposit */}
          <div className="border border-zinc-800 p-6">
            <p className="text-xs text-zinc-500 tracking-widest mb-4">DEPOSIT SOL</p>
            <div className="flex gap-4">
              <input
                type="number"
                placeholder="0.00"
                value={depositAmount}
                onChange={(e) => setDepositAmount(e.target.value)}
                className="bg-zinc-900 border border-zinc-700 text-white px-4 py-3 w-full focus:outline-none focus:border-green-400 text-sm tracking-wider"
              />
              <button
                onClick={handleDeposit}
                disabled={loading}
                className="bg-green-400 text-black font-bold px-8 py-3 text-sm tracking-widest hover:bg-green-300 disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap"
              >
                {loading ? 'PROCESSING...' : 'DEPOSIT'}
              </button>
            </div>
          </div>

          {/* active stakes */}
          {balanceData && balanceData.active_stakes?.length > 0 && (
            <div className="border border-zinc-800 p-6">
              <p className="text-xs text-zinc-500 tracking-widest mb-4">ACTIVE STAKE ACCOUNTS</p>
              <div className="space-y-3">
                {balanceData.active_stakes.map((stake) => (
                  <div key={stake.stake_account} className="flex items-center justify-between border border-zinc-800 p-4">
                    <div>
                      <p className="text-xs text-zinc-400 font-mono">
                        {stake.stake_account.slice(0, 8)}...{stake.stake_account.slice(-8)}
                      </p>
                      <p className="text-white font-bold mt-1">
                        {stake.amount_sol.toFixed(6)} SOL
                      </p>
                      <p className="text-xs text-zinc-600 mt-1">
                        {new Date(stake.staked_at).toLocaleDateString()}
                      </p>
                    </div>
                    <button
                      onClick={() => handleClaim(stake.stake_account)}
                      disabled={loading}
                      className="border border-zinc-600 text-zinc-400 px-4 py-2 text-xs tracking-widest hover:border-green-400 hover:text-green-400 disabled:opacity-50"
                    >
                      CLAIM
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* withdraw */}
          <div className="border border-zinc-800 p-6">
            <p className="text-xs text-zinc-500 tracking-widest mb-2">REQUEST WITHDRAWAL</p>
            <p className="text-xs text-zinc-600 mb-4 leading-relaxed">
              Deactivates all stake accounts. After one epoch cooldown, use CLAIM above to receive your SOL + yield.
            </p>
            <button
              onClick={handleWithdraw}
              disabled={loading}
              className="border border-zinc-600 text-zinc-400 px-8 py-3 text-sm tracking-widest hover:border-red-400 hover:text-red-400 disabled:opacity-50"
            >
              {loading ? 'PROCESSING...' : 'REQUEST WITHDRAW'}
            </button>
          </div>

          {/* message */}
          {message && (
            <div className="border border-green-400 p-4">
              <p className="text-green-400 text-sm tracking-wide">{message}</p>
            </div>
          )}

          {/* wallet info */}
          <div className="border border-zinc-900 p-4">
            <p className="text-xs text-zinc-600 tracking-widest">
              CONNECTED: {publicKey?.toString().slice(0, 8)}...{publicKey?.toString().slice(-8)}
            </p>
            <p className="text-xs text-zinc-600 tracking-widest mt-1">
              WALLET BALANCE: {balanceData ? balanceData.wallet_balance_sol.toFixed(4) : '---'} SOL
            </p>
          </div>

        </div>
      )}
    </main>
  );
}
