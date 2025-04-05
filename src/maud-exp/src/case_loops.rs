use maud::html;
use std::fmt::Display;

pub fn create() -> String {
    let players = vec![
        Player {
            name: "Can Cey Rambo".to_string(),
            score: 89,
            level: Level::Beginner,
        },
        Player {
            name: "Trinity".to_string(),
            score: 92,
            level: Level::Pro,
        },
        Player {
            name: "Niyo".to_string(),
            score: 98,
            level: Level::Elit,
        },
        Player {
            name: "Maria Terasova".to_string(),
            score: 75,
            level: Level::Elit,
        },
    ];
    let content = html! {
        #summary ."col-sm-2"
        {
            p { h1 {   "There are " (players.len()) " players in this room"  } }
            table{
                tr{
                    th{
                        "Player"
                    }
                    th{
                        "Score"
                    }
                    th{
                        "Level"
                    }
                }
                @for player in players {
                    tr{
                        td{
                            (player.name)
                        }
                        td{
                            @if player.score > 95 {
                                b { (player.score) }
                            } @else {
                                { (player.score) }
                            }
                        }
                        td{
                            @match player.level {
                                Level::Beginner => {
                                    p style="color:green" { (player.level) }
                                },
                                Level::Pro=>{
                                    p style="color:blue" { (player.level) }
                                },
                                Level::Elit => {
                                    p style="color:red" .bright-red { (player.level) }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    content.into_string()
}

struct Player {
    name: String,
    score: u8,
    level: Level,
}

enum Level {
    Beginner,
    Pro,
    Elit,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Beginner => write!(f, "Beginner"),
            Level::Pro => write!(f, "Professional"),
            Level::Elit => write!(f, "Elit"),
        }
    }
}
