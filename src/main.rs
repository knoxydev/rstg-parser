#![allow(warnings)]

use grammers_client::{Client, Config};
use grammers_session::Session;

use std::io::{self, Write};
use std::path::Path;
use dotenv::dotenv;
use std::env;


async fn get_chats(client: Client) -> Result<(), Box<dyn std::error::Error>>
{
	let mut dialogs = client.iter_dialogs();
	
	while let Some(dialog) = dialogs.next().await.unwrap()
	{		
		let chat = dialog.chat();
		println!("{} ({})", chat.name(), chat.id());
	}

	Ok(())
} 


#[tokio::main]
async fn main()
{
	dotenv().ok();

  // core.telegram.org/
  let mut api_id: i32 = 0;
  let api_hash: String = env::var("API_HASH").expect("api_hash - string");
  let phone: String = env::var("PHONE").expect("phone - string");

  {
  	let api_id_past = env::var("API_ID").expect("api_id - int");
  	api_id = api_id_past.parse().expect("api_id - int");
  }

  //Load or create a new session file
  let session = Session::load_file_or_create("session.session").unwrap();

	let config = Config
	{
		session,
		api_id,
		api_hash: api_hash.to_string(),
		params: Default::default(),
	};


	let mut client = Client::connect(config).await.unwrap();


	if client.is_authorized().await.unwrap() == false
	{
		println!("Sending login code to {}", &phone);
		let sent_code = client.request_login_code(&phone).await.unwrap();

		print!("Enter the code you received: ");
		io::stdout().flush().unwrap();

		let mut code = String::new();
		io::stdin().read_line(&mut code).unwrap();
		let code = code.trim();

		// You can also handle sign-up here if needed
		client.sign_in(&sent_code, code).await.unwrap();
		client.session().save_to_file("session.session").unwrap();
	}
	println!("Logged in as: {:?}", client.get_me().await.unwrap());


	get_chats(client).await.unwrap();
}





