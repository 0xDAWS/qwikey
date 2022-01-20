# Qwikey

A random key generator written in Rust

# Usage

Qwikey has two output modes

### ASCII Mode

ASCII mode works by generating a random string from the full set of ASCII characters

A range of options are available to customise the output from this mode. By default all ASCII characters are used, or you can set flags for which sets you would like to use.

| Option  | Type  | Required | Short | Long      | ASCII Characters                    |
|:-------:|:-----:|:--------:|:-----:|:---------:|:-----------------------------------:|
| length  | usize | yes      | -L    | --length  |                                     |
| lower   | bool  | no       | -l    | --lower   | abcdefghijklmnopqrstuvwxyz          |
| upper   | bool  | no       | -u    | --upper   | ABCDEFGHIJKLMNOPQRSTUVWXYZ          |
| digits  | bool  | no       | -d    | --digits  | 0123456789                          |
| special | bool  | no       | -s    | --special | !"#$%&\'()*+,-./:;<=>?@[\\]^_`{\|}~ |

### Hex Mode

Hex mode will generate a random key which contains only valid hex digits in either upper or lower case

| Option        | Type  | Required | Short | Long        | Hex Digits       |
|:-------------:|:-----:|:--------:|:-----:|:-----------:|:----------------:|
| length        | usize | yes      | -L    | --length    |                  |
| hex lowercase | bool  | no       | -x    | --hex       | 0123456789abcdef |
| hex uppercase | bool  | no       | -X    | --hex-upper | 0123456789ABCDEF |

### Other Options

| Option             | Type | Required | Short | Long           |
|:------------------:|:----:|:--------:|:-----:| -------------- |
| non ambiguous      | bool | no       | -n    | --no-lookalike |
| entropy calculator | bool | no       | -e    | --entropy      |

# Examples

Random string of length=32 with auto/default options

```
qwikey -L 32
OR
qwikey --length 32
S<4F7:mGH:Zz@}Bd6Y5@dnUO;->7"uSC
```

Random string of length=32 with only lower and upper ASCII sets

```
# qwikey -ul -L 32
OR 
# qwikey --upper --lower --length 32
BpxGrVoeAjmHGvMekfoCKtNgBtUczJyD
```

Random string of length=32 using lower,upper and digit sets and using non ambiguous output option

```
# qwikey -uldn -L 32
OR
# qwikey --upper --lower --digits --no-lookalike --length 32
3N94Xdpd2WGnMrkTC42cZeXrbF3Ptzx6
```

Random string of length=32 using auto/default options and calculating entropy

```
# qwikey -eL 32
OR
# qwikey --entropy --length 32
5gYO=VHWr+V-p|PoaX3[iM#y:RH5vKf{

Entropy: 209 bits
```

Hex string (lowercase) of length=32

```
# qwikey -xL 32
OR
# qwikey --hex --length 32
2540864e15967bea384949cffe720b6a
```

Hex string (uppercase) of length=32 and calculating entropy

```
# qwikey -XeL 32
OR
# qwikey --hex-upper --entropy --length 32
B7ECB92A10128DCDCC50C92A65628246

Entropy: 128 bits
```

You may notice that the entropy of a 32 character hex string is calculated to be ~128 bits, whereas the entropy of an ascii password of the same length using all sets is ~209 bits. This is because the hex key (in this case) is generated by using a 16 byte array, and each byte in the array is repesented by a 2 character hex digit from 0x00-0xFF to make the 32 character key. 

So even though they are both the same length (32 characters), how a key is generated has an effect on its entropy value. There is more information on entropy calculation below.  

# Notes

### Entropy Calculation & Password Strength

Password entropy is a measure of what the password could have been based on how it was generated. This value (measured in bits) is directly related to how computationally difficult it would be for an attacker to bruteforce a password. Generally, a higher entropy value means a better password but there are circumstances where this does not hold true. For example, we could generate a random password of length=16, using the full ASCII character set which theoretically would have reasonably good entropy. 

```
# qwikey -eL 32
x}Cg#kAT&_UHCO{w

Entropy: 105 bits
```

But what if this 'random' password happened to be `abcdefgh12345678` or `0000000000000000`? 

Passwords like the ones above would be considered extremely weak in the real world, even if they are generated randomly. This is because entropy is really just a theoretical value, which while useful, will not guarantee password strength nor security.

Currently Qwikey does not allow repeating characters in the keys it generates, so `0000000000000000` should not be possible, but while extraordinarily unlikely, it would be possible to generate `abcdefgh12345678` or some other equally insecure, easy to guess password. Should you happen to get unlucky to generate a key like this, please do not use it and simply run Qwikey again to get a new key. 

The steps to find the entropy value in bits are

- Calculate the possible permutations of charset (c) given length (l)

- Take the log2 of the permutations to get the value in bits

Qwikey calculates the entropy using the following function:

```rust
fn calculate_entropy(l: usize, c: usize) -> u32 {
    let length       = l as f64;
    let charspace    = c as f64;
    let permutations = chars_count.powf(length);
    return permutations.log2().round() as u32;
}
```

Once the entropy is known, we can determine with precision how many guesses it would take to exhaust every possible permutation.

For example if we generate a key with entropy=64 bits, it would take `2^64` guesses to exhaust all combinations in a brute-force attack, and on average an attacker will have to try at least `(2^64)/2` guesses to find the correct key.
