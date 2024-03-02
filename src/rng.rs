pub struct Xoroshiro {
    pub lo: u64,
    pub hi: u64,
}

pub fn xSetSeed(xr: &mut Xoroshiro, value: u64) {
    const XL: u64 = 0x9e3779b97f4a7c15;
    const XH: u64 = 0x6a09e667f3bcc909;
    const A: u64 = 0xbf58476d1ce4e5b9;
    const B: u64 = 0x94d049bb133111eb;

    let mut l = value ^ XH;
    let mut h = l.wrapping_add(XL);
    l = (l ^ (l >> 30)).wrapping_mul(A);
    h = (h ^ (h >> 30)).wrapping_mul(A);
    l = (l ^ (l >> 27)).wrapping_mul(B);
    h = (h ^ (h >> 27)).wrapping_mul(B);
    l ^= l >> 31;
    h ^= h >> 31;

    xr.lo = l;
    xr.hi = h;
}

pub fn xNextLong(xr: &mut Xoroshiro) -> u64 {
    let l = xr.lo;
    let h = xr.hi;
    let n = (l.wrapping_add(h)).rotate_left(17).wrapping_add(l);
    let mut new_l = l.rotate_left(49) ^ h ^ (h << 21);
    let new_h = h.rotate_left(28);

    xr.lo = new_l;
    xr.hi = new_h;

    n
}

fn next(seed: &mut u64, bits: i32) -> i32 {
    *seed = (*seed * 0x5deece66d + 0xb) & ((1u64 << 48) - 1);
    ((*seed as i64) >> (48 - bits)) as i32
}

pub fn nextInt(seed: &mut u64, n: i32) -> i32 {
    let mut bits: i32;
    let mut val: i32;
    let m = n - 1;

    if (m & n) == 0 {
        let x = n as u64 * next(seed, 31) as u64;
        return (x as i64 >> 31) as i32;
    }

    loop {
        bits = next(seed, 31);
        val = bits % n;
        if bits - val + m >= 0 {
            break;
        }
    }

    val
}

pub fn xNextInt(xr: &mut Xoroshiro, n: u32) -> i32 {
    let mut r: u64 = (xNextLong(xr) & 0xFFFFFFFF) * n as u64;
    if r < n as u64 {
        while r < (((!n).wrapping_add(1)) % n) as u64 {
            r = (xNextLong(xr) & 0xFFFFFFFF ) * n as u64;
        }
    }
    (r >> 32) as i32
}

pub fn xNextDouble(xr: &mut Xoroshiro) -> f64 {
   (xNextLong(xr) >> (64-53)) as f64 * 1.1102230246251565E-16
}

pub fn lerp(part: f64, from: f64, to: f64) -> f64 {
    from + part * (to - from)
}

pub fn rotr32(a: u32, b: u8) -> u32 {
    (a >> b) | (a << (32-b))
}