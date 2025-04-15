mod case_intro;
mod case_loops;
mod case_partial;

use maud::Markup;
use rocket::{Error, get, routes};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Error> {
    /*
        Test etmek için http://127.0.0.1:8000/home
    */
    rocket::build().mount("/", routes![index]).launch().await?;

    Ok(())
    // let content = case_one::create();
    // let content = case_loops::create();
    // let content = case_partial::create();
    // println!("{}", content);

    // let mut index_file = File::create("index.html").expect("Unable to create file");
    // index_file
    //     .write_all(content.as_bytes())
    //     .expect("Unable to write data");
}

#[get("/home")]
fn index() -> Markup {
    case_partial::create()
}
