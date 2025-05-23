//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

#[cfg(feature = "anchor")]
use anchor_lang::prelude::{AnchorDeserialize, AnchorSerialize};
#[cfg(not(feature = "anchor"))]
use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct CreateBondingCurve {
    pub mint: solana_program::pubkey::Pubkey,

    pub creator: solana_program::pubkey::Pubkey,

    pub bonding_curve: solana_program::pubkey::Pubkey,

    pub bonding_curve_token_account: solana_program::pubkey::Pubkey,

    pub global: solana_program::pubkey::Pubkey,

    pub whitelist: Option<solana_program::pubkey::Pubkey>,

    pub metadata: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub associated_token_program: solana_program::pubkey::Pubkey,

    pub token_metadata_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl CreateBondingCurve {
    pub fn instruction(
        &self,
        args: CreateBondingCurveInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateBondingCurveInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mint, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.creator,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bonding_curve,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bonding_curve_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.global,
            false,
        ));
        if let Some(whitelist) = self.whitelist {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                whitelist, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::PUMP_SCIENCE_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.metadata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_metadata_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateBondingCurveInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::PUMP_SCIENCE_ID,
            accounts,
            data,
        }
    }
}

#[cfg_attr(not(feature = "anchor"), derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "anchor", derive(AnchorSerialize, AnchorDeserialize))]
pub struct CreateBondingCurveInstructionData {
    discriminator: [u8; 8],
}

impl CreateBondingCurveInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [94, 139, 158, 50, 69, 95, 8, 45],
        }
    }
}

#[cfg_attr(not(feature = "anchor"), derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "anchor", derive(AnchorSerialize, AnchorDeserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateBondingCurveInstructionArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub start_time: Option<i64>,
}

