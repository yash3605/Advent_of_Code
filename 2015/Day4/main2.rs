use md5;
use regex::Regex;

fn main() {
    let secret = "bgvyzdsv";
    let re = Regex::new(r"^0{6,}").unwrap();
    let mut counter: u64 = 1;

    loop {
        let input = format!("{}{}", secret, counter);
        let digest = format!("{:x}", md5::compute(input));

        if re.is_match(&digest) {
            println!("Found! Counter: {}, Digest: {:?}", counter, digest);
            break;
        }
        counter += 1;
    }
}

/*
[dependencies]
"md5" = "0.7"
"regex" = "1"
*/
