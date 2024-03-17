use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use cp_swap::states::{AmmConfig, PoolState};
use spl_tlv_account_resolution::seeds::Seed;
use spl_tlv_account_resolution::account::ExtraAccountMeta;
use anchor_spl::{
   token_interface::{Mint, TokenAccount, TokenInterface}
};
use spl_tlv_account_resolution::{
    state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::{ExecuteInstruction, TransferHookInstruction};

declare_id!("DrWbQtYJGtsoRwzKqAbHKHKsCJJfpysudF39GBVFSxub");

#[program]
pub mod transfer_hook {
    use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};

    use super::*;

    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        // the fee is 1% of the amount
        let fee = amount / 100;
        // half the fee is burned. token_0_account is the source token account
        let burn_amount = fee / 2;
        // a quarter is swapped into sol
        let sol_amount = fee / 4;
        // the rest is deposited along with sol
        let deposit_amount = fee - burn_amount - sol_amount;
        // burn_ix
        let burn_ix = spl_token_2022::instruction::burn(
            &ctx.accounts.token_program_2022.key(),
            &ctx.accounts.source_token.key(),
            &ctx.accounts.mint.key(),
            &ctx.accounts.owner.key(),
            &[],
            burn_amount,
        )?;
        
        let seeds: &[&[u8]] = &[b"delegate",
            &[ctx.bumps.delegate],
        ];

        invoke_signed(&burn_ix, &[
            ctx.accounts.source_token.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.token_program_2022.to_account_info(),
        ], &[&seeds])?;
        
        
        invoke_signed(
            &Instruction {
                program_id: cp_swap::id(),
                accounts: vec![
                    AccountMeta::new(ctx.accounts.delegate.key(), true),
                    //authority
                    AccountMeta::new(ctx.accounts.authority.key(), false),
                    // amm_config
                    
                    AccountMeta::new(ctx.accounts.amm_config.key(), false),
                    // pool_state
                    AccountMeta::new(ctx.accounts.pool_state.key(), false),
                    //input_token_account
                    AccountMeta::new(ctx.accounts.token_0_account.key(), false),
                    //output_token_account
                    AccountMeta::new(ctx.accounts.token_1_account.key(), false),
                    //input_token_vault
                    AccountMeta::new(ctx.accounts.token_0_vault.key(), false),
                    //output_token_vault
                    AccountMeta::new(ctx.accounts.token_1_vault.key(), false),
                    //input_token_program
                    AccountMeta::new(ctx.accounts.token_program_2022.key(), false),
                    //output_token_program
                    AccountMeta::new(ctx.accounts.token_program.key(), false),
                    //input_mint
                    AccountMeta::new(ctx.accounts.mint.key(), false),
                    //output_mint
                    AccountMeta::new(ctx.accounts.vault_1_mint.key(), false),

                ],
                data: cp_swap::instruction::SwapBaseInput {
                    amount_in: sol_amount,
                    minimum_amount_out: 0
                }
                .try_to_vec()?,

            },
            &[
                ctx.accounts.delegate.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.amm_config.to_account_info(),
                ctx.accounts.pool_state.to_account_info(),
                ctx.accounts.token_0_account.to_account_info(),
                ctx.accounts.token_1_account.to_account_info(),
                ctx.accounts.token_0_vault.to_account_info(),
                ctx.accounts.token_1_vault.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.token_program_2022.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.vault_1_mint.to_account_info(),
                
            ],
            &[&seeds],
        )?;

        // deposit_ix

        invoke_signed(
            &Instruction {
                program_id: cp_swap::id(),
                accounts: vec![
                    AccountMeta::new(ctx.accounts.delegate.key(), true),
                    //authority
                    AccountMeta::new(ctx.accounts.authority.key(), false),
                    // pool_state
                    AccountMeta::new(ctx.accounts.pool_state.key(), false),
                    //lp_token
                    AccountMeta::new(ctx.accounts.owner_lp_token.key(), false),
                    //input_token_account
                    AccountMeta::new(ctx.accounts.token_0_account.key(), false),
                    //output_token_account
                    AccountMeta::new(ctx.accounts.token_1_account.key(), false),
                    //input_token_vault
                    AccountMeta::new(ctx.accounts.token_0_vault.key(), false),
                    //output_token_vault
                    AccountMeta::new(ctx.accounts.token_1_vault.key(), false),
                    //input_token_program
                    AccountMeta::new(ctx.accounts.token_program.key(), false),
                    //output_token_program
                    AccountMeta::new(ctx.accounts.token_program_2022.key(), false),
                    //input_mint
                    AccountMeta::new(ctx.accounts.mint.key(), false),
                    //output_mint
                    AccountMeta::new(ctx.accounts.vault_1_mint.key(), false),
                    //lp_mint
                    AccountMeta::new(ctx.accounts.lp_mint.key(), false),

                ],
                data: cp_swap::instruction::SwapBaseInput {
                    amount_in: deposit_amount,
                    minimum_amount_out: 0
                }
                .try_to_vec()?,

            },
            &[
                ctx.accounts.delegate.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.pool_state.to_account_info(),
                ctx.accounts.owner_lp_token.to_account_info(),
                ctx.accounts.token_0_account.to_account_info(),
                ctx.accounts.token_1_account.to_account_info(),
                ctx.accounts.token_0_vault.to_account_info(),
                ctx.accounts.token_1_vault.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.token_program_2022.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.vault_1_mint.to_account_info(),
                ctx.accounts.lp_mint.to_account_info(),
                
            ],
            &[&seeds],
        )?;



        msg!("Hello Transfer Hook!");
        Ok(())
    }

    // fallback instruction handler as workaround to anchor instruction discriminator check
    pub fn fallback<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        data: &[u8],
    ) -> Result<()> {
        let instruction = TransferHookInstruction::unpack(data)?;

        // match instruction discriminator to transfer hook interface execute instruction  
        // token2022 program CPIs this instruction on token transfer
        match instruction {
            TransferHookInstruction::Execute { amount } => {
                let amount_bytes = amount.to_le_bytes();

                // invoke custom transfer hook instruction on our program
                __private::__global::transfer_hook(program_id, accounts, &amount_bytes)
            }
            _ => return Err(ProgramError::InvalidInstructionData.into()),
        }
    }
    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        let account_metas = vec![
            // Accounts needed for Swap
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.mint.key(), false, true)?,
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.vault_1_mint.key(), false, true)?,
            // Accounts needed for delegate
            ExtraAccountMeta::new_with_seeds(
                &[Seed::Literal {
                    bytes: "delegate".as_bytes().to_vec(),
                }],
                false, // is_signer
                false,  // is_writable
            )?,
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.token_program_2022.key(), false, true)?,
            // amm_config, pool_state, owner_lp_token, token_0_account, token_1_account, token_0_vault, token_1_vault
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.amm_config.key(), false, true)?,
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.pool_state.key(), false, true)?,
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.owner_lp_token.key(), false, true)?,
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.token_0_account.key(), false, true)?,
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.token_1_account.key(), false, true)?,
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.token_0_vault.key(), false, true)?,
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.token_1_vault.key(), false, true)?,
            // token_program, lp_mint
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.token_program.key(), false, true)?,
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.lp_mint.key(), false, true)?,
            // input_token_program, output_token_program, system_program
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.system_program.key(), false, true)?,
            // authority
            ExtraAccountMeta::new_with_pubkey(&ctx.accounts.authority.key(), true, false)?,
            
        ];

        // calculate account size
        let account_size = ExtraAccountMetaList::size_of(account_metas.len())? as u64;
        // calculate minimum required lamports
        let lamports = Rent::get()?.minimum_balance(account_size as usize);

        let mint = ctx.accounts.mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"extra-account-metas",
            &mint.as_ref(),
            &[ctx.bumps.extra_account_meta_list],
        ]];

        // create ExtraAccountMetaList account
        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.extra_account_meta_list.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            lamports,
            account_size,
            ctx.program_id,
        )?;

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &account_metas,
        )?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    // ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        payer = payer,
        space = 8 + 16 * 256, // Adjusted for a large number of accounts; tweak as necessary
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
    )]
    /// CHECK:no
    extra_account_meta_list: AccountInfo<'info>,

    #[account(address = token_0_vault.mint)]
    mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(address = token_1_vault.mint)]
    vault_1_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [b"delegate"],
        bump
    )]
    pub delegate: SystemAccount<'info>,

    token_program_2022: Interface<'info, TokenInterface>,

    #[account(address = pool_state.load()?.amm_config)]
    amm_config: Box<Account<'info, AmmConfig>>,

    #[account(mut)]
    pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    owner_lp_token: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    token_0_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    token_1_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    token_0_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    token_1_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    token_program: Interface<'info, TokenInterface>,

    #[account(mut, address = pool_state.load()?.lp_mint)]
    lp_mint: Box<InterfaceAccount<'info, Mint>>,

    system_program: Program<'info, System>,
    /// CHECK:no
    authority: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account,
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    

    #[account(address = token_1_vault.mint)]
    vault_1_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [b"delegate"],
        bump
    )]
    pub delegate: SystemAccount<'info>,

    token_program_2022: Interface<'info, TokenInterface>,

    #[account(address = pool_state.load()?.amm_config)]
    amm_config: Box<Account<'info, AmmConfig>>,

    #[account(mut)]
    pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    owner_lp_token: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    token_0_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    token_1_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    token_0_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    token_1_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    token_program: Interface<'info, TokenInterface>,

    #[account(mut, address = pool_state.load()?.lp_mint)]
    lp_mint: Box<InterfaceAccount<'info, Mint>>,

    system_program: Program<'info, System>,
    /// CHECK:no
    authority: AccountInfo<'info>,

}
