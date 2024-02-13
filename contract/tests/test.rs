use restaurant_review::{instructions::ReviewPayload, process_instruction};
use std::default;
use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction, system_program,
    },
    solana_program_test::{processor, tokio, ProgramTest},
    solana_sdk::{signature::Signer, signer::keypair::Keypair, transaction::Transaction},
    std::str::FromStr,
};

#[tokio::test]
async fn test_add_review() {
    let program_id = Pubkey::new_unique();

    let program_test = ProgramTest::new(
        "RestaurantReview",
        program_id,
        processor!(process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    // Payer is the initializer account

    let review_payload = ReviewPayload {
        title: String::from("Test Title"),
        description: String::from("Test description."),
        rating: 7,
        location: String::from("Test Location 12345"),
    };

    let (pda_pubkey, bump_seed) = Pubkey::find_program_address(
        &[
            payer.pubkey().as_ref(),
            review_payload.title.as_bytes().as_ref(),
        ],
        &program_id,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(pda_pubkey, false),
                AccountMeta::new(system_program::ID, false),
            ],
            // data: review_payload, // TODO Find a way to serialize this ReviewPayload into a Vec<u8>
            data: Vec::<u8>::default(),
        }],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();
}
