use maud::html;
use std::fs::File;
use std::io::Write;

fn main() {
    let hero = "Can Claud 1 Dam";
    let intro = html! {
        h1 { (hero) ", Personal Page" }
        p {"Hello. My name is " b { (hero) } "... Welcome to my personal page."}
        pre {
            r#"
                This is a long text about,
                the all time famous fighter character,
                Can
                Claud
                1
                Dam.
            "#
        }

        #wishForm {
            p { "You can fill this form for your request" }

            form {
                table {
                    tr {
                        td {
                            label for="txtName" { "Your Name" }
                        }
                    }
                    tr {
                        td {
                            input type="text" name="txtName";
                        }
                    }
                    tr
                    {
                        td
                        {
                            label for="txtRequest" {"Your wish"}
                        }
                    }
                    tr{
                        td
                        {
                            input type="text" name="txtWish";
                        }
                    }
                    tr{
                        td{
                            input type="checkbox" name="chkRobot";
                            " "
                            label for="chkRobot" {"I am not a robot!" }
                        }
                    }
                    tr{
                        td{
                            input type="submit" name="btnSave";
                        }
                    }
                }
            }
        }
        #bottom{
            ul {
                li {
                    a href="about:blank" { "About" }
                }
                li {
                    a href="https://buraksenyurt.com" { "Web Page"}
                }
                li dir="rtl" {
                    "Scoobii Dubi Duuu "
                    small { "(yi haaa)"}
                }
            }
        }

    };
    let content = intro.into_string();
    println!("{}", content);

    let mut index_file = File::create("index.html").expect("Unable to create file");
    index_file
        .write_all(content.as_bytes())
        .expect("Unable to write data");
}
