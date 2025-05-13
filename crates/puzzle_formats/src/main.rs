use std::{
    fs::{self, File},
    io::Write,
};

use puzzle_formats::{fpuzzles::FPuzzlesFormat, url::decode_url};

fn main() -> std::io::Result<()> {
    fs::create_dir_all("generated")?;
    let mut value_file = File::create("generated/json_value.txt")?;
    let mut struct_file = File::create("generated/json_struct.txt")?;
    let mut value_out_file = File::create("generated/json_value_out.txt")?;

    let value = decode_url(include_str!("../assets/puzzleid.txt").trim());

    value_file.write_all(serde_json::to_string_pretty(&value)?.as_bytes())?;

    let result = serde_json::from_value::<FPuzzlesFormat>(value)?;
    struct_file.write_all(format!("{result:#?}").as_bytes())?;

    value_out_file.write_all(serde_json::to_string_pretty(&result)?.as_bytes())?;

    println!("Files written");

    // let client = reqwest::ClientBuilder::new()
    //     .redirect(reqwest::redirect::Policy::none())
    //     .build()
    //     .unwrap();

    // let response = client
    //     .request(reqwest::Method::GET, "https://tinyurl.com/2b5dwuy3")
    //     .send()
    //     .;

    Ok(())
}
