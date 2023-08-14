use gcd_lcm::{
    gcd,
    lcm,
};
use prng::Prng;

/// Calculate Carmichael's totient function λ(n) where n = p * q and p and q are prime.
/// λ(n) is the smallest value m where a**m ≡ 1 mod n for all a that are relatively prime to n.
///
/// https://en.wikipedia.org/wiki/RSA_(cryptosystem)#Key_generation
/// Compute λ(n), where λ is Carmichael's totient function.
/// Since n = pq, λ(n) = lcm(λ(p), λ(q)),
/// and since p and q are prime, λ(p) = φ(p) = p − 1,
/// and likewise λ(q) = q − 1.
/// Hence λ(n) = lcm(p − 1, q − 1).
///
/// https://en.wikipedia.org/wiki/Carmichael_function
/// https://en.wikipedia.org/wiki/Carmichael_number
pub fn totient(p: u64, q: u64) -> u64 {
    lcm(p - 1, q - 1) 
}

pub fn random_exponent(prng: &mut Prng, lambda: u64) -> u64 {
    loop {
        let e: u64 = prng.next_u64(3, lambda);
        if gcd(e, lambda) == 1 {
            return e;
        }
    }
}

pub fn inverse_mod(n: u64, modulus: u64) -> u64 {
    let mut x: u64 = 0;

    loop {
        let u = (x * modulus) % n;
        if u == 1 { return x; }
        let mut v = (n - u) / modulus;
        if u + (v * modulus) < n { v += 1; }
        x += v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totient_61_53() {
        let result = totient(61, 53);
        assert_eq!(780, result);
    }

    #[test]
    fn inverse_mod_780_173_413() {
        let result = inverse_mod(780, 17);
        assert_eq!(413, result);
    }

}
