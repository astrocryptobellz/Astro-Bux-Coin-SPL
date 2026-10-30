# BARP Security Notes

**Status: prototype only; unaudited, uncompiled, and untested in this workflow. Do not use with real funds or valuable production tokens.**

## v0.2 trust boundaries

- The first initializer becomes the administrator; there is no admin rotation or recovery instruction.
- Members sign their own registration and receive a profile PDA scoped to the program config and wallet.
- Only the administrator can create/manage campaigns, approve awards, suspend/reactivate members, and pause/unpause the program.
- The administrator is a trusted off-chain eligibility attestor. The contract checks registration and campaign category but cannot prove a task, tenure milestone, or contribution actually occurred.
- Each campaign has its own cap. There is no global lifetime supply cap.
- A receipt PDA prevents reuse of the same 32-byte activity ID within one campaign.
- Member reward totals are cumulative accounting counters in mint base units; they are not a separate withdrawable balance.

## Important limitations and review items

- Campaign category is an administrative label; the program does not enforce category-specific evidence.
- Admin compromise permits arbitrary rewards to registered members up to campaign caps.
- No admin rotation, multi-signature governance, global mint cap, rate limiting, campaign expiration, receipt cleanup, member profile closure, or on-chain evidence oracle exists.
- Mint and freeze authority are the program PDA. Review the intended freeze-authority policy and recovery implications.
- The program ID remains a placeholder and must not be used for validator testing or deployment.
- The public repository exposes source code; never commit keypairs, seed phrases, wallet exports, or secrets.

## Required tests before any testnet consideration

1. Compile with pinned toolchain and dependency versions.
2. Test registration, duplicate registration, profile PDA derivation, and member-wallet binding.
3. Test each reward category and reject inactive/unregistered members.
4. Test unauthorized admin actions and unauthorized pause/unpause.
5. Test duplicate activity IDs, cross-campaign receipt isolation, campaign cap exhaustion, cap changes below minted amount, zero awards, and arithmetic overflow.
6. Test inactive campaigns, global pause, wrong config/mint/campaign accounts, wrong member profile, and ATA constraints.
7. Run formatting, static analysis, dependency review, and an independent Solana security review of PDA seeds, account constraints, CPI signer seeds, token authorities, and economics.
8. Document eligibility evidence, admin operating procedures, supply policy, decimals, freeze authority, incident response, and key management.

No wallet connection, signing, transaction, minting, deployment, or spending is authorized by this development task.
