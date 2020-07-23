pub fn paper(version: String, dir: String) -> Result<(), Box<dyn std::error::Error>> {
	use std::fs;

	let url = format!("https://papermc.io/api/v1/paper/{}/latest/download", version);
	let mut response = reqwest::blocking::get(&url)?;

	let mut server_jar = fs::File::create(format!("{}/server.jar", dir))?;
	response.copy_to(&mut server_jar)?;

	// if user agrees to eula, make eula=true
	if dialoguer::Confirm::new().with_prompt("Do you agree to the EULA at https://account.mojang.com/documents/minecraft_eula?").interact()? {
		println!("You have agreed to the minecraft EULA.");
		fs::write(format!("{}/eula.txt", dir), "eula=true")?;
	} else {
		println!("ok bye");
	}
	Ok(())
}

/*
next is vanilla
bungeecord is going to be the hardest because it needs a separate function to create bungeecord servers with the bungeecord configuration already there
*/