use maud::{DOCTYPE, Markup, html};

fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title {  }
    }
}

fn body(title: &str, scores: &Vec<Score>) -> Markup {
    html! {
        h1 { "Last Exam Scores" }

        table {
            tr {
                th { "Id" }
                th { "Student" }
                th { "Point" }
                th { "Result" }
            }
            @for (idx,score) in scores.iter().enumerate() {
                tr {
                    td { (idx) }
                    td { (score.owner) }
                    td { (score.value) }
                    td {
                        @match score.value {
                            0..50 => "Fail",
                            50..60 =>"C",
                            60..70 =>"C+",
                            70..80 =>"B",
                            80..90 =>"B+",
                            90..=100 =>"A",
                            _ => ""
                        }
                    }
                }
            }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer {
            a href="https://buraksenyurt.com" { "For details" }
        }
    }
}

pub fn create() -> String {
    let title = "Last Exam Scores";

    let scores = vec![
        Score::new("Rodrigo".to_string(), 46),
        Score::new("Margarita".to_string(), 54),
        Score::new("Anita".to_string(), 75),
        Score::new("Herge".to_string(), 81),
        Score::new("Lukas".to_string(), 35),
        Score::new("Strazel".to_string(), 92),
    ];
    html! {
        (header(title))
        (body(title,&scores))
        (footer())
    }
    .into_string()
}

struct Score {
    owner: String,
    value: u8,
}

impl Score {
    fn new(owner: String, value: u8) -> Self {
        Score { owner, value }
    }
}
