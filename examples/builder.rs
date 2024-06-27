use anyhow::Result;
use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;

#[allow(unused)]
#[derive(Debug, Builder)]
#[builder(build_fn(name = "_prv_build"))]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into, strip_option), default)]
    email: Option<String>,
    #[builder(setter(custom))]
    dob: DateTime<Utc>,

    #[builder(setter(skip))]
    age: u32,
    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

fn main() -> Result<()> {
    let user = User::build()
        .name("Alice")
        .email("alice@example.com")
        .skill("programming")
        .skill("debugging")
        .dob("1990-01-01T00:00:00Z")
        .build()?;
    println!("{:?}", user);
    Ok(())
}

impl User {
    pub fn build() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    // custom build method cover original method
    pub fn build(&self) -> Result<User> {
        let mut user = self._prv_build()?;
        user.age = (Utc::now().year() - user.dob.year()) as _;
        Ok(user)
    }
    pub fn dob(&mut self, value: &str) -> &mut Self {
        self.dob = DateTime::parse_from_rfc3339(value)
            .map(|dt| dt.with_timezone(&Utc))
            .ok();
        self
    }
}
