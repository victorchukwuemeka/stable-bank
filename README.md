# StableBank

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
           |
           v
StableBank stakes it natively on Solana
           |
           v
Staking yield accumulates (6–9% APY)
           |
           v
User earns yield on their balance
           |
           v
User withdraws SOL + earnings anytime
```

StableBank is the savings account that banks never gave you.

---

## Onramp / Offramp

StableBank is designed so that anyone — even without crypto knowledge — can use it.
No wallet setup. No SOL required upfront. Just savings that earn.

```
ONRAMP
User has Naira / USD
           |
           v
Pay via bank transfer, card, or mobile money
(Transak / Flutterwave)
           |
           v
SOL purchased automatically
           |
           v
SOL deposited into StableBank and starts earning

OFFRAMP
User wants to withdraw earnings
           |
           v
StableBank unstakes SOL
           |
           v
SOL converts to USDC via Jupiter swap
           |
           v
Transak / Flutterwave converts USDC to Naira / USD
           |
           v
Lands in user bank account
```

Onramp / Offramp providers:

| Provider | Coverage | Notes |
|---|---|---|
| Transak | Nigeria, 150+ countries | Naira + bank transfer support |
| Flutterwave | Nigeria first | Local bank transfer, cards |
| MoonPay | Global | Card payments |
| Onramp.money | Solana native | Direct SOL onramp |

---

## Staking Backends

StableBank does not build staking from scratch.
It sits on top of existing, battle-tested Solana staking protocols:

| Protocol | Type | Est. Yield |
|---|---|---|
| Native Solana Staking | Validator delegation | ~6-7% APY |
| Marinade Finance | Liquid Staking | ~7% APY |
| Jito | Liquid Staking + MEV | ~8-9% APY |
| Sanctum | LST Infrastructure | Varies |

StableBank routes deposits to these protocols and passes yield back to users minus a small protocol fee.

---

## Yield Aggregator (Phase 2)

StableBank will automatically route SOL to whichever protocol offers the best yield at any given time.

```
User deposits SOL
           |
           v
Aggregator checks all protocols in real time
Native: 6-7% | Marinade: 7% | Jito: 8-9% | Kamino: 10%+
           |
           v
SOL routed to best yield automatically
           |
           v
User always earns maximum available APY
```

---

## Agentic Yield Optimization (Phase 3)

An autonomous agent monitors yields 24/7 and rebalances deposits when better opportunities arise.

```
Agent detects Jito yield > Native yield by 1%
           |
           v
Agent triggers rebalance instruction
           |
           v
SOL moves from Native staking to Jito automatically
           |
           v
User earns better yield without doing anything
```

---

## Architecture

### Option A — Off-Chain (Phase 1, Now)
A backend server holds a custodial wallet, receives user SOL, stakes natively on Solana, and tracks balances in a database.

- Cheapest to build, no deployment costs, ship fast
- Custodial — users trust you with their SOL
- Best for: personal testing and close circle of users

### Option B — Hybrid (Phase 2)
A lightweight Pinocchio program handles deposits and withdrawals. Backend handles staking and yield logic.

- Partially trustless, lower cost than fully on-chain
- Best for: small public beta

### Option C — Fully On-Chain (Phase 3)
Everything lives in a Solana program. Non-custodial. Users always control their funds.

- Fully trustless, production-grade
- Best for: full public launch

> Current stage: Option A

---

## Tech Stack

| Layer | Tool |
|---|---|
| Blockchain | Solana |
| On-Chain Programs | Pinocchio |
| Backend | Rust (Axum, solana-sdk, solana-client) |
| Staking | Native Solana staking → Marinade / Jito (Phase 2) |
| Database | SQLite (Phase 1) → PostgreSQL (Phase 2) |
| Frontend | React + Solana Wallet Adapter |
| Onramp / Offramp | Transak / Flutterwave |
| RPC | Helius (free tier) |

---

## Project Structure

```
stablebank/
├── backend/                   # Rust backend (Axum)
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── db.rs
│   │   ├── staking.rs
│   │   ├── marinade.rs
│   │   ├── wallet/
│   │   │   └── mod.rs
│   │   └── routes/
│   │       ├── mod.rs
│   │       ├── deposit.rs
│   │       ├── balance.rs
│   │       └── withdraw.rs
│   ├── Cargo.toml
│   └── .env
├── programs/                  # On-chain programs (Pinocchio) — Phase 2
│   └── stablebank/
│       └── src/
├── app/                       # Frontend (React) — in progress
│   ├── components/
│   └── pages/
├── docs/
│   ├── whitepaper.md
│   └── architecture.md
├── README.md
└── .gitignore
```

---

## Status

> Phase 1 — Backend Complete, Frontend Next

| Milestone | Status |
|---|---|
| Whitepaper v0.1 | Done |
| README | Done |
| Protocol wallet + Solana connection | Done |
| Deposit route + DB recording | Done |
| Native staking integration | Done |
| Balance + yield tracking route | Done |
| Withdraw + claim routes | Done |
| Frontend (deposit / balance / withdraw) | In progress |
| Yield aggregator (Marinade + Jito) | Phase 2 |
| Agentic yield optimizer | Phase 2 |
| Onramp / Offramp integration | Phase 2 |
| Pinocchio on-chain program | Phase 3 |
| Security review | Phase 3 |
| Mainnet launch | Phase 3 |

---

## Roadmap

### Phase 1 — Off-Chain MVP (Now)
- Backend with deposit, staking, balance, withdraw
- SQLite database tracking all user positions
- Frontend — deposit, dashboard, withdraw UI
- Test full cycle on devnet with personal SOL
- Onboard small circle of trusted users

### Phase 2 — Aggregator + Agent
- Integrate Marinade and Jito alongside native staking
- Yield aggregator — auto route to best APY
- Autonomous agent — monitors and rebalances 24/7
- Onramp / Offramp — Naira to SOL and back
- Migrate database to PostgreSQL

### Phase 3 — Fully On-Chain
- Pinocchio program — non-custodial deposits and withdrawals
- On-chain aggregator logic
- Crank bot — off-chain agent triggers on-chain rebalancing
- Apply for Solana Foundation grant for audit
- Mainnet deployment with TVL cap
- Superteam Nigeria — community and funding

---

## Funding & Cost Reality

| Cost | Phase 1 | Phase 2 | Phase 3 |
|---|---|---|---|
| Deployment | $0 (off-chain) | Low (small program) | Medium (full program) |
| RPC (Helius) | Free tier | Free / $9/mo | Paid plan |
| Hosting | Free (Railway / Render) | Free / cheap | Paid |
| Audit | Not needed | Not needed | Required |
| Starting SOL | Your own | Your own | Protocol reserve needed |

Funding targets: Solana Foundation grants, Superteam Nigeria, Colosseum hackathons.

---

## Risks & Honesty

- Phase 1 is custodial. Early users are trusting you personally, not a smart contract. Be transparent about this.
- Staking protocols carry risk. Battle-tested but not risk-free.
- Slashing risk is near zero on Solana but worth monitoring.
- Regulatory landscape around crypto savings products is evolving. Build responsibly.

---

## Contributing

Solo project in Phase 1. Contributions will open in Phase 2.
Ideas and feedback? Open an issue.

---

## License

MIT — see LICENSE

---

*StableBank — because you should be your own bank.*