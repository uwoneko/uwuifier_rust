use std::fs;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline(always)]
fn match_single(input: &str, index: usize, needle: &str) -> bool {
    if index + needle.len() > input.len() {
        return false;
    }

    input[index..].starts_with(needle)
}

fn my_replace(input: String, from: &str, to: &str) -> String {
    let mut i = 0;
    let input_length = input.len();
    let input_bytes = input.as_bytes();
    let mut result = String::with_capacity(input_length);
    while i < input_length {
        if match_single(&input, i, from) {
            result.push_str(to);
            i += from.len();
            continue;
        }

        result.push(char::from(input_bytes[i]));
        i += 1;
    }
    result
}

fn std_replace(input: String, from: &str, to: &str) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    for (start, part) in input.match_indices(from) {
        result.push_str(unsafe { input.get_unchecked(last_end..start) });
        result.push_str(to);
        last_end = start + part.len();
    }
    result.push_str(unsafe { input.get_unchecked(last_end..input.len()) });
    result
}

fn benchmark(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("uwu");
        group.bench_function("my", |b| {
            let input = fs::read_to_string("input.txt").unwrap();
            b.iter(|| {
                my_replace(black_box(input.clone()), "r", "w")
            })
        });
        group.bench_function("std", |b| {
            let input = fs::read_to_string("input.txt").unwrap();
            b.iter(|| {
                std_replace(black_box(input.clone()), "r", "w")
            })
        });
    }
    {
        let mut group = c.benchmark_group("uwu_multiple");
        group.bench_function("my", |b| {
            let input = fs::read_to_string("input.txt").unwrap();
            b.iter(|| {
                let mut i = 0;
                let input_length = input.len();
                let input_bytes = input.as_bytes();
                let mut result = String::with_capacity(input_length);
                while i < input_length {
                    if match_single(&input, i, "r") {
                        result.push_str("w");
                        i += 1;
                        continue;
                    }
                    if match_single(&input, i, "aww") {
                        result.push_str("uwu");
                        i += 3;
                        continue;
                    }
                    if match_single(&input, i, "wo") {
                        result.push_str("owo");
                        i += 2;
                        continue;
                    }

                    result.push(char::from(input_bytes[i]));
                    i += 1;
                }
                result
            })
        });
        group.bench_function("std", |b| {
            let input = fs::read_to_string("input.txt").unwrap();
            b.iter(|| {
                let result = std_replace(black_box(input.clone()), "r", "w");
                let result = std_replace(result, "aww", "uwu");
                let result = std_replace(result, "wo", "owo");
                result
            })
        });
    }
}

criterion_group!(benchy, benchmark);
criterion_main!(benchy);
