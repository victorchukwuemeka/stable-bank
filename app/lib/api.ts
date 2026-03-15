const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3002';

export async function deposit(fromPubkey: string, amountSol: number) {
    const res = await fetch(`${API_URL}/deposit`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ from_pubkey: fromPubkey, amount_sol: amountSol }),
    });
    return res.json();
}

export async function getBalance(pubkey: string) {
    const res = await fetch(`${API_URL}/balance/${pubkey}`);
    return res.json();
}

export async function requestWithdraw(pubkey: string) {
    const res = await fetch(`${API_URL}/withdraw`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ pubkey }),
    });
    return res.json();
}

export async function claimWithdraw(pubkey: string, stakeAccount: string) {
    const res = await fetch(`${API_URL}/withdraw/claim`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ pubkey, stake_account: stakeAccount }),
    });
    return res.json();
}

export async function getProtocolConfig(){
    const res = await fetch(`${API_URL}/config`);
    return res.json()
}