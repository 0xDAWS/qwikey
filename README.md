# Qwikey

A fast and lightweight random key generator written in Rust

# Usage

Qwikey has two main output modes (hex and str)

### String Mode

String mode works by generating a string from the full set of ASCII characters

A range of options are available to customise the output from string mode. It's possible by using the following flags to use the complete ASCII set, or just a subset of your own choosing.  

| Option  | Type  | Required | Short | Long      | ASCII Characters                    |
|:-------:|:-----:|:--------:|:-----:|:---------:|:-----------------------------------:|
| length  | usize | yes      | -L    | --length  |                                     |
| lower   | bool  | no       | -l    | --lower   | abcdefghijklmnopqrstuvwxyz          |
| upper   | bool  | no       | -u    | --upper   | ABCDEFGHIJKLMNOPQRSTUVWXYZ          |
| digits  | bool  | no       | -d    | --digits  | 0123456789                          |
| special | bool  | no       | -s    | --special | !"#$%&\'()*+,-./:;<=>?@[\\]^_`{\|}~ |

### Hex Mode

Hex mode will generate a key whcih contains only valid hex digits

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

Random string of length 32 with auto/default options

```
qwikey -L 32
OR
qwikey --length 32
S<4F7:mGH:Zz@}Bd6Y5@dnUO;->7"uSC
```

Random string of length 32 with only lower and upper ASCII sets

```
# qwikey -ul -L 32
OR 
# qwikey --upper --lower --length 32
BpxGrVoeAjmHGvMekfoCKtNgBtUczJyD
```

Random string of length 32 using lower,upper and digit sets and using non ambiguous output option

```
# qwikey -uldn -L 32
OR
# qwikey --upper --lower --digits --no-lookalike --length 32
3N94Xdpd2WGnMrkTC42cZeXrbF3Ptzx6
```

Random string of length 32 using auto/default options and calculating entropy

```
# qwikey --entropy --length 32
5gYO=VHWr+V-p|PoaX3[iM#y:RH5vKf{

Entropy: 209 bits
```

# Notes

### Entropy Calculation

Password entropy is a measure what the password could have been based on how it was generated. This value (measured in bits) is related to how computationally difficult it would be for an attacker to bruteforce a password. Generally a higher entropy value means a better password but there are circumstances where this does not hold true. For example, it's possible to generate a random password of length 16, using the full ASCII character set which theoretically would have high entropy. 

But what if this 'random' password happened to be 'abcdefghijklmnop' or '0000000000000000'? This would be considered a very weak password, even though it has high entropy due to how it was generated. The entropy is purely a theoretical value, while useful, it's important to note that it alone will not guarantee password strength nor security.

Currently Qwikey does not allow repeating characters in the keys it generates, so '0000000000000000' would not be possible, but while extraordinarily unlikely, it would be possible to generate 'abcdefghijklmnop' or some other equally insecure passwords, easy to guess password. Should you happen to get unlucky to generate a key like this, please do not use it and simply run Qwikey again to get a new key. 

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
