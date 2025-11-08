struct MenuState;
struct PlayingState;
struct PausedState;
struct GameOverState;

struct GameLoop<State = MenuState> {
    state: PhantomData<State>,
}

impl Default for GameLoop<MenuState> {
    fn default() -> Self {
        GameLoop { state: PhantomData }
    }

    fn start(self) -> GameLoop<PlayingState> {
        println!("Starting the game...");
        GameLoop { state: PhantomData }
    }
}

fn main() {}
