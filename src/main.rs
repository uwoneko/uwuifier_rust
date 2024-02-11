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
    "*snuzzews",
    "*nuzzwes youw chest*",
    "*wicks uw neck*",
    "*paws*",
    "*puwws*",
    "*meows*",
    "*snugs cwosew*",
    "*pounces on u*"
];

fn uwuify(input: &str) -> String {
    let mut result = String::from(input);

    result = result.replace("love", "luv")
        .replace("nice", "nyaice")
        .replace("what", "wut")
        .replace("you", "u")
        .replace("the", "da");

    result = {
        let mut new_result = String::new();
        let mut last_end = 0;
        for (start, part) in result.match_indices("aww") {
            let result_bytes = result.as_bytes();
            if start + part.len() >= result_bytes.len() { continue; }
            match result_bytes[start + part.len()] {
                b'w' => {},
                b' ' => {},
                _ => continue
            };

            new_result.push_str(unsafe { result.get_unchecked(last_end..start) });
            new_result.push_str("uwu");
            last_end = start + part.len();
        }
        new_result.push_str(unsafe { result.get_unchecked(last_end..result.len()) });
        new_result
    };

    result = {
        let mut new_result = String::new();
        let mut last_end = 0;
        for (start, part) in result.match_indices("awe") {
            let result_bytes = result.as_bytes();
            if start + part.len() >= result_bytes.len() { continue; }
            if start + part.len() + 4 >= result_bytes.len() { continue; }
            if &result_bytes[start + part.len()..start + part.len() + 4] != "some".as_bytes() {
                continue;
            }
            new_result.push_str(unsafe { result.get_unchecked(last_end..start) });
            new_result.push_str("uwu");
            last_end = start + part.len();
        }
        new_result.push_str(unsafe { result.get_unchecked(last_end..result.len()) });
        new_result
    };

    result = {
        let mut new_result = String::new();
        let mut last_end = 0;
        for (start, part) in result.match_indices("n") {
            let result_bytes = result.as_bytes();
            if start + part.len() >= result_bytes.len() { continue; }
            match &result_bytes[start + part.len()] {
                b'a' => {},
                b'e' => {},
                b'i' => {},
                b'o' => {},
                b'u' => {},
                _ => continue
            };

            new_result.push_str(unsafe { result.get_unchecked(last_end..start) });
            new_result.push_str("ny");
            last_end = start + part.len();
        }
        new_result.push_str(unsafe { result.get_unchecked(last_end..result.len()) });
        new_result
    };

    result = {
        let mut new_result = String::new();
        let mut last_end = 0;
        for (start, part) in result.match_indices(".") {
            let result_bytes = result.as_bytes();
            if start == 0 { continue };
            if result_bytes[start - 1] == b'.' { continue };
            if start + part.len() < result_bytes.len() {
                if result_bytes[start + part.len()] != b' ' { continue };
            }

            new_result.push_str(unsafe { result.get_unchecked(last_end..start) });
            new_result.push_str("uwu");
            last_end = start + part.len();
        }
        new_result.push_str(unsafe { result.get_unchecked(last_end..result.len()) });
        new_result
    };

    return result;
}

fn main() {
    println!("{}", uwuify("love nice what you the aww awwwww awesome na ne ni no nu"));
}
