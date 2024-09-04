
use bs58;
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::pubkey;



#[tokio::main]
pub async fn get_quote(token_in: &str, token_out: &str, wallet: &str, amount: u64){
    let jupiter_swap_api_client = JupiterSwapApiClient::new(String::from("https://quote-api.jup.ag/v6"));

    let token_in: Pubkey = to_pubkey(token_in).unwrap();
    let token_out: Pubkey = to_pubkey(token_out).unwrap();
    let wallet: Pubkey = to_pubkey(wallet).unwrap();

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



fn to_pubkey(string: &str) -> Result<Pubkey, String>{

    match string.len(){

        32 => { 
            let mut string_array: [u8; 32] = [0; 32];
            string_array.copy_from_slice(string.as_bytes());
            Ok(Pubkey::from(string_array))
            }
        44 => {
            let pubkey = string.parse::<Pubkey>().expect("Invalid public key format");
            Ok(pubkey)
        }

        _ => {
            return Err("String doesn't match length".to_string());
        }
    }
    
}