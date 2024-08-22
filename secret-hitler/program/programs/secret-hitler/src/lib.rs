pub use crate::errors::GameErrorCode;
pub use anchor_lang::prelude::*;

pub mod constants;
pub mod enums;
pub mod errors;
pub mod helpers;
pub mod instructions;
pub mod state;

pub use enums::*;
pub use instructions::*;

declare_id!("AnvTCoxxQzscMBqVPtdEsc6it1U39rmqt6rQvfCt9Uac");

#[program]
pub mod secret_hitler {

    use super::*;
}
