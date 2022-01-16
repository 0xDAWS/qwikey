use std::f64;

struct AsciiChars {
    pub lower: Vec<char>,
    pub upper: Vec<char>,
    pub digit: Vec<char>,
    pub spec : Vec<char>,
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

            spec: vec![
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

pub fn build_charset(chars: &mut Vec<char>,
                     arg_upper: bool, arg_lower: bool, 
                     arg_digit: bool, arg_spec:  bool,
                     arg_noambig: bool) {

    let mut set = AsciiChars::charset();

    if let (false,false,false,false) = 
           (arg_upper,arg_lower,arg_digit,arg_spec) { 
        chars.append(&mut set.lower);
        chars.append(&mut set.upper);
        chars.append(&mut set.digit);
        chars.append(&mut set.spec );

    } else {
        if arg_upper { 
            chars.append(&mut set.upper);
        } 
    
        if arg_lower {
            chars.append(&mut set.lower);
        } 
    
        if arg_digit {
            chars.append(&mut set.digit);
        } 
    
        if arg_spec {
            chars.append(&mut set.spec );
        }      
    }

    // If noambig mode true, then remove all ambiguous characters from chars
    if arg_noambig {
        for i in 0..set.ambig.len() {
            chars.retain(|&x| x != set.ambig[i]);
        }
    }
}

fn get_rounded_log2_as_u32(f: &f64) -> u32 {
    return f.log2().round() as u32;
}

pub fn calculate_entropy(l: usize, c: usize) -> u32 {
    let length     = l as f64;
    let charspace  = c as f64;
    let result     = charspace.powf(length);

    return get_rounded_log2_as_u32(&result);
}

pub fn print_entropy(e: &u32) {
    if e > &1024 {
        println!("\nWarning: The key length you have chosen has too many permutations for qwikey to accurately calculate password entropy");
    } else {
        println!("\nEntropy: {} bits",e);
    }
}