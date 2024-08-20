use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Setup,                        // Game is being set up.
    ChancellorNomination,         // President is nominating a chancellor.
    ChancellorVoting,             // Voting on the chancellor is taking place.
    LegislativePresident, // In the legislative phase. The president is selecting a card to discard.
    LegislativeChancellor, // In the legislative phase. The chancellor is selecting a card to enact.
    LegislativePresidentVeto, // Chancellor decided to initiate veto, President chooses whether to allow.
    PresidentialPowerPeek,    // President may peek at the next three cards in the deck
    PresidentialPowerInvestigate, // President can investigate a party membership
    PresidentialPowerExecution, // President may choose a player to execute
    PresidentialPowerElection, // President chooses the next president, seat continues as normal after.
    PostLegislative,           // Waiting for the President to end their turn.
    LiberalVictoryPolicy,      // Liberal Party won through enacting Liberal policies.
    LiberalVictoryExecution,   // Liberal Party won through executing Hitler.
    FascistVictoryPolicy,      // Fascist Party won through enacting Fascist policies.
    FascistVictoryElection,    // Fascist Party won by successfully electing Hitler chancellor.
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, PartialEq, Eq)]
pub enum PlayerCount {
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl PlayerCount {
    pub fn liberal_count(&self) -> u8 {
        match self {
            PlayerCount::Five => 3,
            PlayerCount::Six => 4,
            PlayerCount::Seven => 4,
            PlayerCount::Eight => 5,
            PlayerCount::Nine => 5,
            PlayerCount::Ten => 6,
        }
    }
    // number of fascists including hitler
    pub fn fascist_count(&self) -> u8 {
        match self {
            PlayerCount::Five => 2,
            PlayerCount::Six => 2,
            PlayerCount::Seven => 3,
            PlayerCount::Eight => 3,
            PlayerCount::Nine => 4,
            PlayerCount::Ten => 4,
        }
    }
}
