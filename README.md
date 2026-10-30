# Astro Bux SPL Rewards Protocol (BARP)

BARP is an Anchor program scaffold for admin-attested Astro Bux SPL token rewards on Solana.

## BARP v0.2.0 — combined rewards

Members register their own profile PDA and can qualify for three campaign categories:

- **Activity:** approved tasks and participation.
- **Membership:** approved member participation or tenure milestones.
- **Contribution:** approved work that helps build the Astro ecosystem.

The program enforces an active member profile, active campaign, global pause switch, campaign cap, and unique 32-byte activity ID per campaign. Every successful award creates an on-chain receipt and updates the member's cumulative reward counter.

## Trust and eligibility model

The program does **not** independently verify off-chain activity, membership tenure, or contribution quality. The administrator is the trusted attestor who decides whether evidence qualifies and submits the award. Do not describe awards as automatic or permissionless.

Members sign their own registration. The administrator can suspend or reactivate a member profile, manage campaign status and caps, and pause the reward program.

## Repository layout

- `programs/astro_bux_rewards/src/lib.rs`: Anchor program
- `tests/astro_bux_rewards.ts`: local-validator test starter
- `SECURITY.md`: threat model and pre-test review checklist

## Amounts

All reward amounts and caps are expressed in the SPL mint's smallest units. With 6 decimals, 1 ASTRO = 1,000,000 base units.

## Current safety status

**Prototype only. Not audited, not compiled, and not tested in this workflow.** The program ID is still a placeholder. The repository source is public. No wallet was connected, no token was minted, no transaction was submitted, and nothing was deployed.

## Local test plan (not executed)

After review, install pinned Rust, Solana, Anchor, and Node dependencies, generate a program ID for local testing, update `declare_id!` and `Anchor.toml`, then run:

```bash
cargo fmt --all -- --check
anchor build
anchor test
```

These commands have not been run. Tests use a local validator and are not a substitute for an independent security audit.

BARP does not implement token sales, presales, swaps, liquidity pools, staking, or automatic recurring payouts. No mainnet launch is part of this version.
