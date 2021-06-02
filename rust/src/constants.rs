use super::schemas::packed::{Uint32, Uint32Reader};
use core::convert::TryFrom;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum SystemStatus {
    Off,
    On,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum DataType {
    ActionData = 0,
    AccountCellData,
    OnSaleCellData,
    BiddingCellData,
    ProposalCellData,
    PreAccountCellData,
    IncomeCellData,
    ConfigCellAccount = 100,
    ConfigCellApply = 101,
    ConfigCellIncome = 102,
    ConfigCellMain,
    ConfigCellPrice,
    ConfigCellProposal,
    ConfigCellProfitRate,
    ConfigCellRecordKeyNamespace,
    ConfigCellPreservedAccount00 = 150,
    // ConfigCellPreservedAccount01,
    // ConfigCellPreservedAccount02,
    // ConfigCellPreservedAccount03,
    // ConfigCellPreservedAccount04,
    // ConfigCellPreservedAccount05,
    // ConfigCellPreservedAccount06,
    // ConfigCellPreservedAccount07,
    ConfigCellCharSetEmoji = 100000,
    ConfigCellCharSetDigit = 100001,
    ConfigCellCharSetEn = 100002,
}

impl TryFrom<u32> for DataType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == DataType::ActionData as u32 => Ok(DataType::ActionData),
            x if x == DataType::AccountCellData as u32 => Ok(DataType::AccountCellData),
            x if x == DataType::OnSaleCellData as u32 => Ok(DataType::OnSaleCellData),
            x if x == DataType::BiddingCellData as u32 => Ok(DataType::BiddingCellData),
            x if x == DataType::ProposalCellData as u32 => Ok(DataType::ProposalCellData),
            x if x == DataType::PreAccountCellData as u32 => Ok(DataType::PreAccountCellData),
            x if x == DataType::IncomeCellData as u32 => Ok(DataType::IncomeCellData),
            x if x == DataType::ConfigCellAccount as u32 => Ok(DataType::ConfigCellAccount),
            x if x == DataType::ConfigCellApply as u32 => Ok(DataType::ConfigCellApply),
            x if x == DataType::ConfigCellIncome as u32 => Ok(DataType::ConfigCellIncome),
            x if x == DataType::ConfigCellMain as u32 => Ok(DataType::ConfigCellMain),
            x if x == DataType::ConfigCellPrice as u32 => Ok(DataType::ConfigCellPrice),
            x if x == DataType::ConfigCellProposal as u32 => Ok(DataType::ConfigCellProposal),
            x if x == DataType::ConfigCellProfitRate as u32 => Ok(DataType::ConfigCellProfitRate),
            x if x == DataType::ConfigCellRecordKeyNamespace as u32 => {
                Ok(DataType::ConfigCellRecordKeyNamespace)
            }
            x if x == DataType::ConfigCellPreservedAccount00 as u32 => {
                Ok(DataType::ConfigCellPreservedAccount00)
            }
            x if x == DataType::ConfigCellCharSetEmoji as u32 => {
                Ok(DataType::ConfigCellCharSetEmoji)
            }
            x if x == DataType::ConfigCellCharSetDigit as u32 => {
                Ok(DataType::ConfigCellCharSetDigit)
            }
            x if x == DataType::ConfigCellCharSetEn as u32 => Ok(DataType::ConfigCellCharSetEn),
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
pub const CHAR_SET_LENGTH: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum CharSetType {
    Emoji,
    Digit,
    En,
    ZhCn,
}

impl TryFrom<u32> for CharSetType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == CharSetType::Emoji as u32 => Ok(CharSetType::Emoji),
            x if x == CharSetType::Digit as u32 => Ok(CharSetType::Digit),
            x if x == CharSetType::En as u32 => Ok(CharSetType::En),
            x if x == CharSetType::ZhCn as u32 => Ok(CharSetType::ZhCn),
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
pub enum LockRole {
    Owner,
    Manager,
}

// [100, 97, 115] equals "das".as_bytes()
pub const WITNESS_HEADER: [u8; 3] = [100, 97, 115];
