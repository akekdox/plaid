use plaid::{SandboxCreatePublicTokenRequest, SupportedProduct, SandboxCreatePublicTokenRequestOptions};


#[tokio::main]
 async fn main() -> Result<(), Box<dyn std::error::Error>> {

     // or use \`plaid::Client::from_env()?\`
     let client = plaid::Client::new(
         "your_client_id",
         String::from("client_secret_i_guess"),
         plaid::Environment::Sandbox,
     );

    let randomProducts:Vec<SupportedProduct> = Vec::new();

    let tokenoptions = SandboxCreatePublicTokenRequestOptions{webhook : Option::from(String::from("client_secret_i_guess")),
        override_username : String::from("client_secret_i_guess"),
        override_password : String::from("client_secret_i_guess")};

    let test = SandboxCreatePublicTokenRequest {
        institution_id:String::from("institution_id"),
        initial_products:randomProducts,
        options:tokenoptions};

     // TODO: use the Link flow instead; https://plaid.com/docs/link/#link-flow
     let public_token = client.sandbox_create_public_token(&test).await?.public_token;

     let access_token = client
         .exchange_public_token(&public_token)
         .await?
         .access_token;

     let _accounts = client.accounts(&access_token).await?.accounts;

    client.bank_transfer_cancel("some transfer id");

     Ok(())
 }