#![feature(portable_simd)]
use tinyrand::{Probability, Rand, RandLim, Seeded, StdRand};
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

// very uwu function
pub fn uwuify(input: &str) -> String {
    let input_ptr = input.as_ptr();

    let seed = xxh3_64(input.as_bytes()); // uh this is so fast??? how???
    let mut rng = StdRand::seed(seed);

    let input_length = input.len();
    let mut result = String::with_capacity(input_length); // allocating 2x seems to make it slower

    result.push_str(EMOJIS[rng.next_lim(EMOJIS.len())]);
    result.push(' ');

    let mut i = 0;
    unsafe { // i love unsafe
        while i < input_length {
            let current_ptr = input_ptr.add(i);
            let current = (*current_ptr).to_ascii_lowercase();

            if current == b'r' || current == b'l' {
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

            // cursed code
            // "aww" that is either followed by w, a space or a \n
            if input.match_single(i, "aww") && (i + 3 >= input_length || {
                let current_offset_3 = (*current_ptr.offset(3)).to_ascii_lowercase();
                current_offset_3 == b'w' || current_offset_3 == b' ' || current_offset_3 == b'\n'
            }) {
                result.push_str("uwu");
                i += 3;
                continue;
            }

            // awesome
            if input.match_single(i, "awesome") {
                result.push_str("uwu");
                i += 3;
                continue;
            }

            // maybe custom function???? they differ by 1 letter
            // "n" followed by a vowel
            const NYA_NEEDLES: [&str; 5] = ["na", "ne", "ni", "no", "nu"];
            if input.match_any(i, &NYA_NEEDLES) {
                result.push_str("ny");
                i += 1;
                continue;
            }

            // uh this is a dot that is not preceeded by a dot and the last dot in the following dots is followed by a space or \n
            // idk its fucked
            if current == b'.' && (i != 0 && *current_ptr.offset(-1) != b'.') {
                let mut j = i;
                let meets_condition = loop {
                    if j >= input_length {
                        break true;
                    }
                    let current = *input_ptr.add(j); // case doesnt matter so no to lower
                    if current != b'.' {
                        if current != b' ' && current != b'\n' {
                            break false;
                        }
                        break true;
                    }
                    j += 1;
                };

                if meets_condition {
                    let s = if rng.next_bool(Probability::new(0.5)) { EMOJIS[rng.next_lim(EMOJIS.len())] } else { PHRASES[rng.next_lim(PHRASES.len())] };
                    result.push(' ');
                    result.push_str(s);
                    result.push('.');
                    i += 1;
                    continue;
                }
            }

            // below we compare to spaces and \ns so we dont need to lowercase
            // maybe i can put all +1s into a local variable but idk how to do it without doing fucked up owos

            // colon followed by a space, \n, or end of string
            if current == b',' && (i + 1 >= input_length || *current_ptr.offset(1) == b' ' || *current_ptr.offset(1) == b'\n') {
                let s = PHRASES[rng.next_lim(PHRASES.len())];
                result.push(' ');
                result.push_str(s);
                result.push(',');
                i += 1;
                continue;
            }

            // an exclamation mark that is is followed by a space, \n, end of string, or another question mark
            if current == b'!' && (i + 1 >= input_length || *current_ptr.offset(1) == b' ' || *current_ptr.offset(1) == b'\n' || *current_ptr.offset(1) == b'!') {
                result.push_str("!!");
                i += 1;
                continue;
            }

            // a question mark that is is followed by a space, \n, end of string, or another question mark
            if current == b'?' && (i + 1 >= input_length || *current_ptr.offset(1) == b' ' || *current_ptr.offset(1) == b'\n' || *current_ptr.offset(1) == b'?') {
                result.push_str(if rng.next_bool(Probability::new(0.5)) { "?!" } else { "!?" });
                i += 1;
                continue;
            }

            // "wo" that is not preceeded by "o" and not followed by "w"
            if input.match_single(i, "wo") && (i == 0 || (*current_ptr.offset(-1)).to_ascii_lowercase() != b'o') && (i + 2 >= input_length || (*current_ptr.offset(2)).to_ascii_lowercase() != b'w') {
                result.push_str("owo");
                i += 2;
                continue;
            }

            result.push(char::from(current));
            i += 1;
        }
    }

    result.push(' ');
    result.push_str(PHRASES[rng.next_lim(PHRASES.len())]);

    result
}

// a trait to be nice
pub trait Match {
    fn match_single(&self, index: usize, needle: &str) -> bool;

    fn match_any(&self, index: usize, needles: &[&str]) -> bool;
}

impl Match for &str {
    #[inline(always)] // keep yourself inlined 😊❤️
    fn match_single(&self, index: usize, needle: &str) -> bool {
        // SAFETY: trust me bro 🫡
        let self_length = self.len();
        let needle_length = needle.len();
        if index + needle_length > self_length {
            return false;
        }

        let self_bytes = self.as_ptr();
        let needle_bytes = needle.as_ptr();

        unsafe {
            for i in 0..needle_length {
                if *self_bytes.add(index + i) != *needle_bytes.add(i) {
                    return false;
                }
            }
            true
        }
    }

    #[inline(always)]
    fn match_any(&self, index: usize, needles: &[&str]) -> bool {
        for &needle in needles {
            if self.match_single(index, needle) {
                return true;
            }
        }

        false
    }
}
