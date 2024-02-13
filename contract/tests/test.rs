use restaurant_review::process_instruction;

use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
    },
    solana_program_test::{processor, tokio, ProgramTest},
    solana_sdk::{signature::Signer, signer::keypair::Keypair, transaction::Transaction},
    std::str::FromStr,
};

#[tokio::test]
async fn success() {
    let program_id = Pubkey::default();

    let program_test = ProgramTest::new(
        "RestaurantReview",
        program_id,
        processor!(process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
}
