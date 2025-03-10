mod cow_samples;
mod niche_option;
mod arena_alloc;

fn main() {
    cow_samples::run();
    
    niche_option::run();

    arena_alloc::run();
}
