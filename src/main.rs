mod argparser;
mod rng;
mod utils;

use rng::{get_random_buf,get_rand_string};
use utils::{build_charset,calculate_entropy,print_entropy};

// External
use hex::{encode,encode_upper};
use zeroize::Zeroize;

fn get_rhex(ucase: bool, l: usize, calc_e: bool) {
    let mut key  : Vec<u8> = vec![0;l/2]; 

    let res_random = get_random_buf(&mut key);

    if let Err(e) = res_random {
        panic!("Error: {}", &e);
    } 

    if ucase {
        println!("{}",encode_upper(&key));
    } else {
        println!("{}",encode(&key));
    }

    key.zeroize();

    if calc_e {
        let e = calculate_entropy(l/2, 256);
        print_entropy(&e);
    }
}

fn get_rstr(charset: &mut Vec<char>, l: usize, calc_e: bool) {
    let mut key  : Vec<char> = vec!['0';l]; 

    get_rand_string(&mut key, charset);
    
    println!("{}", &key.iter().collect::<String>());

    key.zeroize();

    if calc_e {
        let e = calculate_entropy(l, charset.len());
        print_entropy(&e);
    }
}

fn main() {
    let args = argparser::get_args();

    if args.length > 4096 {
        panic!("Error: length must be less than {}", 4096);
    }

    if let true = (args.hex || args.hexupper) {
        get_rhex(args.hexupper, args.length, args.entropy);

    } else {
        let mut charset: Vec<char> = Vec::new();

        build_charset(&mut charset, args.upper, args.lower, 
            args.digits, args.special, args.noambiguous);

        get_rstr(&mut charset, args.length, args.entropy);

        charset.zeroize();
    }

    
}
