use std::{
    fs::{self, File},
    io::Write,
};

use puzzle_formats::FPuzzlesFormat;

fn main() -> std::io::Result<()> {
    fs::create_dir_all("generated")?;
    let mut value_file = File::create("generated/json_value.txt")?;
    let mut struct_file = File::create("generated/json_struct.txt")?;

    let result = puzzle_formats::decode_url(include_str!("../assets/puzzleid.txt").trim());

    value_file.write_all(serde_json::to_string_pretty(&result)?.as_bytes())?;
    struct_file
        .write_all(format!("{:#?}", serde_json::from_value::<FPuzzlesFormat>(result)).as_bytes())?;

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
