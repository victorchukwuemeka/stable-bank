'use client';

import dynamic from 'next/dynamic';

const WalletMultiButton = dynamic(
    () => import('@solana/wallet-adapter-react-ui').then(mod => mod.WalletMultiButton),
    { ssr: false }
);

export default function WalletButton({ className }: { className?: string }) {
    return <WalletMultiButton className={className} />;
}