use serde::Deserialize;

fn main() {
    let (time, time2weeks) = {
        let token = std::fs::read_to_string(".token")
            .expect("You must have a file named '.token' in the same directory as this");

        let name = std::env::args().last().unwrap();

        let id = if name.len() == 17 && name.chars().all(char::is_numeric) {
            name.parse::<u128>().unwrap()
        } else {
            let page: IdResponse = ureq::get(&format!(
            "http://api.steampowered.com/ISteamUser/ResolveVanityURL/v0001/?key={}&vanityurl={}",
            token, name
        ))
            .call()
            .unwrap()
            .into_json()
            .unwrap_or_else(|_| panic!("Failed to get user details based off given name {}", name));

            page.response.steamid.parse::<u128>().unwrap()
        };

        let page: GameResponse = ureq::get(&format!("
		http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&include_played_free_games=true&format=json",
		token, id
	))
	.call().unwrap()
	.into_json().unwrap_or_else(|_| panic!("The given id {} was either incorrect, or the associated account was private", id));

        page.response.count_time()
    };

    println!(
		"\nTotal time played in hours: {:.2}, of which {:.2}% was played in the past two weeks ({:.2} hours)\nTotal time played in days: {:.2}\n",
		time / 60.,
		(time2weeks / time) * 100.,
		time2weeks / 60.,
		time / 1440.
	);
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
    fn count_time(self) -> (f64, f64) {
        self.games.into_iter().fold((0., 0.), |(a, p), x| {
            (a + x.playtime_forever, p + x.playtime_2weeks.unwrap_or(0.))
        })
    }
}
