#[cfg(feature = "generate")]
use {
    std::io::Write,
    std::{collections::BTreeMap, fs::File},
};

#[cfg(feature = "generate")]
#[tokio::main]
async fn main() {
    let json = reqwest::get("https://calendrier.api.gouv.fr/jours-feries/metropole.json")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // dbg!(&json.unwrap());
    let v: BTreeMap<chrono::NaiveDate, String> = serde_json::from_str(&json).unwrap();
    let v = v
        .into_keys()
        .map(|f| f.to_string())
        .map(|f| format!("\"{f}\",\n"))
        .collect::<String>();

    let v = format!(
        r#"
pub const JOURS_FERIES: &[&str] = &[
    {v}
];
        "#
    );

    let output = std::env::var("OUTPUT").unwrap();
    let mut file = File::create(output).unwrap();
    file.write_all(v.as_bytes()).unwrap();
}

#[cfg(not(feature = "generate"))]
fn main() {}
