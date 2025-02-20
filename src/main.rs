use std::collections::BTreeMap;

use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

use rocket::response::Redirect;
use serde::{Deserialize, Serialize};

use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
struct ChampionsResponse {
    data: BTreeMap<String, Champion>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Champion {
    id: String,
    name: String,
    title: String,
    tags: Vec<String>,
    info: ChampionInfo,
    partype: String,

    #[serde(default)]
    skins: Vec<Skin>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChampionInfo {
    attack: u8,
    defense: u8,
    magic: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Skin {
    num: u8,
    name: String,
}

#[get("/")]
fn bare_index() -> Redirect {
    let word1 = random_word::gen_len(rand::random_range(3..=6), random_word::Lang::En)
        .expect("unable to generate word");
    let word2 = random_word::gen_len(rand::random_range(3..=6), random_word::Lang::En)
        .expect("unable to generate word");

    Redirect::to(format!("/{word1}_{word2}"))
}

#[get("/<seed>")]
fn index(seed: &str) -> Template {
    let champions = get_random_champs(seed).expect("unable to fetch champions");

    let grid = champions.chunks(6).collect::<Vec<_>>();

    Template::render("index", context!( seed: seed, champ_grid: grid ))
}

fn get_random_champs(seed: &str) -> Result<Vec<(Champion, Skin)>, Box<dyn std::error::Error>> {
    let mut rng: Pcg64 = Seeder::from(seed).into_rng();

    let champions =
        ureq::get("https://ddragon.leagueoflegends.com/cdn/15.4.1/data/en_US/champion.json")
            .call()?
            .body_mut()
            .read_json::<ChampionsResponse>()?
            .data;

    let chosen_champions = champions.values().choose_multiple(&mut rng, 24);

    let mut res = Vec::new();

    for champ in chosen_champions {
        let fetched_champ = ureq::get(format!(
            "https://ddragon.leagueoflegends.com/cdn/15.4.1/data/en_US/champion/{}.json",
            champ.id
        ))
        .call()?
        .body_mut()
        .read_json::<ChampionsResponse>()?
        .data;

        let v = fetched_champ.values().next().cloned().unwrap();
        let skin = v.skins.choose(&mut rng).unwrap().clone();

        res.push((v, skin))
    }

    Ok(res)
}

#[launch]
fn rocket() -> _ {
    // let mut out = "<html><body><table>".to_string();
    // out += "<tr>";
    // out += &format!(
    //     "<td><img src=\"https://ddragon.leagueoflegends.com/cdn/15.4.1/img/champion/{}.png\" /></td>",
    //     champion.id
    // );
    // out += "</tr>";

    // out += "</table></body></html>";

    // std::fs::write("./board.html", out)?;

    rocket::build()
        .mount("/", routes![bare_index, index])
        .attach(Template::fairing())
}
