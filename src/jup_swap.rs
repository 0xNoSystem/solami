use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_sdk::pubkey::{Pubkey, PubkeyError};
use solana_sdk::pubkey;


#[tokio::main]
pub async fn get_quote(token_in: &str, token_out: &str, wallet: &str, amount: u64){
    let jupiter_swap_api_client = JupiterSwapApiClient::new(String::from("https://quote-api.jup.ag/v6"));

    
    let token_in: Pubkey = to_pubkey(token_in);
    let token_out: Pubkey = to_pubkey(token_out);
    let wallet: Pubkey = to_pubkey(wallet);

    //amount: 1000 =  1 SOL
    let quote_request = QuoteRequest {
        amount: amount,
        input_mint: token_in,
        output_mint: token_out,
        slippage_bps: 50,
        ..QuoteRequest::default()
    };

    // GET /quote
    let quote_response = jupiter_swap_api_client.quote(&quote_request).await.unwrap();
    println!("{quote_response:#?}");

    // POST /swap
    let swap_response = jupiter_swap_api_client
        .swap(&SwapRequest {
            user_public_key: wallet,
            quote_response: quote_response.clone(),
            config: TransactionConfig::default(),
        })
        .await
        .unwrap();

    println!("Raw tx len: {}", swap_response.swap_transaction.len());

    // Perform further actions as needed...

    // POST /swap-instructions
    let swap_instructions = jupiter_swap_api_client
        .swap_instructions(&SwapRequest {
            user_public_key: wallet,
            quote_response,
            config: TransactionConfig::default(),
        })
        .await
        .unwrap();
    println!("{swap_instructions:#?}");
}



fn to_pubkey(string: &str) -> Pubkey{

    assert_eq!(string.len(), 42, "Address length is different than 32");

    let mut string_array: [u8; 42] = [0; 42];
    string_array.copy_from_slice(string.as_bytes());

    pubkey!(Pubkey::from(string_array))
}
