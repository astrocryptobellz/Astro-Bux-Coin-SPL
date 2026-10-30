import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Keypair } from "@solana/web3.js";

describe("astro_bux_rewards", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.AstroBuxRewards as Program<any>;
  const admin = (provider.wallet as anchor.Wallet).publicKey;
  const recipient = Keypair.generate().publicKey;

  const [config] = PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId);
  const [mintAuthority] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint_authority"), config.toBuffer()], program.programId
  );
  const [rewardMint] = PublicKey.findProgramAddressSync(
    [Buffer.from("reward_mint"), config.toBuffer()], program.programId
  );

  it("initializes the reward mint", async () => {
    await program.methods.initialize(6).accounts({
      admin, config, mintAuthority, rewardMint,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY,
    }).rpc();

    const stored = await program.account.rewardsConfig.fetch(config);
    assert.equal(stored.admin.toBase58(), admin.toBase58());
    assert.equal(stored.rewardMint.toBase58(), rewardMint.toBase58());
    assert.equal(stored.paused, false);
  });

  it("creates a capped campaign and rejects a duplicate activity ID", async () => {
    const campaignId = new anchor.BN(1);
    const [campaign] = PublicKey.findProgramAddressSync([
      Buffer.from("campaign"), config.toBuffer(),
      campaignId.toArrayLike(Buffer, "le", 8),
    ], program.programId);
    await program.methods.createCampaign(campaignId, new anchor.BN(5_000_000))
      .accounts({ config, admin, campaign, systemProgram: SystemProgram.programId }).rpc();

    const activityId = Buffer.alloc(32, 7);
    const [receipt] = PublicKey.findProgramAddressSync(
      [Buffer.from("receipt"), campaign.toBuffer(), activityId], program.programId
    );
    const recipientTokenAccount = getAssociatedTokenAddressSync(rewardMint, recipient);

    const accounts = {
      config, admin, recipient, campaign, mintAuthority, rewardMint, receipt,
      recipientTokenAccount, tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    };
    await program.methods.awardReward([...activityId], new anchor.BN(1_000_000))
      .accounts(accounts).rpc();

    const tokenAccount = await getAccount(provider.connection, recipientTokenAccount);
    assert.equal(tokenAccount.amount, BigInt(1_000_000));

    let rejected = false;
    try {
      await program.methods.awardReward([...activityId], new anchor.BN(1))
        .accounts(accounts).rpc();
    } catch { rejected = true; }
    assert.equal(rejected, true, "duplicate activity must be rejected");
  });
});
