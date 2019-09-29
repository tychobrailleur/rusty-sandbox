use rand::{RngCore, ChaChaRng, FromEntropy};


/*

from https://docs.rs/rand/0.5.0/rand/prng/chacha/struct.ChaChaRng.html

A cryptographically secure random number generator that uses the
ChaCha algorithm.

ChaCha is a stream cipher designed by Daniel J. Bernstein [1], that we
use as an RNG. It is an improved variant of the Salsa20 cipher family,
which was selected as one of the "stream ciphers suitable for
widespread adoption" by eSTREAM [2].

ChaCha uses add-rotate-xor (ARX) operations as its basis. These are
safe against timing attacks, although that is mostly a concern for
ciphers and not for RNGs. Also it is very suitable for SIMD
implementation. Here we do not provide a SIMD implementation yet,
except for what is provided by auto-vectorisation.

With the ChaCha algorithm it is possible to choose the number of
rounds the core algorithm should run. The number of rounds is a
tradeoff between performance and security, where 8 rounds is the
minimum potentially secure configuration, and 20 rounds is widely used
as a conservative choice. We use 20 rounds in this implementation, but
hope to allow type-level configuration in the future.

We use a 64-bit counter and 64-bit stream identifier as in Benstein's
implementation [1] except that we use a stream identifier in place of
a nonce. A 64-bit counter over 64-byte (16 word) blocks allows 1 ZiB
of output before cycling, and the stream identifier allows 264 unique
streams of output per seed. Both counter and stream are initialized to
zero but may be set via set_word_pos and set_stream.

*/

fn main() {
    let mut ra = ChaChaRng::new_unseeded();

    // Always prints: 2917185654 2419978656
    println!("{:?}", ra.next_u32());
    println!("{:?}", ra.next_u32());

    let mut rb = ChaChaRng::from_entropy();
    println!("{:?}", rb.next_u32());
    println!("{:?}", rb.next_u32());
}
