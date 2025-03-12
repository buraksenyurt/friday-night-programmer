mod arena_alloc;
mod boxing;
mod cow_samples;
mod niche_option;
mod zero_cost_abstraction;

fn main() {
    cow_samples::run();

    niche_option::run();

    arena_alloc::run();

    boxing::run();

    zero_cost_abstraction::run();
}
