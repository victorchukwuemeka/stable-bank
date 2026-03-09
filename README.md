# StableBank 🏦

**A decentralized savings protocol on Solana.**
> Deposit SOL. We stake it. You earn. No banks. No middlemen.

---

## The Idea

Traditional banks take your money, lend it out at 10–20%, and pay you back 0.5%.
StableBank flips that. Your SOL gets staked on existing Solana protocols — you keep the yield.

No LP. No trading. No complexity.
Just **deposit → stake → earn.**

---

## How It Works

```
User sends SOL to StableBank
           ↓
StableBank stakes it on Marinade / Jito
           ↓
Staking yield accumulates (7–9% APY)
           ↓
User earns yield on their balance
           ↓
User withdraws SOL + earnings anytime
```

StableBank is the savings account that banks never gave you.

---

## Staking Backends

StableBank does not build staking from scratch.
It sits **on top** of existing, battle-tested Solana staking protocols:

| Protocol | Type | Est. Yield |
|---|---|---|
| [Marinade Finance](https://marinade.finance) | Liquid Staking | ~7% APY |
| [Jito](https://jito.network) | Liquid Staking + MEV | ~8–9% APY |
| [Sanctum](https://sanctum.so) | LST Infrastructure | Varies |

> StableBank routes deposits to these protocols and passes yield back to users minus a small protocol fee.

---

## Architecture Approaches

StableBank can be built in three ways depending on the stage:

### Option A — Off-Chain *(Phase 1, Now)*
A backend server holds a custodial wallet, receives user SOL, stakes it via Marinade/Jito SDK, and tracks balances in a database.

- ✅ Cheapest to build
- ✅ No deployment costs
- ✅ Ship fast, validate fast
- ⚠️ Custodial — users trust you with their SOL
- **Best for:** Friends and family early adopters

### Option B — Hybrid *(Phase 2)*
A lightweight on-chain program handles deposits and withdrawals. Backend handles staking logic and yield distribution.

- ✅ Partially trustless
- ✅ Lower cost than fully on-chain
- ⚠️ Still some custodial elements
- **Best for:** Small public beta

### Option C — Fully On-Chain *(Phase 3)*
Everything lives in a Solana program. Non-custodial. Users always control their funds.

- ✅ Fully trustless
- ✅ Production-grade
- ⚠️ Highest cost (deployment rent + audit)
- **Best for:** Full public launch

> **Current stage: Option A**

---

## Tech Stack

| Layer | Tool |
|---|---|
| Blockchain | Solana |
| On-Chain Programs | [Pinocchio](https://github.com/febo/pinocchio) |
| Staking | Marinade SDK / Jito SDK |
| Backend | Node.js / TypeScript |
| Database | PostgreSQL (balance tracking) |
| Frontend | React + Solana Wallet Adapter |
| RPC | Helius (free tier) |

---

## Project Structure

```
stablebank/
├── programs/              # On-chain programs (Pinocchio)
│   └── stablebank/
│       └── src/
├── app/                   # Frontend (React)
│   ├── components/
│   └── pages/
├── backend/               # Off-chain server (Node.js)
│   ├── staking/           # Marinade / Jito integration
│   ├── accounts/          # User balance tracking
│   └── yield/             # Yield calculation + distribution
├── docs/                  # Documentation
│   ├── whitepaper.md
│   └── architecture.md
├── scripts/               # Utility + deployment scripts
├── tests/                 # Tests
├── README.md
└── package.json
```

> Update this structure to match your actual setup as you build.

---

## Status

> 🟡 **Phase 1 — Building & Validating**

| Milestone | Status |
|---|---|
| Whitepaper v0.1 | ✅ Done |
| README | ✅ Done |
| Backend wallet + staking integration | 🔲 Up next |
| Marinade / Jito SDK connection | 🔲 Not started |
| Balance tracking (DB) | 🔲 Not started |
| Basic frontend (deposit / withdraw) | 🔲 Not started |
| Test with personal SOL | 🔲 Not started |
| Invite first users (people you know) | 🔲 Not started |
| On-chain program (Pinocchio) | 🔲 Phase 2 |
| Public launch | 🔲 Phase 3 |

---

## Roadmap

### Phase 1 — Off-Chain MVP *(Now)*
- Build backend that receives SOL and stakes via Marinade / Jito
- Track user balances in a database
- Build a simple UI — deposit, balance, withdraw
- Test with your own SOL first
- Onboard a small circle of trusted users

### Phase 2 — Hybrid Protocol
- Introduce a lightweight Pinocchio program for deposits / withdrawals
- Reduce custodial surface area
- Add yield dashboard for users
- Expand to more staking backends

### Phase 3 — Fully On-Chain
- Full Pinocchio on-chain program
- Non-custodial — users control their funds
- Security review
- Open public launch

---

## Funding & Cost Reality

StableBank is designed to be built with **minimal capital.** Here's the honest breakdown:

| Cost | Phase 1 | Phase 2 | Phase 3 |
|---|---|---|---|
| Deployment | $0 (off-chain) | Low (small program) | Medium (full program) |
| RPC (Helius) | Free tier | Free / $9/mo | Paid plan |
| Hosting (backend) | Free (Railway / Render) | Free / cheap | Paid |
| Audit | Not needed | Not needed | Required |
| Starting SOL | Your own | Your own | Protocol reserve needed |

> The goal of Phase 1 is to prove the model works **before spending serious money.**

---

## Risks & Honesty

- **Phase 1 is custodial.** Early users are trusting you personally, not a smart contract. Be transparent about this.
- **Staking protocols carry risk.** Marinade and Jito are battle-tested but not risk-free.
- **Slashing risk is near zero on Solana** but worth monitoring.
- **Regulatory landscape** around crypto savings products is evolving. Build responsibly.

---

## Contributing

Solo project in Phase 1. Contributions will open in Phase 2.
Ideas and feedback? Open an issue.

---

## License

MIT — see [LICENSE](./LICENSE)

---

*StableBank — because you should be your own bank.*