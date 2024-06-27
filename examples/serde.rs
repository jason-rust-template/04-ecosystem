use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    skills: Vec<String>,
}

fn main() -> Result<()> {
    let user = User {
        name: "John".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };
    let json = serde_json::to_string(&user)?;
    println!("json: {}", json);

    let user1 = serde_json::from_str(&json)?;
    println!("user1: {:?}", user1);

    assert_eq!(user, user1);

    Ok(())
}
