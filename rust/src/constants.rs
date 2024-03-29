use super::schemas::packed::{Uint32, Uint32Reader};
use core::convert::TryFrom;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum SystemStatus {
    Off,
    On,
}

pub const PRESERVED_ACCOUNT_CELL_COUNT: u8 = 20;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum DataType {
    ActionData = 0,
    AccountCellData,
    AccountSaleCellData,
    AccountAuctionCellData,
    ProposalCellData,
    PreAccountCellData,
    IncomeCellData,
    OfferCellData,
    SubAccount,
    ConfigCellAccount = 100,              // args: 0x64000000
    ConfigCellApply = 101,                // args: 0x65000000
    ConfigCellIncome = 103,               // args: 0x67000000
    ConfigCellMain,                       // args: 0x68000000
    ConfigCellPrice,                      // args: 0x69000000
    ConfigCellProposal,                   // args: 0x6a000000
    ConfigCellProfitRate,                 // args: 0x6b000000
    ConfigCellRecordKeyNamespace,         // args: 0x6c000000
    ConfigCellRelease,                    // args: 0x6d000000
    ConfigCellUnAvailableAccount,         // args: 0x6e000000
    ConfigCellSecondaryMarket,            // args: 0x6f000000
    ConfigCellReverseResolution,          // args: 0x70000000
    ConfigCellSubAccount,                 // args: 0x71000000
    ConfigCellSubAccountBetaList,         // args: 0x72000000
    ConfigCellPreservedAccount00 = 10000, // args: 0x10270000
    ConfigCellPreservedAccount01,
    ConfigCellPreservedAccount02,
    ConfigCellPreservedAccount03,
    ConfigCellPreservedAccount04,
    ConfigCellPreservedAccount05,
    ConfigCellPreservedAccount06,
    ConfigCellPreservedAccount07,
    ConfigCellPreservedAccount08,
    ConfigCellPreservedAccount09,
    ConfigCellPreservedAccount10,
    ConfigCellPreservedAccount11,
    ConfigCellPreservedAccount12,
    ConfigCellPreservedAccount13,
    ConfigCellPreservedAccount14,
    ConfigCellPreservedAccount15,
    ConfigCellPreservedAccount16,
    ConfigCellPreservedAccount17,
    ConfigCellPreservedAccount18,
    ConfigCellPreservedAccount19,     // args: 0x23270000
    ConfigCellCharSetEmoji = 100000,  // args: 0xa0860100
    ConfigCellCharSetDigit = 100001,  // args: 0xa1860100
    ConfigCellCharSetEn = 100002,     // args: 0xa2860100
    ConfigCellCharSetZhHans = 100003, // args: 0xa3860100, not available yet
    ConfigCellCharSetZhHant = 100004, // args: 0xa4860100, not available yet
}

