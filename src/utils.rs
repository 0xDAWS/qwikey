use std::f64;
use crate::argparser::Args;

struct AsciiChars {
    pub lower: Vec<char>,
    pub upper: Vec<char>,
    pub digit: Vec<char>,
    pub specl: Vec<char>,
    pub ambig: Vec<char>
}

impl AsciiChars {
    pub fn charset() -> Self {
        AsciiChars {
            lower: vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 
            'h', 'i', 'j', 'k', 'l', 'm', 'n', 
            'o', 'p', 'q', 'r', 's', 't', 'u', 
            'v', 'w', 'x', 'y', 'z'],

            upper: vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 
            'H', 'I', 'J', 'K', 'L', 'M', 'N',
            'O', 'P', 'Q', 'R', 'S', 'T', 'U', 
            'V', 'W', 'X', 'Y', 'Z'],

            digit: vec![
            '0', '1', '2', '3', '4', '5', '6', 
            '7', '8', '9'],

            specl:  vec![
            '!', '"', '#', '$', '%', '&', '(', 
            ')', '*', '+', ',', '-', '.', '/', 
            ':', ';', '<', '=', '>', '?', '@', 
            '[', '\\', ']', '^', '_', '`', '{', 
            '|', '}', '~'], 

            ambig: vec![
            '0','O','o','l','1','i','I','|','!']
        }
    }
}

pub fn build_charset(chars: &mut Vec<char>, args: &Args) {

    let mut set = AsciiChars::charset();

    if let (false,false,false,false) = 
           (args.upper,args.lower,args.digits,args.special) { 
        chars.append(&mut set.lower);
        chars.append(&mut set.upper);
        chars.append(&mut set.digit);
        chars.append(&mut set.specl);

    } else {
        if args.upper { 
            chars.append(&mut set.upper);
        } 
    
        if args.lower {
            chars.append(&mut set.lower);
        } 
    
        if args.digits {
            chars.append(&mut set.digit);
        } 
    
        if args.special {
            chars.append(&mut set.specl);
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
    let length       = l as f64;
    let chars_count  = c as f64;
    let permutations = chars_count.powf(length);
    return permutations.log2().round() as u32;
}

pub fn print_entropy(e: &u32) {
    if e > &1024 {
        println!("\nWarning: The key length you have chosen has too many permutations for qwikey to accurately calculate password entropy");
    } else {
        println!("\nEntropy: {} bits",e);
    }
}