# Security Notes

**Status: prototype only; not audited. Do not use with real funds or valuable production tokens.**

## Authority and controls
- The initializer becomes the administrator.
- Only the administrator can create campaigns, award rewards, and pause the program.
- A PDA is the SPL mint authority.
- Campaign caps are denominated in token base units.
- A receipt PDA prevents reusing the same 32-byte activity ID in one campaign.

## Limitations
- The administrator can award tokens to any recipient up to the remaining campaign cap; the program does not verify off-chain activity or eligibility.
- There is no global supply cap, admin rotation/recovery, campaign cancellation, account closure, or rate limiting in this first version.
- The program ID is a placeholder.
- The code has not been compiled or independently reviewed in this workflow.

## Before deployment
1. Replace the placeholder program ID and securely retain its generated keypair.
2. Pin and verify toolchain/dependency versions.
3. Run formatting, build, tests, and static analysis.
4. Add tests for unauthorized access, pause behavior, duplicate IDs, cap exhaustion, overflow, wrong campaign/config, and invalid recipient accounts.
5. Obtain an independent Solana security review of PDA seeds, constraints, CPI signer seeds, authority lifecycle, and token economics.
6. Document decimals, total-supply policy, mint/freeze authority policy, and recovery procedures.
7. Obtain explicit approval before wallet connection, devnet/mainnet deployment, minting, or any transaction.

Never commit private keys, seed phrases, or wallet exports.