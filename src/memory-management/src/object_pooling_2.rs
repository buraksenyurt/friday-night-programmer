use typed_arena::Arena;

#[derive(Debug)]
struct AssetServer {
    assets: Vec<String>,
    id: u32,
}

pub fn run() {
    println!("\nObject Pooling With typed_arena\n");

    let arena = Arena::new();

    let server_1 = arena.alloc(AssetServer {
        assets: vec![
            "player.png".to_string(),
            "tileSet.png".to_string(),
            "colors.png".to_string(),
        ],
        id: 1234,
    });

    let server_2 = arena.alloc(AssetServer {
        assets: vec![
            "human.png".to_string(),
            "brick.png".to_string(),
            "block.png".to_string(),
            "juice.jpg".to_string(),
            "intro.wav".to_string(),
        ],
        id: 1255,
    });

    println!(
        "Server {} has {} assets",
        server_1.id,
        server_1.assets.len()
    );
    println!(
        "Server {} has {} assets",
        server_2.id,
        server_2.assets.len(),
    );
}