/// Instruction builder for `CreateBondingCurve`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` mint
///   1. `[writable, signer]` creator
///   2. `[writable]` bonding_curve
///   3. `[writable]` bonding_curve_token_account
///   4. `[]` global
///   5. `[optional]` whitelist
///   6. `[writable]` metadata
///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   8. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   9. `[]` associated_token_program
///   10. `[optional]` token_metadata_program (default to `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`)
///   11. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   12. `[]` event_authority
///   13. `[]` program
#[derive(Default)]
pub struct CreateBondingCurveBuilder {
    mint: Option<solana_program::pubkey::Pubkey>,
    creator: Option<solana_program::pubkey::Pubkey>,
    bonding_curve: Option<solana_program::pubkey::Pubkey>,
    bonding_curve_token_account: Option<solana_program::pubkey::Pubkey>,
    global: Option<solana_program::pubkey::Pubkey>,
    whitelist: Option<solana_program::pubkey::Pubkey>,
    metadata: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    token_metadata_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    start_time: Option<i64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateBondingCurveBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    #[inline(always)]
    pub fn creator(&mut self, creator: solana_program::pubkey::Pubkey) -> &mut Self {
        self.creator = Some(creator);
        self
    }
    #[inline(always)]
    pub fn bonding_curve(&mut self, bonding_curve: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bonding_curve = Some(bonding_curve);
        self
    }
    #[inline(always)]
    pub fn bonding_curve_token_account(
        &mut self,
        bonding_curve_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.bonding_curve_token_account = Some(bonding_curve_token_account);
        self
    }
    #[inline(always)]
    pub fn global(&mut self, global: solana_program::pubkey::Pubkey) -> &mut Self {
        self.global = Some(global);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn whitelist(&mut self, whitelist: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.whitelist = whitelist;
        self
    }
    #[inline(always)]
    pub fn metadata(&mut self, metadata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_token_program = Some(associated_token_program);
        self
    }
    /// `[optional account, default to 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s']`
    #[inline(always)]
    pub fn token_metadata_program(
        &mut self,
        token_metadata_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_metadata_program = Some(token_metadata_program);
        self
    }
    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(&mut self, program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.symbol = Some(symbol);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.uri = Some(uri);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn start_time(&mut self, start_time: i64) -> &mut Self {
        self.start_time = Some(start_time);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts =
            CreateBondingCurve {
                mint: self.mint.expect("mint is not set"),
                creator: self.creator.expect("creator is not set"),
                bonding_curve: self.bonding_curve.expect("bonding_curve is not set"),
                bonding_curve_token_account: self
                    .bonding_curve_token_account
                    .expect("bonding_curve_token_account is not set"),
                global: self.global.expect("global is not set"),
                whitelist: self.whitelist,
                metadata: self.metadata.expect("metadata is not set"),
                system_program: self
                    .system_program
                    .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                )),
                associated_token_program: self
                    .associated_token_program
                    .expect("associated_token_program is not set"),
                token_metadata_program: self.token_metadata_program.unwrap_or(
                    solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                ),
                rent: self.rent.unwrap_or(solana_program::pubkey!(
                    "SysvarRent111111111111111111111111111111111"
                )),
                event_authority: self.event_authority.expect("event_authority is not set"),
                program: self.program.expect("program is not set"),
            };
        let args = CreateBondingCurveInstructionArgs {
            name: self.name.clone().expect("name is not set"),
            symbol: self.symbol.clone().expect("symbol is not set"),
            uri: self.uri.clone().expect("uri is not set"),
            start_time: self.start_time.clone(),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_bonding_curve` CPI accounts.
pub struct CreateBondingCurveCpiAccounts<'a, 'b> {
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub creator: &'b solana_program::account_info::AccountInfo<'a>,

    pub bonding_curve: &'b solana_program::account_info::AccountInfo<'a>,

    pub bonding_curve_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub global: &'b solana_program::account_info::AccountInfo<'a>,

    pub whitelist: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_bonding_curve` CPI instruction.
pub struct CreateBondingCurveCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub creator: &'b solana_program::account_info::AccountInfo<'a>,

    pub bonding_curve: &'b solana_program::account_info::AccountInfo<'a>,

    pub bonding_curve_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub global: &'b solana_program::account_info::AccountInfo<'a>,

    pub whitelist: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateBondingCurveInstructionArgs,
}

impl<'a, 'b> CreateBondingCurveCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateBondingCurveCpiAccounts<'a, 'b>,
        args: CreateBondingCurveInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            mint: accounts.mint,
            creator: accounts.creator,
            bonding_curve: accounts.bonding_curve,
            bonding_curve_token_account: accounts.bonding_curve_token_account,
            global: accounts.global,
            whitelist: accounts.whitelist,
            metadata: accounts.metadata,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            associated_token_program: accounts.associated_token_program,
            token_metadata_program: accounts.token_metadata_program,
            rent: accounts.rent,
            event_authority: accounts.event_authority,
            program: accounts.program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(14 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mint.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.creator.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bonding_curve.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bonding_curve_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.global.key,
            false,
        ));
        if let Some(whitelist) = self.whitelist {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *whitelist.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::PUMP_SCIENCE_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.metadata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_metadata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateBondingCurveInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::PUMP_SCIENCE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(14 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.creator.clone());
        account_infos.push(self.bonding_curve.clone());
        account_infos.push(self.bonding_curve_token_account.clone());
        account_infos.push(self.global.clone());
        if let Some(whitelist) = self.whitelist {
            account_infos.push(whitelist.clone());
        }
        account_infos.push(self.metadata.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.associated_token_program.clone());
        account_infos.push(self.token_metadata_program.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.event_authority.clone());
        account_infos.push(self.program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `CreateBondingCurve` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` mint
///   1. `[writable, signer]` creator
///   2. `[writable]` bonding_curve
///   3. `[writable]` bonding_curve_token_account
///   4. `[]` global
///   5. `[optional]` whitelist
///   6. `[writable]` metadata
///   7. `[]` system_program
///   8. `[]` token_program
///   9. `[]` associated_token_program
///   10. `[]` token_metadata_program
///   11. `[]` rent
///   12. `[]` event_authority
///   13. `[]` program
pub struct CreateBondingCurveCpiBuilder<'a, 'b> {
    instruction: Box<CreateBondingCurveCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateBondingCurveCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateBondingCurveCpiBuilderInstruction {
            __program: program,
            mint: None,
            creator: None,
            bonding_curve: None,
            bonding_curve_token_account: None,
            global: None,
            whitelist: None,
            metadata: None,
            system_program: None,
            token_program: None,
            associated_token_program: None,
            token_metadata_program: None,
            rent: None,
            event_authority: None,
            program: None,
            name: None,
            symbol: None,
            uri: None,
            start_time: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    #[inline(always)]
    pub fn creator(
        &mut self,
        creator: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.creator = Some(creator);
        self
    }
    #[inline(always)]
    pub fn bonding_curve(
        &mut self,
        bonding_curve: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bonding_curve = Some(bonding_curve);
        self
    }
    #[inline(always)]
    pub fn bonding_curve_token_account(
        &mut self,
        bonding_curve_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bonding_curve_token_account = Some(bonding_curve_token_account);
        self
    }
    #[inline(always)]
    pub fn global(
        &mut self,
        global: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.global = Some(global);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn whitelist(
        &mut self,
        whitelist: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.whitelist = whitelist;
        self
    }
    #[inline(always)]
    pub fn metadata(
        &mut self,
        metadata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata = Some(metadata);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.associated_token_program = Some(associated_token_program);
        self
    }
    #[inline(always)]
    pub fn token_metadata_program(
        &mut self,
        token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_metadata_program = Some(token_metadata_program);
        self
    }
    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(
        &mut self,
        program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program = Some(program);
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.instruction.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.instruction.symbol = Some(symbol);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.instruction.uri = Some(uri);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn start_time(&mut self, start_time: i64) -> &mut Self {
        self.instruction.start_time = Some(start_time);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = CreateBondingCurveInstructionArgs {
            name: self.instruction.name.clone().expect("name is not set"),
            symbol: self.instruction.symbol.clone().expect("symbol is not set"),
            uri: self.instruction.uri.clone().expect("uri is not set"),
            start_time: self.instruction.start_time.clone(),
        };
        let instruction = CreateBondingCurveCpi {
            __program: self.instruction.__program,

            mint: self.instruction.mint.expect("mint is not set"),

            creator: self.instruction.creator.expect("creator is not set"),

            bonding_curve: self
                .instruction
                .bonding_curve
                .expect("bonding_curve is not set"),

            bonding_curve_token_account: self
                .instruction
                .bonding_curve_token_account
                .expect("bonding_curve_token_account is not set"),

            global: self.instruction.global.expect("global is not set"),

            whitelist: self.instruction.whitelist,

            metadata: self.instruction.metadata.expect("metadata is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            associated_token_program: self
                .instruction
                .associated_token_program
                .expect("associated_token_program is not set"),

            token_metadata_program: self
                .instruction
                .token_metadata_program
                .expect("token_metadata_program is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            event_authority: self
                .instruction
                .event_authority
                .expect("event_authority is not set"),

            program: self.instruction.program.expect("program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CreateBondingCurveCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    creator: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bonding_curve: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bonding_curve_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    global: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    whitelist: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    metadata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_metadata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    start_time: Option<i64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
