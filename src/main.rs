use std::{env, fs};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use xxhash_rust::xxh3::xxh3_64;

const EMOJIS: [&str; 17] = [
    "<3",
    ":3",
    "uwu",
    "owo",
    "^^",
    ">w<",
    ">~<",
    ">.<",
    "^w^",
    "(◕ᴥ◕)",
    "ʕ•ᴥ•ʔ",
    "ʕ￫ᴥ￩ʔ",
    "(*^ω^)",
    "(◕‿◕✿)",
    "(*^.^*)",
    "(つ✧ω✧)つ",
    "(/ =ω=)/"
];

const PHRASES: [&str; 8] = [
    "*snuzzews*",
    "*nuzzwes youw chest*",
    "*wicks uw neck*",
    "*paws*",
    "*puwws*",
    "*meows*",
    "*snugs cwosew*",
    "*pounces on u*"
];

const SIMPLE_REPLACEMENTS: [(&str, &str); 10] = [
    ("wove", "wuv"),
    ("nice", "nyaice"),
    ("what", "wut"),
    ("you", "u"),
    ("the", "da"),
    ("hewwo", "hewo"),
    ("cat", "neko"),
    ("kitty", "neko"),
    ("cute", "kawaii"),
    ("hi", "hii"),
];

fn uwuify(input: &str) -> String {
    let input = input.to_lowercase();
    let input = input.as_str();
    let input_bytes = input.as_bytes();

    let seed = xxh3_64(input_bytes);
    let mut rng = StdRng::seed_from_u64(seed);

    let input_length = input.len();
    let mut result = String::with_capacity(input_length);

    result.push_str(EMOJIS[rng.gen_range(0..EMOJIS.len())]);
    result.push(' ');

    let mut i = 0;
    while i < input_length {
        if input_bytes[i] == b'r' || input_bytes[i] == b'l' {
            result.push('w');
            i += 1;
            continue;
        }

        for (from, to) in SIMPLE_REPLACEMENTS {
            if input.match_single(i, from) {
                result.push_str(to);
                i += from.len();
                continue;
            }
        }

        if input.match_single(i, "aww") && (i + 3 >= input_length || input_bytes[i + 3] == b'w' || input_bytes[i + 3] == b' ' || input_bytes[i + 3] == b'\n') {
            result.push_str("uwu");
            i += 3;
            continue;
        }

        if input.match_single(i, "awesome") {
            result.push_str("uwu");
            i += 3;
            continue;
        }

        const NYA_NEEDLES: [&str; 5] = ["na", "ne", "ni", "no", "nu"];
        if input.match_any(i, &NYA_NEEDLES) {
            result.push_str("ny");
            i += 1;
            continue;
        }

        if input_bytes[i] == b'.' && (i != 0 && input_bytes[i - 1] != b'.') {
            let mut j = i;
            let meets_condition = loop {
                if j >= input_length {
                    break true;
                }
                if input_bytes[j] != b'.' {
                    if input_bytes[j] != b' ' && input_bytes[j] != b'\n' {
                        break false;
                    }
                    break true;
                }
                j += 1;
            };

            if meets_condition {
                let s = if rng.gen_bool(0.5) { EMOJIS[rng.gen_range(0..EMOJIS.len())] } else { PHRASES[rng.gen_range(0..PHRASES.len())] };
                result.push(' ');
                result.push_str(s);
                result.push('.');
                i += 1;
                continue;
            }
        }

        if input_bytes[i] == b',' && (i + 1 >= input_length || input_bytes[i + 1] == b' ' || input_bytes[i + 1] == b'\n') {
            let s = PHRASES[rng.gen_range(0..PHRASES.len())];
            result.push(' ');
            result.push_str(s);
            result.push(',');
            i += 1;
            continue;
        }

        if input_bytes[i] == b'!' && (i + 1 >= input_length || input_bytes[i + 1] == b' ' || input_bytes[i + 1] == b'\n' || input_bytes[i + 1] == b'!') {
            result.push_str("!!");
            i += 1;
            continue;
        }

        if input_bytes[i] == b'?' && (i + 1 >= input_length || input_bytes[i + 1] == b' ' || input_bytes[i + 1] == b'\n' || input_bytes[i + 1] == b'?') {
            result.push_str(if rng.gen_bool(0.5) { "?!" } else { "!?" });
            i += 1;
            continue;
        }

        if input.match_single(i, "wo") && (i == 0 || input_bytes[i - 0] != b'o') && (i + 1 >= input_length || input_bytes[i + 1] != b'w') {
            result.push_str("owo");
            i += 2;
            continue;
        }

        result.push(char::from(input_bytes[i]));
        i += 1;
    }

    result.push(' ');
    result.push_str(PHRASES[rng.gen_range(0..PHRASES.len())]);

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: uwuifier_rust input.txt output.txt");
        return;
    }

    let content = fs::read_to_string(&args[1]).expect("could not read file");

    let uwuified_content = uwuify(&content);

    fs::write(&args[2], uwuified_content).expect("could not write to file");
}

pub trait Match {
    fn match_single(&self, index: usize, needle: &str) -> bool;

    fn match_any(&self, index: usize, needles: &[&str]) -> bool;
}
impl Match for &str {
    #[inline(always)]
    fn match_single(&self, index: usize, needle: &str) -> bool {
        if index + needle.len() > self.len() {
            return false;
        }

        self[index..].starts_with(needle)
    }

    #[inline(always)]
    fn match_any(&self, index: usize, needles: &[&str]) -> bool {
        for &needle in needles {
            if index + needle.len() > self.len() { continue; }
            if self[index..].starts_with(needle) {
                return true;
            }
        }

        false
    }
}
