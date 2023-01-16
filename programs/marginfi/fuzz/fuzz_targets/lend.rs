#![no_main]

use std::collections::BTreeMap;

use anchor_lang::{
    prelude::{AccountLoader, Context, Pubkey, Rent},
    Key,
};
use anyhow::Result;
use arbitrary::Arbitrary;
use fixed::types::I80F48;
use libfuzzer_sys::fuzz_target;
use marginfi::{
    prelude::MarginfiGroup,
    state::marginfi_group::{Bank, WrappedI80F48},
};
use marginfi_fuzz::{setup_marginfi_group, BankAndOracleConfig, MarginfiGroupAccounts};

#[derive(Debug, Arbitrary)]
enum Action {
    Deposit {
        owner: OwnerId,
        bank: BankId,
        asset_amount: AssetAmount,
    },
    Withdraw {
        owner: OwnerId,
        bank: BankId,
        asset_amount: AssetAmount,
    },
    UpdateOracle {
        bank: BankId,
    },
    AccrueInterest {
        bank: BankId,
    },
    UpdateBankConfig,
    Liquidate {
        liquidator: OwnerId,
        liquidatee: OwnerId,
        asset_bank: BankId,
        liability_bank: BankId,
        asset_amount: AssetAmount,
    },
    HandleBankruptcy {
        owner: OwnerId,
        bank: BankId,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnerId(u8);
const NUM_OWNERS: u8 = 8;
impl<'a> Arbitrary<'a> for OwnerId {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let i: u8 = u.arbitrary()?;
        Ok(OwnerId(i % NUM_OWNERS))
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        (1, Some(1))
    }

    fn arbitrary_take_rest(mut u: arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Self::arbitrary(&mut u)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BankId(u8);
const N_BANKS: usize = 8;
impl<'a> Arbitrary<'a> for BankId {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let i: u8 = u.arbitrary()?;
        Ok(BankId(i % N_BANKS as u8))
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        (1, Some(1))
    }

    fn arbitrary_take_rest(mut u: arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Self::arbitrary(&mut u)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssetAmount(u64);
const MAX_ASSET_AMOUNT: u64 = 1_000_000;
impl<'a> Arbitrary<'a> for AssetAmount {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let i: u64 = u.arbitrary()?;
        Ok(AssetAmount(i % MAX_ASSET_AMOUNT))
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        (8, Some(8))
    }

    fn arbitrary_take_rest(mut u: arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Self::arbitrary(&mut u)
    }
}

#[derive(Debug, Arbitrary)]
pub struct ActionSequence(Vec<Action>);

#[derive(Debug, Arbitrary)]
pub struct FuzzerContext {
    pub action_sequence: ActionSequence,
    pub initial_bank_configs: [BankAndOracleConfig; N_BANKS],
}

fuzz_target!(|data: FuzzerContext| { process_actions(data).unwrap() });

fn process_actions(ctx: FuzzerContext) -> Result<()> {
    let bump = bumpalo::Bump::new();
    let mut mga = MarginfiGroupAccounts::setup(&bump);

    mga.setup_banks(&bump, Rent::free(), N_BANKS, &ctx.initial_bank_configs);

    let al =
        AccountLoader::<MarginfiGroup>::try_from_unchecked(&marginfi::id(), &mga.marginfi_group)
            .unwrap();
    assert_eq!(al.load()?.admin, mga.owner.key());

    // for action in actions {
    //     process_action(action, fuzzer_context)?;
    // }

    Ok(())
}

fn process_action(action: Action) -> Result<()> {
    match action {
        //     Action::Deposit {
        //         asset_amount,
        //         bank,
        //         owner,
        //     } => {
        //         marginfi::instructions::bank_deposit(Context::new(
        //             marginfi::id(),
        //             marginfi::accounts::BankDeposit {
        //                 marginfi_group: todo!(),
        //                 marginfi_account: todo!(),
        //                 signer: todo!(),
        //                 bank: todo!(),
        //                 signer_token_account: todo!(),
        //                 bank_liquidity_vault: todo!(),
        //                 token_program: todo!(),
        //             },
        //             &[],
        //             BTreeMap::new(),
        //         ));
        //     }
        _ => unimplemented!(),
    };

    Ok(())
}
