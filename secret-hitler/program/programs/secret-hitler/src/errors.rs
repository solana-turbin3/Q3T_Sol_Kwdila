use anchor_lang::error_code;

#[error_code]
pub enum GameErrorCode {
    #[msg("The Game must be in the required game state")]
    InvalidGameState,
    #[msg("Player has already joined the game")]
    PlayerAlreadyJoined,
    #[msg("The game is full")]
    MaxPlayersReached,
    #[msg("The specified player/pubkey is not in the game")]
    PlayerNotInGame,
    #[msg("Host Player Can not leave game. Consider ending game instead")]
    HostPlayerLeaving,
    #[msg("Bet vault must be provided with bet amount")]
    BetVaultNotFound,
    #[msg("Deposit vault must be provided with deposit amount")]
    DepositVaultNotFound,
    #[msg("5 to 10 players are needed to play")]
    MinimumPlayersNotReached,
    #[msg("The nominated chancellor is ineligible")]
    IneligibleChancellorNominated,
    #[msg("Eliminated Player can not participate in game")]
    EiminatedPlayer,
    #[msg("Number of votes can not exceed player count")]
    MaxVotesReached,
    #[msg("Each Player can vote once per nomination")]
    PlayerAlreadyVoted,
    #[msg("Wait for turn duration to finish")]
    TurnNotFinished,
}
