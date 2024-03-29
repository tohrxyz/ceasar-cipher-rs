use std::env;

// defined alphabet
// '-' starts, to be a placeholder of 0th element
const UPPERCASE_ALPHABET: &str = "-ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE_ALPHABET: &str = "-abcdefghijklmnopqrstuvwxyz";
const R_UPPERCASE_ALPHABET: &str = "ZYXWVUTSRQPONMLKJIHGFEDCBA-";
const R_LOWERCASE_ALPHABET: &str = "zyxwvutsrqponmlkjihgfedcba-";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No args. Input text string and number to be shifted by.");
        return;
    }
    // println!("Args: {:#?}", &args);
    let input: &str = &args[1];
    let nshift: i32 = args[2].parse().unwrap();
    // println!("input: {}", &input);
    // println!("nshift: {}", &nshift);

    // println!("uppercase: {} ", &UPPERCASE_ALPHABET);
    // println!("lowercase: {}", &LOWERCASE_ALPHABET);
    let cleaned_input = input.replace(" ", "");
    if !is_valid_alphabet_char(&cleaned_input) {
        return;
    }

    let encrypted_string = encrypt_shift_string(&cleaned_input, &nshift);
    decrypt_shift_string(&encrypted_string, &nshift);

}

fn is_valid_alphabet_char(input: &str) -> bool {
    let mut is_valid = true;
    for c in input.chars() {
        if UPPERCASE_ALPHABET.contains(c) || LOWERCASE_ALPHABET.contains(c) {
            continue;
        } else {
            println!("Invalid char '{}' , only uppercase and lowercase english alphabet!", c);
            is_valid = false;
            break;
        }
    }
    return is_valid;
}

fn encrypt_shift_string<'a>(input: &'a str, nshifted: &'a i32) -> String {
    let ics = input.char_indices();
    let mut shifted_text: String = String::from("");
    for (index, c) in ics.clone().into_iter() {
        if UPPERCASE_ALPHABET.contains(c) {
            let x =  match UPPERCASE_ALPHABET.char_indices().find(|&x| x.1 == c.clone()) {
                Some(some) => {
                    some.0
                },
                None => { 
                    println!("No x usize");
                    0
                 },
            };
            let next_pos = match (x + *nshifted as usize) > 26 {
                true => {
                    (x + *nshifted as usize) - 26
                },
                false => x + *nshifted as usize,
            };
            let char_at_shifted_position = UPPERCASE_ALPHABET.char_indices().nth(next_pos);
            match char_at_shifted_position {
                Some(char_tuple) => {
                    shifted_text.push_str(&char_tuple.1.to_string());
                    // println!("string [{}]: {}", index, &shifted_text);
                },
                None => {
                    println!("No char at shifted position [{}]: '{}'", index, c);
                    println!("x: {} Multiple: {}", x, (x + x.next_multiple_of(*nshifted as usize)));
                    println!("{:?}", UPPERCASE_ALPHABET.chars().nth(x));
                },
            }
        }
        if LOWERCASE_ALPHABET.contains(c) {
            let x =  match LOWERCASE_ALPHABET.char_indices().find(|&x| x.1 == c.clone()) {
                Some(some) => {
                    some.0
                },
                None => { 
                    println!("No x usize");
                    0
                 },
            };
            let next_pos = match (x + *nshifted as usize) > 26 {
                true => {
                    (x + *nshifted as usize) - 26
                },
                false => x + *nshifted as usize,
            };
            let char_at_shifted_position = LOWERCASE_ALPHABET.char_indices().nth(next_pos);
            match char_at_shifted_position {
                Some(char_tuple) => {
                    shifted_text.push_str(&char_tuple.1.to_string());
                    // println!("string [{}]: {}", index, &shifted_text);
                },
                None => {
                    println!("No char at shifted position [{}]: '{}'", index, c);
                    println!("x: {} Multiple: {}", x, (x + x.next_multiple_of(*nshifted as usize)));
                    println!("{:?}", LOWERCASE_ALPHABET.chars().nth(x));
                },
            }
        }
    }
    println!("Input string: {}", input);
    println!("Encrypted string: {} by {}", &shifted_text, nshifted);
    let result = shifted_text.clone();
    return result;
}

fn decrypt_shift_string(input: &str, nshifted: &i32) -> String {
    let ics = input.char_indices();
    let mut shifted_text: String = String::from("");
    for (index, c) in ics.clone().into_iter() {
        if R_UPPERCASE_ALPHABET.contains(c) {
            let x =  match R_UPPERCASE_ALPHABET.char_indices().find(|&x| x.1 == c.clone()) {
                Some(some) => {
                    some.0
                },
                None => { 
                    println!("No x usize");
                    0
                 },
            };
            let next_pos = match (x + *nshifted as usize) > 26 {
                true => {
                    (x + *nshifted as usize) - 26
                },
                false => x + *nshifted as usize,
            };
            let char_at_shifted_position = R_UPPERCASE_ALPHABET.char_indices().nth(next_pos);
            match char_at_shifted_position {
                Some(char_tuple) => {
                    shifted_text.push_str(&char_tuple.1.to_string());
                    // println!("string [{}]: {}", index, &shifted_text);
                },
                None => {
                    println!("No char at shifted position [{}]: '{}'", index, c);
                    println!("x: {} Multiple: {}", x, (x + x.next_multiple_of(*nshifted as usize)));
                    println!("{:?}", R_UPPERCASE_ALPHABET.chars().nth(x));
                },
            }
        }
        if R_LOWERCASE_ALPHABET.contains(c) {
            let x =  match R_LOWERCASE_ALPHABET.char_indices().find(|&x| x.1 == c.clone()) {
                Some(some) => {
                    some.0
                },
                None => { 
                    println!("No x usize");
                    0
                 },
            };
            let next_pos = match (x + *nshifted as usize) > 26 {
                true => {
                    (x + *nshifted as usize) - 26
                },
                false => x + *nshifted as usize,
            };
            let char_at_shifted_position = R_LOWERCASE_ALPHABET.char_indices().nth(next_pos);
            match char_at_shifted_position {
                Some(char_tuple) => {
                    shifted_text.push_str(&char_tuple.1.to_string());
                    // println!("string [{}]: {}", index, &shifted_text);
                },
                None => {
                    println!("No char at shifted position [{}]: '{}'", index, c);
                    println!("x: {} Multiple: {}", x, (x + x.next_multiple_of(*nshifted as usize)));
                    println!("{:?}", R_LOWERCASE_ALPHABET.chars().nth(x));
                },
            }
        }
    }
    println!("Input string: {}", input);
    println!("Decrypted string: {} by {}", &shifted_text, nshifted);
    let result = shifted_text.clone();
    return result;
}