use c2pa::ManifestStore;
use std::env;
use std::error::Error;
use std::process;
use base64;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ManifestThumbs {
    thumbnail: Option<String>,
    thumbnails: Vec<String>
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide file");
        process::exit(0x0100);
    }

    let file_path = &args[1];
    let manifest_store = ManifestStore::from_file(file_path)?;
    let manifest = manifest_store.get_active().unwrap();
    let ingredients = manifest.ingredients();
    let mut thumbs = ManifestThumbs { thumbnail: None, thumbnails: Vec::new()};
    if let Some((format, data)) = manifest.thumbnail() {
        thumbs.thumbnail = Some(format!("data:{};charset=utf-8;base64,{}", format, base64::encode(data.to_vec())));
    }

    for i in 0..ingredients.len() {
        if let Some((format, data)) = ingredients[i].thumbnail() {
            thumbs.thumbnails.push(format!("data:{};charset=utf-8;base64,{}", format, base64::encode(data.to_vec())));
        }
    }

    println!("{}", serde_json::to_string(&thumbs).unwrap());
    Ok(())
}
