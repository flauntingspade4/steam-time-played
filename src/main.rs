use serde::Deserialize;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::fs::read_to_string(".token")?;
    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    name = String::from(name.trim());
    let id;
    if name.len() == 17 && only_nums(&name) {
        id = name.parse::<u128>().unwrap();
    } else {
        let page: IdResponse = reqwest::blocking::get(&format!(
            "http://api.steampowered.com/ISteamUser/ResolveVanityURL/v0001/?key={}&vanityurl={}",
            token, name
        ))?
        .json()?;
        id = page.response.steamid.parse::<u128>().unwrap();
    }

    let page: GameResponse = reqwest::blocking::get(&format!("http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json", token, id))?.json()?;
    println!(
        "\nTotal time played in hours: {}",
        page.response.count_time() / 60.
    );

    Ok(())
}
#[derive(Deserialize)]
struct User {
    pub steamid: String,
}
#[derive(Deserialize)]
struct IdResponse {
    pub response: User,
}
#[derive(Deserialize)]
struct GameResponse {
    pub response: Games,
}
#[derive(Deserialize)]
struct Games {
    pub game_count: u64,
    pub games: Vec<Game>,
}
impl Games {
    fn count_time(&self) -> f64 {
        self.games.iter().fold(0., |a, x| &a + x.playtime_forever)
    }
}
#[derive(Deserialize)]
struct Game {
    playtime_forever: f64,
}
fn only_nums(i: &String) -> bool {
    i.chars().all(char::is_numeric)
}
