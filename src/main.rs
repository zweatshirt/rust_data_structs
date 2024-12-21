mod vectors;
mod hashmaps;
mod stacks_queues;

fn main() {
    vectors::make_vecs();
    vectors::iter_vec();
    hashmaps::make_maps();
    hashmaps::map_with_cap();
    hashmaps::map_challenge();
    stacks_queues::stacks();
    stacks_queues::queues();
}
