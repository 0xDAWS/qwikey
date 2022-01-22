use getrandom::{getrandom, Error};
use hex::{encode, encode_upper};
use rand::{thread_rng, Rng};
use zeroize::Zeroize;

use crate::argparser::Args;
use crate::utils::{build_charset, calculate_entropy, print_entropy};

fn gen_random_buf(buf: &mut Vec<u8>) -> Result<(), Error> {
    getrandom(buf)?;
    Ok(())
}

fn gen_random_str(charset: &[char], l: usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = thread_rng();
    let mut ctr = 0;
    let mut key = String::with_capacity(l);

    while ctr < l {
        let idx = rng.gen_range(0..charset.len());

        // Check key[ctr-1] for a repeating character
        if ctr == 0 {
            key.push(charset[idx]);
            ctr += 1;
        } else if key.chars().nth(ctr - 1).unwrap() == charset[idx] {
            continue;
        } else {
            key.push(charset[idx]);
            ctr += 1;
        }
    }

    Ok(key)
}

pub fn get_str_key(args: &Args) {
    let mut charset: Vec<char> = Vec::new();
    build_charset(&mut charset, args);

    match gen_random_str(&charset, args.length) {
        Ok(mut k) => {
            println!("{}", &k);

            k.zeroize();

            if args.entropy {
                print_entropy(&calculate_entropy(args.length, charset.len()));
            }
        }
        Err(e) => {
            eprintln!("Error: {}", &e);
        }
    }
}

pub fn get_hex_key(args: &Args) {
    let mut key: Vec<u8> = vec![0; args.length / 2];

    if let Err(e) = gen_random_buf(&mut key) {
        eprintln!("Error: {}", &e);
        return;
    }

    if args.hexupper {
        println!("{}", encode_upper(&key));
    } else {
        println!("{}", encode(&key));
    }

    key.zeroize();

    if args.entropy {
        print_entropy(&calculate_entropy(args.length / 2, u8::MAX.into()));
    }
}
