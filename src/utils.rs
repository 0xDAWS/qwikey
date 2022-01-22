use crate::argparser::Args;
use std::f64;

struct AsciiChars {
    pub lower: Vec<char>,
    pub upper: Vec<char>,
    pub digit: Vec<char>,
    pub specl: Vec<char>,
    pub ambig: Vec<char>,
}

impl AsciiChars {
    pub fn charset() -> Self {
        AsciiChars {
            lower: ('a'..='z').collect(),
            upper: ('A'..='Z').collect(),
            digit: ('0'..='9').collect(),
            specl: vec![
                '!', '"', '#', '$', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';',
                '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
            ],

            ambig: vec!['0', 'O', 'o', 'l', '1', 'i', '|', 'G', '6', 'B', '8'],
        }
    }
}

pub fn build_charset(chars: &mut Vec<char>, args: &Args) {
    let set = AsciiChars::charset();

    if let (false, false, false, false) = (args.upper, args.lower, args.digits, args.special) {
        chars.extend(set.upper.iter());
        chars.extend(set.lower.iter());
        chars.extend(set.digit.iter());
        chars.extend(set.specl.iter());
    } else {
        if args.upper {
            chars.extend(set.upper.iter());
        }

        if args.lower {
            chars.extend(set.lower.iter());
        }

        if args.digits {
            chars.extend(set.digit.iter());
        }

        if args.special {
            chars.extend(set.specl.iter());
        }
    }

    // If noambig mode true, then remove all ambiguous characters from chars
    if args.noambiguous {
        for i in 0..set.ambig.len() {
            chars.retain(|&x| x != set.ambig[i]);
        }
    }
}

pub fn calculate_entropy(l: usize, c: usize) -> u32 {
    let length = l as f64;
    let chars_size = c as f64;
    let permutations = chars_size.powf(length);
    return permutations.log2().round() as u32;
}

pub fn print_entropy(e: &u32) {
    if e > &1024 {
        eprintln!("\nWarning: The key length you have chosen has too many permutations for qwikey to accurately calculate password entropy");
    } else {
        println!("\nEntropy: {} bits", e);
    }
}