impl TryFrom<u32> for DataType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == DataType::ActionData as u32 => Ok(DataType::ActionData),
            x if x == DataType::AccountCellData as u32 => Ok(DataType::AccountCellData),
            x if x == DataType::AccountSaleCellData as u32 => Ok(DataType::AccountSaleCellData),
            x if x == DataType::AccountAuctionCellData as u32 => Ok(DataType::AccountAuctionCellData),
            x if x == DataType::ProposalCellData as u32 => Ok(DataType::ProposalCellData),
            x if x == DataType::PreAccountCellData as u32 => Ok(DataType::PreAccountCellData),
            x if x == DataType::IncomeCellData as u32 => Ok(DataType::IncomeCellData),
            x if x == DataType::OfferCellData as u32 => Ok(DataType::OfferCellData),
            x if x == DataType::SubAccount as u32 => Ok(DataType::SubAccount),
            x if x == DataType::ConfigCellAccount as u32 => Ok(DataType::ConfigCellAccount),
            x if x == DataType::ConfigCellApply as u32 => Ok(DataType::ConfigCellApply),
            x if x == DataType::ConfigCellIncome as u32 => Ok(DataType::ConfigCellIncome),
            x if x == DataType::ConfigCellMain as u32 => Ok(DataType::ConfigCellMain),
            x if x == DataType::ConfigCellPrice as u32 => Ok(DataType::ConfigCellPrice),
            x if x == DataType::ConfigCellProposal as u32 => Ok(DataType::ConfigCellProposal),
            x if x == DataType::ConfigCellProfitRate as u32 => Ok(DataType::ConfigCellProfitRate),
            x if x == DataType::ConfigCellRelease as u32 => Ok(DataType::ConfigCellRelease),
            x if x == DataType::ConfigCellSecondaryMarket as u32 => Ok(DataType::ConfigCellSecondaryMarket),
            x if x == DataType::ConfigCellReverseResolution as u32 => Ok(DataType::ConfigCellReverseResolution),
            x if x == DataType::ConfigCellSubAccount as u32 => Ok(DataType::ConfigCellSubAccount),
            x if x == DataType::ConfigCellSubAccountBetaList as u32 => Ok(DataType::ConfigCellSubAccountBetaList),
            x if x == DataType::ConfigCellRecordKeyNamespace as u32 => Ok(DataType::ConfigCellRecordKeyNamespace),
            x if x == DataType::ConfigCellUnAvailableAccount as u32 => Ok(DataType::ConfigCellUnAvailableAccount),
            x if x == DataType::ConfigCellPreservedAccount00 as u32 => Ok(DataType::ConfigCellPreservedAccount00),
            x if x == DataType::ConfigCellPreservedAccount01 as u32 => Ok(DataType::ConfigCellPreservedAccount01),
            x if x == DataType::ConfigCellPreservedAccount02 as u32 => Ok(DataType::ConfigCellPreservedAccount02),
            x if x == DataType::ConfigCellPreservedAccount03 as u32 => Ok(DataType::ConfigCellPreservedAccount03),
            x if x == DataType::ConfigCellPreservedAccount04 as u32 => Ok(DataType::ConfigCellPreservedAccount04),
            x if x == DataType::ConfigCellPreservedAccount05 as u32 => Ok(DataType::ConfigCellPreservedAccount05),
            x if x == DataType::ConfigCellPreservedAccount06 as u32 => Ok(DataType::ConfigCellPreservedAccount06),
            x if x == DataType::ConfigCellPreservedAccount07 as u32 => Ok(DataType::ConfigCellPreservedAccount07),
            x if x == DataType::ConfigCellPreservedAccount08 as u32 => Ok(DataType::ConfigCellPreservedAccount08),
            x if x == DataType::ConfigCellPreservedAccount09 as u32 => Ok(DataType::ConfigCellPreservedAccount09),
            x if x == DataType::ConfigCellPreservedAccount10 as u32 => Ok(DataType::ConfigCellPreservedAccount10),
            x if x == DataType::ConfigCellPreservedAccount11 as u32 => Ok(DataType::ConfigCellPreservedAccount11),
            x if x == DataType::ConfigCellPreservedAccount12 as u32 => Ok(DataType::ConfigCellPreservedAccount12),
            x if x == DataType::ConfigCellPreservedAccount13 as u32 => Ok(DataType::ConfigCellPreservedAccount13),
            x if x == DataType::ConfigCellPreservedAccount14 as u32 => Ok(DataType::ConfigCellPreservedAccount14),
            x if x == DataType::ConfigCellPreservedAccount15 as u32 => Ok(DataType::ConfigCellPreservedAccount15),
            x if x == DataType::ConfigCellPreservedAccount16 as u32 => Ok(DataType::ConfigCellPreservedAccount16),
            x if x == DataType::ConfigCellPreservedAccount17 as u32 => Ok(DataType::ConfigCellPreservedAccount17),
            x if x == DataType::ConfigCellPreservedAccount18 as u32 => Ok(DataType::ConfigCellPreservedAccount18),
            x if x == DataType::ConfigCellPreservedAccount19 as u32 => Ok(DataType::ConfigCellPreservedAccount19),
            x if x == DataType::ConfigCellCharSetEmoji as u32 => Ok(DataType::ConfigCellCharSetEmoji),
            x if x == DataType::ConfigCellCharSetDigit as u32 => Ok(DataType::ConfigCellCharSetDigit),
            x if x == DataType::ConfigCellCharSetEn as u32 => Ok(DataType::ConfigCellCharSetEn),
            x if x == DataType::ConfigCellCharSetZhHans as u32 => Ok(DataType::ConfigCellCharSetZhHans),
            x if x == DataType::ConfigCellCharSetZhHant as u32 => Ok(DataType::ConfigCellCharSetZhHant),
            _ => Err(()),
        }
    }
}

impl TryFrom<Uint32> for DataType {
    type Error = ();

    fn try_from(v: Uint32) -> Result<Self, Self::Error> {
        Self::try_from(u32::from(v))
    }
}

impl<'r> TryFrom<Uint32Reader<'r>> for DataType {
    type Error = ();

    fn try_from(v: Uint32Reader) -> Result<Self, Self::Error> {
        Self::try_from(u32::from(v))
    }
}

// The length of CharSetType
pub const CHAR_SET_LENGTH: usize = 5;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum CharSetType {
    Emoji,
    Digit,
    En,
    ZhHans,
    ZhHant,
    // ⚠️ DO NOT Forget to update CHAR_SET_LENGTH at the same time.
}

impl TryFrom<u32> for CharSetType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == CharSetType::Emoji as u32 => Ok(CharSetType::Emoji),
            x if x == CharSetType::Digit as u32 => Ok(CharSetType::Digit),
            x if x == CharSetType::En as u32 => Ok(CharSetType::En),
            x if x == CharSetType::ZhHans as u32 => Ok(CharSetType::ZhHans),
            x if x == CharSetType::ZhHant as u32 => Ok(CharSetType::ZhHant),
            _ => Err(()),
        }
    }
}

impl TryFrom<Uint32> for CharSetType {
    type Error = ();

    fn try_from(v: Uint32) -> Result<Self, Self::Error> {
        Self::try_from(u32::from(v))
    }
}

impl<'r> TryFrom<Uint32Reader<'r>> for CharSetType {
    type Error = ();

    fn try_from(v: Uint32Reader) -> Result<Self, Self::Error> {
        Self::try_from(u32::from(v))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum ProposalSliceItemType {
    Exist,
    Proposed,
    New,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum AccountStatus {
    Normal,
    Selling,
    Auction,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum SubAccountEnableStatus {
    Off,
    On,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum LockRole {
    Owner,
    Manager,
}

// [100, 97, 115] equals b"das"
pub const WITNESS_HEADER: [u8; 3] = [100, 97, 115];
pub const WITNESS_HEADER_BYTES: usize = WITNESS_HEADER.len();
pub const WITNESS_TYPE_BYTES: usize = 4;
pub const WITNESS_LENGTH_BYTES: usize = 4;
