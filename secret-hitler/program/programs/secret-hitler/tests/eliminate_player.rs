use anchor_lang::prelude::{AccountMeta, Pubkey};
use anchor_lang::InstructionData;
use anchor_lang::Space;
use solana_program_test::*;
use solana_program_test::*;
use solana_sdk::{
    account::Account, clock::Clock, instruction::Instruction, native_token::LAMPORTS_PER_SOL,
    signature::keypair::Keypair, signer::Signer,
};

use solana_sdk::entrypoint::ProcessInstruction;

use instructions::*;
use secret_hitler::{
    entry,
    enums::{GameState, PlayerCount},
    state::GameData,
};
use utils::*;

mod instructions;
mod utils;

#[tokio::main]
async fn test_eliminate_player() {
    let mut program_test = ProgramTest::default();
    program_test.add_program("secret_hitler", secret_hitler::id(), convert_entry!(entry));

    let nomination = Keypair::new();
    let host = Keypair::new();
    airdrop(&mut program_test, host.pubkey(), 10 * LAMPORTS_PER_SOL);

    let mut all_players = vec![Keypair::new().pubkey(); 4];
    all_players.push(host.pubkey());

    let (game_key, bump) = get_game_data_address(&host.pubkey());

    let mut context = program_test.start_with_context().await;

    let curren_time: i64 = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .unix_timestamp;
    let mut game = vec![0u8; GameData::INIT_SPACE];
    GameData {
        host: host.pubkey(),
        turn_duration: 100,
        max_players: 5,
        start_player_count: Some(PlayerCount::Five),
        all_starting_players: all_players,
        active_players: all_players,
        turn_started_at: Some(curren_time),
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
    }
    .try_serialize(&mut game)
    .unwrap();

    context.set_account(
        &game_key,
        Account {
            lamports: u32::MAX as u64,
            data: game,
            owner: secret_hitler::id(),
            ..Account::default()
        },
    );

    let ix_metas = eliminate_player_metas(&host.pubkey(), &game_key, &nomination.pubkey());
    let ix_eliminate = build_ix(&secret_hitler::id(), ix_metas, eliminate_player_data());

    // Define the signers for the transaction.
    let signers = vec![&host];

    // Process the instruction in the simulated environment.
    let res = utils::process_instruction(
        &mut program_test_context,
        ix_initialize,
        &host.pubkey(),
        signers,
    )
    .await;

    // Assert that the instruction was successful.
    assert!(res.is_ok());
}
// Function to build the actual Solana instruction using the program ID, accounts, and data.
pub fn build_ix(program_id: &Pubkey, accounts: Vec<AccountMeta>, data: Vec<u8>) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}
