use prng::Prng;
use fast_exponentiation::fast_exp_mod;
use primality_test::find_prime;
use rsa::{
    totient,
    random_exponent,
    inverse_mod,
};

const MIN: u64 = 1000;
const MAX: u64 = 10000;
const TESTS: i32 = 25;

fn print_vars(n: u64, p: u64, q: u64, e: u64, d: u64, lambda: u64) {
    println!("*** Public ***");
    println!("Public key modulus:    {n}");
    println!("Public key exponent e: {e}");
    println!("");

    println!("*** Private ***");
    println!("Primes:    {p}, {q}");
    println!("λ(n): {lambda}");
    println!("d: {d}");
    println!("");
}

fn encrypt_decrypt(msg: u64, e: u64, d: u64, n: u64) {
    let cipher_text: u64 = fast_exp_mod(msg, e, n);
    let plain_text: u64 = fast_exp_mod(cipher_text, d, n);
    
    println!("-----");
    println!("msg = {msg}");
    println!("encrypted = {cipher_text}");
    println!("decrypted = {plain_text}");
    println!();
}

fn run_rsa_env(prng: &mut Prng) {
    println!("-----\nRun RSA environment");
    // Use find_prime to pick two random primes p and q between 1,000 and 10,000
    let p = find_prime(prng, MIN, MAX, TESTS);
    let q = find_prime(prng, MIN, MAX, TESTS);

    // Calculate the public key modulus n = p * q
    let n = p * q;

    // Use the totient function to calculate Carmichael’s totient λ_n for n
    let lambda = totient(p, q);

    // Use the random_exponent function to pick a random public key exponent e
    // in the range [3, λ_n) where gcd(e, λ_n) = 1
    let e: u64 = random_exponent(prng, lambda);

    // Use the inverse_mod function to find the inverse of e in the modulus λ_n. Call that inverse d
    let d: u64 = inverse_mod(lambda, e);

    print_vars(n, p, q, e, d, lambda);

    encrypt_decrypt(1234567, e, d, n);
    encrypt_decrypt(1337, e, d, n);
    encrypt_decrypt(8675309, e, d, n);
}

fn run_test_cases(p: u64, q: u64, e: u64) {
    println!("-----\nRun test cases");

    // Calculate the public key modulus n = p * q
    let n = p * q;

    // Use the totient function to calculate Carmichael’s totient λ_n for n
    let lambda = totient(p, q);

    // Use the inverse_mod function to find the inverse of e in the modulus λ_n. Call that inverse d
    let d: u64 = inverse_mod(lambda, e);

    print_vars(n, p, q, e, d, lambda);

    encrypt_decrypt(1234567, e, d, n);
    encrypt_decrypt(1337, e, d, n);
    encrypt_decrypt(8675309, e, d, n);
}

fn main() {
    let mut prng = Prng::new();

    run_test_cases(3449, 5009, 1652899);
    run_rsa_env(&mut prng);
}
