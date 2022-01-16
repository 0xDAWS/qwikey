use getrandom::{getrandom,Error}; 
use rand::{thread_rng, Rng};

pub fn get_random_buf(buf: &mut Vec<u8>) -> Result<(), Error> {
    getrandom(buf)?;
    Ok(())
}

pub fn get_rand_string(key   : &mut Vec<char>,
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