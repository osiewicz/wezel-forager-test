use std::hint::black_box;

#[inline(never)]
pub fn expanded_workload(seed: u64) -> u64 {
    let mut value = seed;

    for round in 0..32 {
        value = mix_add(value, round);
        value = mix_rotate(value, round);
        value = mix_multiply(value, round);
        value = mix_xor(value, round);
        value = mix_subtract(value, round);
        value = mix_reverse(value, round);
    }

    black_box(value)
}

#[inline(never)]
fn mix_add(value: u64, round: u64) -> u64 {
    value.wrapping_add(round).wrapping_add(0x9e37_79b9)
}

#[inline(never)]
fn mix_rotate(value: u64, round: u64) -> u64 {
    value.rotate_left((round % 63 + 1) as u32)
}

#[inline(never)]
fn mix_multiply(value: u64, round: u64) -> u64 {
    value.wrapping_mul(round.wrapping_mul(2).wrapping_add(1))
}

#[inline(never)]
fn mix_xor(value: u64, round: u64) -> u64 {
    value ^ round.wrapping_mul(0x45d9_f3b)
}

#[inline(never)]
fn mix_subtract(value: u64, round: u64) -> u64 {
    value.wrapping_sub(round.rotate_left(7))
}

#[inline(never)]
fn mix_reverse(value: u64, round: u64) -> u64 {
    value.reverse_bits() ^ round
}

#[inline(never)]
pub fn second_expanded_workload(seed: u64) -> u64 {
    let mut value = seed;

    for round in 0..48 {
        value = widen_low(value, round);
        value = widen_high(value, round);
        value = shuffle_left(value, round);
        value = shuffle_right(value, round);
        value = fold_product(value, round);
        value = fold_sum(value, round);
    }

    black_box(value)
}

#[inline(never)]
fn widen_low(value: u64, round: u64) -> u64 {
    value.wrapping_add(round.wrapping_mul(0x517c_c1b7))
}

#[inline(never)]
fn widen_high(value: u64, round: u64) -> u64 {
    value.wrapping_add(round.rotate_right(11)).swap_bytes()
}

#[inline(never)]
fn shuffle_left(value: u64, round: u64) -> u64 {
    value.rotate_left((round % 31 + 1) as u32) ^ 0xa24b_aed4
}

#[inline(never)]
fn shuffle_right(value: u64, round: u64) -> u64 {
    value.rotate_right((round % 29 + 1) as u32) ^ 0x9fb2_1c65
}

#[inline(never)]
fn fold_product(value: u64, round: u64) -> u64 {
    value.wrapping_mul(round.wrapping_add(3)).count_ones() as u64 ^ value
}

#[inline(never)]
fn fold_sum(value: u64, round: u64) -> u64 {
    value
        .wrapping_add(value.count_zeros() as u64)
        .wrapping_sub(round)
}
