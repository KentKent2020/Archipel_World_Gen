use crate::rng::{rotr32};

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const B: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];


pub fn getVoronoiSHA(seed: u64) -> u64 {
    let mut m: [u32; 64] = [0; 64];
    let mut a: [u32; 8] = [0; 8];

    m[0] = (seed as u32).to_be();
    m[1] = (seed >> 32).to_be() as u32;
    m[2] = 0x80000000;
    m[15] = 0x00000040;

    for i in 16..64 {
        m[i] = m[i - 7].wrapping_add(m[i - 16])
            .wrapping_add(rotr32(m[i - 15], 7) ^ rotr32(m[i - 15], 18) ^ (m[i - 15] >> 3))
            .wrapping_add(rotr32(m[i - 2], 17) ^ rotr32(m[i - 2], 19) ^ (m[i - 2] >> 10));
    }

    a.copy_from_slice(&B);

    for i in 0..64 {
        let x = a[7].wrapping_add(K[i]).wrapping_add(m[i])
            .wrapping_add(rotr32(a[4], 6) ^ rotr32(a[4], 11) ^ rotr32(a[4], 25))
            .wrapping_add((a[4] & a[5]) ^ (!a[4] & a[6]));

        let y = rotr32(a[0], 2) ^ rotr32(a[0], 13) ^ rotr32(a[0], 22)
            .wrapping_add((a[0] & a[1]) ^ (a[0] & a[2]) ^ (a[1] & a[2]));

        a[7] = a[6];
        a[6] = a[5];
        a[5] = a[4];
        a[4] = a[3].wrapping_add(x);
        a[3] = a[2];
        a[2] = a[1];
        a[1] = a[0];
        a[0] = x.wrapping_add(y);
    }

    a[0] = a[0].wrapping_add(B[0]);
    a[1] = a[1].wrapping_add(B[1]);

    ((a[0] as u64).to_be() | ((a[1] as u64).to_be() << 32))
}