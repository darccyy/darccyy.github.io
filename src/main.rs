use unreact::prelude::*;

// Where the site is hosted
const URL: &str = "https://darccyy.github.io/";

fn main() -> Result<(), Error> {
    let config = Config {
        strict: true,
        ..Config::default()
    };

    let mut app = Unreact::new(config, is_dev(), URL)?;

    app
        // Index page
        .index("homepage", object! { secret: "Hello!" })
        // Complete app
        .run()?;

    // Only prints if NOT in dev mode
    println!("Compiled for production.");
    Ok(())
}
