use anchor_lang::prelude::*;
use secret_hitler::{
    enums::{GameState, PlayerCount},
    id,
    state::GameData,
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::Instruction,
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer},
};

mod utils;
use utils::*;

mod instructions;
use instructions::*;

#[tokio::test]
async fn test_eliminate_player() {
    let program_id = id();
    let mut program_test = ProgramTest::new("secret_hitler", program_id, None);
    program_test.set_compute_max_units(200_000);

    let host = Keypair::new();
    let player_to_eliminate = Keypair::new();
    let nomination = Keypair::new();

    // Airdrop to host
    airdrop(&mut program_test, host.pubkey(), 10 * LAMPORTS_PER_SOL);

    // Generate game PDA
    let (game_pubkey, bump) = get_game_data_address(&host.pubkey());

    // Create initial game state
    let mut all_players = vec![Keypair::new().pubkey(); 4];
    all_players.push(host.pubkey());

    let game_data = GameData {
        host: host.pubkey(),
        turn_duration: 100,
        max_players: 5,
        start_player_count: Some(PlayerCount::Five),
        all_starting_players: all_players.clone(),
        active_players: all_players,
        turn_started_at: Some(0),
        eliminated_players: vec![],
        fascist_policies_enacted: 0,
        liberal_policies_enacted: 0,
        game_state: GameState::ChancellorNomination,
        is_special_election: false,
        failed_elections: 0,
        current_president_index: 4,
        bump,
        bet_amount: None,
        entry_deposit: None,
        current_chancellor_index: None,
        previous_chancellor_index: None,
        previous_president_index: None,
        bet_vault_bump: None,
        deposit_vault_bump: None,
    };

    let mut game_account_data = vec![0; GameData::INIT_SPACE];
    game_data.try_serialize(&mut game_account_data).unwrap();

    program_test.add_account(
        game_pubkey,
        Account {
            lamports: LAMPORTS_PER_SOL,
            data: game_account_data,
            owner: program_id,
            ..Account::default()
        },
    );

    // Create the instruction using the helper functions
    let ix_metas = eliminate_player_metas(&host.pubkey(), &game_pubkey, &nomination.pubkey());
    let ix_data = eliminate_player_data();

    let instruction = Instruction {
        program_id,
        accounts: ix_metas,
        data: ix_data.try_to_vec().unwrap(),
    };

    // Process the instruction
    let mut context = program_test.start_with_context().await;

    let result = process_instruction(&mut context, instruction, &host.pubkey(), vec![&host]).await;
    assert!(
        result.is_ok(),
        "Failed to process instruction: {:?}",
        result
    );

    // Forward time (if needed for your game logic)
    forward_time(&mut context, 60).await;

    // Verify the final state
    let game_account = context
        .banks_client
        .get_account(game_pubkey)
        .await
        .unwrap()
        .unwrap();
    let updated_game_data: GameData =
        GameData::try_deserialize(&mut game_account.data.as_ref()).unwrap();

    assert!(
        !updated_game_data
            .active_players
            .contains(&player_to_eliminate.pubkey()),
        "Player should be removed from active players"
    );
    assert!(
        updated_game_data
            .eliminated_players
            .contains(&player_to_eliminate.pubkey()),
        "Player should be added to eliminated players"
    );
}
