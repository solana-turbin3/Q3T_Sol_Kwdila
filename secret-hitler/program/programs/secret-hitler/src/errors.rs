use anchor_lang::error_code;

#[error_code]
pub enum GameErrorCode {
    #[msg("Game must be in Setup state to join")]
    GameNotInSetupState,
    #[msg("Player has already joined the game")]
    PlayerAlreadyJoined,
    #[msg("The game is full")]
    MaxPlayersReached,
    #[msg("The player is not in the game")]
    PlayerNotInGame,
    #[msg("Host Player Can not leave game. Consider ending game instead")]
    HostPlayerLeaving,
    #[msg("Bet vault must be provided with bet amount")]
    BetVaultNotFound,
    #[msg("Deposit vault must be provided with deposit amount")]
    DepositVaultNotFound,
}
