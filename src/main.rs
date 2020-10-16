use serde::Deserialize;
fn main() -> Result<(), std::io::Error> {
    let token = std::fs::read_to_string(".token")?;

    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        let mut input_name = String::new();
        println!("Enter Steam username: ");
        std::io::stdin().read_line(&mut input_name)?;
        input_name = String::from(input_name.trim());
        args.push(input_name);
    }
    let name = &args[1];
    let id = if name.len() == 17 && only_nums(&name) {
        name.parse::<u128>().unwrap()
    } else {
        let page: IdResponse = ureq::get(&format!(
            "http://api.steampowered.com/ISteamUser/ResolveVanityURL/v0001/?key={}&vanityurl={}",
            token, name
        ))
        .call()
        .into_json_deserialize()
        .expect(&format!(
            "Failed to get user details based off given name {}",
            name
        ));

        page.response.steamid.parse::<u128>().unwrap()
    };

    let page: GameResponse = ureq::get(&format!("
		http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&include_played_free_games=true&format=json",
		token, id
	))
	.call()
	.into_json_deserialize().expect(&format!("The given id {} was either incorrect, or the associated account was private", id));

    let (time, time2weeks) = page.response.count_time();

    println!(
		"\nTotal time played in hours: {:.2}, of which {:.2}% was played in the past two weeks ({:.2} hours)\nTotal time played in days: {:.2}\n",
		time,
		(time2weeks / time) * 100.,
		time2weeks,
		time / 24.
	);

    Ok(())
}
#[derive(Deserialize)]
struct IdResponse {
    pub response: User,
}
#[derive(Deserialize)]
struct User {
    pub steamid: String,
}
#[derive(Deserialize)]
struct GameResponse {
    pub response: Games,
}
#[derive(Deserialize)]
struct Games {
    pub games: Vec<Game>,
}
#[derive(Deserialize)]
struct Game {
    playtime_2weeks: Option<f64>,
    playtime_forever: f64,
}
impl Games {
    fn count_time(&self) -> (f64, f64) {
        (
            self.games.iter().fold(0., |a, x| a + x.playtime_forever) / 60.,
            self.games
                .iter()
                .fold(0., |a, x| a + x.playtime_2weeks.unwrap_or(0.))
                / 60.,
        )
    }
}
fn only_nums(i: &str) -> bool {
    i.chars().all(char::is_numeric)
}
