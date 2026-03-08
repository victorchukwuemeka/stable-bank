# StableBank — White Paper
### A Decentralized Web3 Banking Protocol on Solana

> **Version 0.1 — Brainstorm Draft**
> *"Be the bank. Keep the profit."*

---

## Table of Contents

- [StableBank — White Paper](#stablebank--white-paper)
    - [A Decentralized Web3 Banking Protocol on Solana](#a-decentralized-web3-banking-protocol-on-solana)
  - [Table of Contents](#table-of-contents)
  - [Abstract](#abstract)
  - [The Problem with Traditional Banking](#the-problem-with-traditional-banking)
  - [The Vision — A Web3 Bank](#the-vision--a-web3-bank)
  - [How StableBank Works](#how-stablebank-works)
  - [How Liquidity Provision (LP) Generates Yield](#how-liquidity-provision-lp-generates-yield)
  - [The Three-Layer Capital Strategy](#the-three-layer-capital-strategy)
    - [Layer 1 — The Savings Layer *(Low Risk)*](#layer-1--the-savings-layer-low-risk)
    - [Layer 2 — The Yield Layer *(Medium Risk)*](#layer-2--the-yield-layer-medium-risk)
    - [Layer 3 — The Growth Layer *(Higher Risk)*](#layer-3--the-growth-layer-higher-risk)
    - [Allocation Summary](#allocation-summary)
  - [The lSOL Receipt Token](#the-lsol-receipt-token)
  - [Risk Management](#risk-management)
  - [Roadmap — Two Phases](#roadmap--two-phases)
    - [Phase 1 — Personal Validation *(Now)*](#phase-1--personal-validation-now)
    - [Phase 2 — Open Protocol *(After Validation)*](#phase-2--open-protocol-after-validation)
  - [Conclusion](#conclusion)

---

## Abstract

StableBank is a decentralized, non-custodial banking protocol built on the Solana blockchain. It allows users to deposit SOL and have that capital automatically deployed across lending protocols and liquidity pools — earning yield that traditional banks keep for themselves.

The core principle is simple: **you become the bank.**

Instead of depositing money into a bank that earns 10–20% lending it out while paying you 0.5%, StableBank puts you directly in the seat of the lender. Your SOL works for you — earning swap fees, lending interest, and LP rewards — all on-chain, all transparent, all under your control.

---

## The Problem with Traditional Banking

Traditional banks operate on a model that is fundamentally misaligned with the depositor:

| What the Bank Does | What You Get |
|---|---|
| Takes your deposit | 0.1% – 0.5% savings interest |
| Lends it out at 10–20% | The bank keeps the spread |
| Provides liquidity to markets | Fees go to the institution |
| Holds your money | You need permission to access it |

The bank is profitable because **your money is the product.** You provide the capital. They collect the yield.

Web3 changes this equation entirely.

---

## The Vision — A Web3 Bank

StableBank is built around one foundational idea:

> *A savings and yield protocol where the depositor captures the value that banks traditionally extract.*

**Phase 1 — Personal Finance Layer**
A single user (the founder) deposits SOL, deploys it across yield strategies, and earns passively. This is the proof-of-concept phase — learn the mechanics, validate the returns, understand the risks with real capital before involving others.

**Phase 2 — Open Protocol**
Once the strategy is validated, StableBank opens to other users. Depositors lock SOL, receive a receipt token (`lSOL`), and earn proportional yield from the protocol's deployed capital. The protocol takes a small performance fee to sustain operations.

This two-phase approach ensures the protocol is **battle-tested before it holds other people's money.**

---

## How StableBank Works

The flow is straightforward:

```
User deposits SOL
        ↓
StableBank smart contract receives SOL
        ↓
Protocol splits SOL across 3 strategy layers
        ↓
Yield accumulates (LP fees + lending interest)
        ↓
User earns yield on their deposit
        ↓
User withdraws SOL + earned yield (minus protocol fee)
```

At deposit, the user receives `lSOL` — a receipt token representing their share of the pool plus accrued yield. When they withdraw, they burn `lSOL` and receive their SOL back with earnings included.

---

## How Liquidity Provision (LP) Generates Yield

Think of LP like owning a currency exchange booth at an airport.

Every time a traveler exchanges currency, the booth charges a small fee. The booth owner earns passively — the more people exchange, the more the owner earns. You don't need to do anything once the booth is set up.

In crypto, LP works the same way:

- You deposit two tokens into a **liquidity pool** (e.g. SOL + USDC)
- Every time a trader swaps between those tokens, a **swap fee** (typically 0.25% – 1%) is charged
- That fee is distributed **proportionally to all liquidity providers**
- The more volume the pool sees, the more you earn

**The key risk:** Impermanent Loss (IL). If the price of SOL moves significantly against USDC, the value of your LP position can be less than if you had simply held SOL. This is the primary risk to manage in the LP strategy layer.

StableBank mitigates this by using **stable pairs** for the majority of capital deployment.

---

## The Three-Layer Capital Strategy

StableBank deploys user capital across three layers, balancing safety and yield:

---

### Layer 1 — The Savings Layer *(Low Risk)*

**Target Allocation: 50% of TVL**

Capital is deposited into Solana lending protocols such as **Marginfi** or **Kamino**. No LP. No impermanent loss. Simply lending SOL/USDC to borrowers and earning interest.

- Expected APY: **5% – 15%**
- Risk level: Low
- Liquidity: High (withdraw anytime)
- Analogy: *A high-yield savings account*

---

### Layer 2 — The Yield Layer *(Medium Risk)*

**Target Allocation: 35% of TVL**

Capital enters liquidity pools with **stable pairs** (e.g. USDC/USDT) on protocols like **Orca** or **Raydium**. Because both tokens are pegged to $1, impermanent loss is near zero. Yield comes purely from swap fees.

- Expected APY: **10% – 25%**
- Risk level: Medium
- Liquidity: Medium
- Analogy: *A money market fund*

---

### Layer 3 — The Growth Layer *(Higher Risk)*

**Target Allocation: 15% of TVL**

A smaller portion of capital is deployed into SOL-paired LP pools (e.g. SOL/USDC). Higher swap fee revenue is earned here due to higher trading volume, but impermanent loss exposure is real. This layer is sized small intentionally.

- Expected APY: **20% – 50%+**
- Risk level: High
- Liquidity: Lower
- Analogy: *An equity growth fund*

---

### Allocation Summary

| Layer | Strategy | Allocation | Risk | Est. APY |
|---|---|---|---|---|
| Layer 1 | Lending (Marginfi/Kamino) | 50% | Low | 5–15% |
| Layer 2 | Stable LP (USDC/USDT) | 35% | Medium | 10–25% |
| Layer 3 | SOL-paired LP | 15% | High | 20–50%+ |

---

## The lSOL Receipt Token

When a user deposits SOL into StableBank, they receive **lSOL** — a liquid receipt token.

**What lSOL represents:**
- Proof of deposit
- Proportional claim on the yield pool
- The amount of SOL you can redeem at any time

**Key properties (to be decided):**

| Property | Option A | Option B |
|---|---|---|
| Transferability | Tradeable on open market | Non-transferable (locked) |
| Early exit | Sell lSOL to exit early | Wait for lock period |
| Complexity | Higher | Lower |

For Phase 1 (personal use), lSOL is simply an internal accounting token. For Phase 2 (open protocol), making lSOL tradeable gives users flexibility — they can exit their position by selling lSOL without the protocol needing to liquidate its LP positions.

---

## Risk Management

StableBank acknowledges the following risks and mitigation strategies:

**1. Impermanent Loss**
Mitigated by keeping the majority of capital (85%) in lending or stable LP pairs where IL is minimal or zero.

**2. Smart Contract Risk**
Third-party protocols (Marginfi, Kamino, Orca, Raydium) carry smart contract risk. Mitigated by using audited, battle-tested protocols and diversifying across multiple platforms.

**3. Liquidity Risk (Bank Run)**
If all users withdraw at once, the protocol may not have instant liquidity. Mitigated by maintaining a **liquidity reserve buffer** (target: 15–20% of TVL always kept liquid).

**4. Oracle / Price Risk**
LP positions depend on accurate price feeds. Mitigated by using Pyth Network or Switchboard oracle integrations.

**5. Regulatory Risk**
DeFi protocols operate in an evolving regulatory landscape. Non-custodial design (users always control their keys) minimizes custodial liability.

---

## Roadmap — Two Phases

### Phase 1 — Personal Validation *(Now)*

- [ ] Manually deposit SOL into Kamino/Marginfi — observe yields
- [ ] Open one stable LP position on Orca — observe fees
- [ ] Track performance over 30/60/90 days
- [ ] Document lessons learned and refine strategy allocations
- [ ] Define the smart contract architecture based on real experience

**Goal:** Prove the yield strategy works with personal capital before building for others.

---

### Phase 2 — Open Protocol *(After Validation)*

- [ ] Develop StableBank smart contracts on Solana (Anchor framework)
- [ ] Deploy lSOL receipt token
- [ ] Build deposit/withdraw UI
- [ ] Integrate with Marginfi, Kamino, Orca, Raydium
- [ ] Security audit
- [ ] Public beta launch
- [ ] DAO governance (community decides allocation weights)

**Goal:** Open StableBank to other users as a fully non-custodial web3 banking protocol.

---

## Conclusion

StableBank is a bet on a simple truth: **the depositor should capture the yield, not the institution.**

By leveraging Solana's fast, cheap infrastructure and the deep liquidity of its DeFi ecosystem, StableBank turns every depositor into a bank — earning lending interest, swap fees, and LP rewards that have historically been reserved for financial institutions.

The approach is deliberate and staged. Phase 1 is personal. Phase 2 is public. By the time others deposit their capital, the strategy has already been validated by the founders' own money.

> *Start as a saver. Scale as a bank.*

---

*StableBank — White Paper v0.1*
*Status: Brainstorm / Pre-Development*
*Chain: Solana*
*Last Updated: March 2026*