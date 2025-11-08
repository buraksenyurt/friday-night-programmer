#![allow(dead_code)]

use std::marker::PhantomData;
use std::mem;

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
}

impl<State> GameLoop<State> {
    fn change<NextState>(&self) -> GameLoop<NextState> {
        GameLoop { state: PhantomData }
    }
    fn reset(&self) -> Self {
        Self { state: PhantomData }
    }
    fn get_state(&self) -> PhantomData<State> {
        self.state
    }
}

impl GameLoop<MenuState> {
    fn play(&self) -> GameLoop<PlayingState> {
        println!("Playing...");
        self.change::<PlayingState>()
    }
}

impl GameLoop<PlayingState> {
    fn pause(&self) -> GameLoop<PausedState> {
        println!("Game paused...");
        self.change::<PausedState>()
    }
    fn loose(&self) -> GameLoop<GameOverState> {
        println!("Game over...");
        self.change::<GameOverState>()
    }
}

impl GameLoop<GameOverState> {
    fn go_to_menu(&self) -> Self {
        println!("Going to menu");
        self.reset()
    }
}

impl GameLoop<PausedState> {
    fn go_on(&self) -> GameLoop<PlayingState> {
        println!("Playing...");
        self.change::<PlayingState>()
    }
}

fn main() {
    let flow = GameLoop::default().play();

    println!(
        "Size of Gameloop is {} bytes and the current state is {:?}",
        mem::size_of_val(&flow),
        flow.get_state()
    );

    let flow = flow.pause().go_on().loose().go_to_menu();
    println!(
        "Size of Gameloop is {} bytes and the current state is {:?}",
        mem::size_of_val(&flow),
        flow.get_state()
    );
}
