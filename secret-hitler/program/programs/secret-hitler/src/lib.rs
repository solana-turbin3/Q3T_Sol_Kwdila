pub use crate::errors::GameErrorCode;
pub use anchor_lang::prelude::*;
pub use session_keys::{session_auth_or, Session, SessionError};
pub mod constants;
pub mod enums;
pub mod errors;
pub mod helpers;
pub mod instructions;
pub mod state;
use instructions::*;

declare_id!("AnvTCoxxQzscMBqVPtdEsc6it1U39rmqt6rQvfCt9Uac");

#[program]
pub mod secret_hitler {

    use super::*;
}
