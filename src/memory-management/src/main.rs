mod arena_alloc;
mod boxing;
mod cow_samples;
mod niche_option;
mod object_pooling;
mod object_pooling_2;
mod object_pooling_refactored;
mod zero_cost_abstraction;

fn main() {
    cow_samples::run();

    niche_option::run();

    arena_alloc::run();

    boxing::run();

    zero_cost_abstraction::run();

    object_pooling::run();

    object_pooling_2::run();

    object_pooling_refactored::run();
}
