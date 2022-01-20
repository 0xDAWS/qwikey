use getrandom::{getrandom,Error}; 
use rand::{thread_rng, Rng};
use hex::{encode,encode_upper};
use zeroize::Zeroize;

use crate::utils::{build_charset,calculate_entropy,print_entropy};
use crate::argparser::Args;

fn gen_random_buf(buf: &mut Vec<u8>) -> Result<(), Error> {
    getrandom(buf)?;
    Ok(())
}

fn gen_random_str(key   : &mut Vec<char>,
                  chars : &mut Vec<char>) {   
               
    let mut rng = thread_rng();
    let mut ctr = 0;

    while ctr < key.len() {
        let idx = rng.gen_range(0..chars.len());

        // Check key[ctr-1] for a repeating character
        if ctr==0 {
            key[ctr]  = chars[idx];
            ctr      += 1;
        } else {
            if key[ctr-1] == chars[idx] {
                continue;

            } else {
                key[ctr]  = chars[idx];
                ctr      += 1;
            }
        }
    }
}

pub fn get_str_key(args: &Args) {

    let mut charset: Vec<char> = Vec::new();
    build_charset(&mut charset, args);

    let mut key  : Vec<char> = vec!['0';args.length]; 
    gen_random_str(&mut key, &mut charset);

    println!("{}", &key.iter().collect::<String>());

    key.zeroize();

    if args.entropy {
        print_entropy(&calculate_entropy(args.length, charset.len()));
    } else {
        return;
    }
}

pub fn get_hex_key(args: &Args) {
    let mut key  : Vec<u8> = vec![0;args.length/2]; 

    if let Err(e) = gen_random_buf(&mut key) {
        panic!("Error: {}", &e);
    } 

    if args.hexupper {
        println!("{}",encode_upper(&key));
    } else {
        println!("{}",encode(&key));
    }

    key.zeroize();

    if args.entropy {
        print_entropy(&calculate_entropy(args.length/2, 256));
    } else {
        return;
    }
}