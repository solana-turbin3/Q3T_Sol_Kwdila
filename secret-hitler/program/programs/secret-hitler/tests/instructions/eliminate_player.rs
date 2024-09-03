use anchor_lang::prelude::{Pubkey, ToAccountMetas};

// Function to define the account metas needed for the Initialize instruction.
pub fn eliminate_player_metas(
    player: &Pubkey,
    game_data: &Pubkey,
    nomination: &Pubkey,
) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
    secret_hitler::accounts::EliminatePlayer {
        game_data: *game_data,
        player: *player,
        nomination: *nomination,
    }
    .to_account_metas(None)
}

// Function to define the data for the Initialize instruction.
pub fn eliminate_player_data() -> secret_hitler::instruction::EliminateInactivePlayer {
    // Return the Initialize instruction with the input and mint parameters.
    secret_hitler::instruction::EliminateInactivePlayer {}
}
