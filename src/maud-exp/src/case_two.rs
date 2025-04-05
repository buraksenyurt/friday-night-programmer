use maud::html;

pub fn create() -> String {
    let players = vec![
        Player {
            name: "Can Cey Rambo".to_string(),
            score: 89,
        },
        Player {
            name: "Trinity".to_string(),
            score: 92,
        },
        Player {
            name: "Niyo".to_string(),
            score: 98,
        },
        Player {
            name: "Maria Terasova".to_string(),
            score: 75,
        },
    ];
    let content = html! {
        p { "There are " (players.len()) " players in this room"  }
        table{
            tr{
                th{
                    "Player"
                }
                th{
                    "Score"
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
                }
            }
        }
    };
    content.into_string()
}

struct Player {
    name: String,
    score: u8,
}
