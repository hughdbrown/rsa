# Purpose
Put all the parts of the RSA project together and show how public key cryptography works.

# Results
The program in main runs the professor's test cases and then shows the operation of the RSA workflow on random inputs.
```
-----
Run test cases
*** Public ***
Public key modulus:    17276041
Public key exponent e: 1652899

*** Private ***
Primes:    3449, 5009
λ(n): 2158448
d: 1034123

-----
msg = 1234567
encrypted = 9062798
decrypted = 1234567

-----
msg = 1337
encrypted = 14630698
decrypted = 1337

-----
msg = 8675309
encrypted = 14188033
decrypted = 8675309

-----
Run RSA environment
*** Public ***
Public key modulus:    64643273
Public key exponent e: 13352057

*** Private ***
Primes:    6781, 9533
λ(n): 16156740
d: 282893

-----
msg = 1234567
encrypted = 26574268
decrypted = 1234567

-----
msg = 1337
encrypted = 34313848
decrypted = 1337

-----
msg = 8675309
encrypted = 35546790
decrypted = 8675309
```
