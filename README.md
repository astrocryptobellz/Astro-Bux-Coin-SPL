# Astro Bux SPL Rewards (BARP)

An Anchor program scaffold for admin-authorized Astro Bux SPL token rewards.

## Features
- Creates a classic SPL Token mint controlled by a program-derived authority.
- Creates reward campaigns with per-campaign caps.
- Awards tokens to recipient associated token accounts.
- Prevents duplicate awards for a 32-byte activity ID within a campaign.
- Supports pausing awards and emits campaign/reward events.

## Safety status
This is an unaudited development scaffold, not production-ready. No wallet was connected, no token was minted, no transaction was sent, and nothing was deployed.

**Important:** The program ID is currently the System Program placeholder. Replace it with a newly generated program ID and update both `declare_id!` and `Anchor.toml` before validator testing or deployment. Never commit private keys or seed phrases.

## Layout
- `programs/astro_bux_rewards/src/lib.rs`: Anchor program
- `tests/astro_bux_rewards.ts`: integration-test starter
- `SECURITY.md`: assumptions and review checklist

## Build/test
After replacing the placeholder program ID and installing Rust, Solana, Anchor, and Node dependencies:
```bash
anchor build
anchor test
```
These commands have not been run here.

Amounts and campaign caps are expressed in the mint's smallest units. With 6 decimals, 1 ASTRO = 1,000,000 base units.

This program only mints capped campaign rewards. It does not implement a sale, presale, liquidity pool, swap, staking, or automatic payout schedule.