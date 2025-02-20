use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize)]
struct ChampionsResponse {
    data: BTreeMap<String, Champion>,
}

#[derive(Deserialize)]
struct Champion {
    id: String,
    name: String,
    title: String,
    tags: Vec<String>,
    info: ChampionInfo,
}

#[derive(Deserialize, Debug)]
struct ChampionInfo {
    attack: u8,
    defense: u8,
    magic: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng: Pcg64 = Seeder::from("stripy zebra").into_rng();

    let champions =
        ureq::get("https://ddragon.leagueoflegends.com/cdn/15.4.1/data/en_US/champion.json")
            .call()?
            .body_mut()
            .read_json::<ChampionsResponse>()?
            .data;

    let champions2 = champions.values().choose_multiple(&mut rng, 24);

    let mut out = "<html><body><table>".to_string();

    for champion_row in champions2.chunks(6) {
        out += "<tr>";
        for champion in champion_row {
            println!("{}", champion.name);
            out += &format!(
                "<td><img src=\"https://ddragon.leagueoflegends.com/cdn/15.4.1/img/champion/{}.png\" /></td>",
                champion.id
            );
        }
        out += "</tr>";
    }

    out += "</table></body></html>";

    std::fs::write("./board.html", out)?;

    Ok(())
}
