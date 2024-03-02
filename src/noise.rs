use crate::rng::{xNextLong, Xoroshiro, xNextInt, xNextDouble};

pub struct PerlinNoise {
    d: [u8; 256+1],
    h2: u8,
    a: f64,
    b: f64,
    c: f64,
    amplitude: f64,
    lacunarity: f64,
    d2: f64,
    t2: f64,
}

pub struct OctaveNoise {
    octcnt: i32,
    octaves: PerlinNoise,
}

pub struct DoublePerlinNoise {
    amplitude: f64,
    octA: OctaveNoise,
    octB: OctaveNoise,
}

fn xPerlinInit(noise: &mut PerlinNoise, xr: &mut Xoroshiro) {
    let mut i: i32 = 0;

    noise.a = xNextDouble(xr) * 256.0;
    noise.b = xNextDouble(xr) * 256.0;
    noise.c = xNextDouble(xr) * 256.0;
    noise.amplitude = 1.0;
    noise.lacunarity = 1.0;

    let idx: &mut [u8; 257] = &mut noise.d;
    for i in 0..256 {
        idx[i as usize] = i as u8;
    }
    for i in 0..256 {
        let j = xNextInt(xr, 256 - i) + i as i32;
        let n = idx[i as usize];
        idx[i as usize] = idx[j as usize];
        idx[j as usize] = n;
    }
    idx[256] = idx[0];
    let i2 = noise.b.floor();
    let d2 = noise.b - i2;
    noise.h2 = i2 as u8;
    noise.d2 = d2;
    noise.t2 = d2 * d2 * d2 * (d2 * (d2 * 6.0 - 15.0) + 10.0);
}

fn xOctaveInit(
    noise: &mut OctaveNoise,
    xr: &mut Xoroshiro,
    octaves: &mut [PerlinNoise],
    amplitudes: &[f64],
    omin: i32,
    len: i32,
    nmax: i32,
) -> i32 {
    const MD5_OCTAVE_N: [(u64, u64); 13] = [
        (0xb198de63a8012672, 0x7b84cad43ef7b5a8),
        (0x0fd787bfbc403ec3, 0x74a4a31ca21b48b8),
        (0x36d326eed40efeb2, 0x5be9ce18223c636a),
        (0x082fe255f8be6631, 0x4e96119e22dedc81),
        (0x0ef68ec68504005e, 0x48b6bf93a2789640),
        (0xf11268128982754f, 0x257a1d670430b0aa),
        (0xe51c98ce7d1de664, 0x5f9478a733040c45),
        (0x6d7b49e7e429850a, 0x2e3063c622a24777),
        (0xbd90d5377ba1b762, 0xc07317d419a7548d),
        (0x53d39c6752dac858, 0xbcd1c5a80ab65b3e),
        (0xb4a24d7a84e7677b, 0x023ff9668e89b5c4),
        (0xdffa22b534c5f608, 0xb9b67517d3665ca9),
        (0xd50708086cef4d7c, 0x6e1651ecc7f43309),
    ];

    const LACUNA_INI: [f64; 13] = [
        1.0, 0.5, 0.25, 1.0 / 8.0, 1.0 / 16.0, 1.0 / 32.0, 1.0 / 64.0, 1.0 / 128.0, 1.0 / 256.0, 1.0 / 512.0, 1.0 / 1024.0,
        1.0 / 2048.0, 1.0 / 4096.0,
    ];

    const PERSIST_INI: [f64; 10] = [
        0.0, 1.0, 2.0 / 3.0, 4.0 / 7.0, 8.0 / 15.0, 16.0 / 31.0, 32.0 / 63.0, 64.0 / 127.0, 128.0 / 255.0, 256.0 / 511.0,
    ];

    assert!(-omin >= 0 && -omin < LACUNA_INI.len() as i32 && len < PERSIST_INI.len() as i32 && len >= 0); //FIXME: Check if it crash the code

    let mut lacuna = LACUNA_INI[-omin as usize];
    let mut persist = PERSIST_INI[len as usize];
    let mut xlo = xNextLong(xr);
    let mut xhi = xNextLong(xr);
    let mut i = 0;
    let mut n = 0;

    while i < len && n != nmax {
        if amplitudes[i as usize] == 0.0 {
            i += 1;
            continue;
        }
        let mut pxr = Xoroshiro { lo: xlo ^ MD5_OCTAVE_N[(12 + omin + i) as usize].0, hi: xhi ^ MD5_OCTAVE_N[(12 + omin + i) as usize].1 };
        xPerlinInit(&mut octaves[n as usize], &mut pxr);
        octaves[n as usize].amplitude = amplitudes[i as usize] * persist;
        octaves[n as usize].lacunarity = lacuna;
        n += 1;
        i += 1;
        lacuna *= 2.0;
        persist *= 0.5;
    }

    noise.octaves = octaves[0];
    noise.octcnt = n;
    n
}



pub fn xDoublePerlinInit(noise: &mut DoublePerlinNoise, xr: &mut Xoroshiro, octaves: &mut [PerlinNoise], amplitudes: &[f64], omin: i32, len: i32, nmax: i32) -> i32 {
    let mut n = 0;
    let mut na = -1;
    let mut nb = -1;

    if nmax > 0 {
        na = (nmax + 1) >> 1;
        nb = nmax - na;
    }

    n += xOctaveInit(&mut noise.octA, xr, &mut [octaves[n as usize]], amplitudes, omin, len, na);
    n += xOctaveInit(&mut noise.octB, xr, &mut [octaves[(n + na) as usize]], amplitudes, omin, len, nb);

    let mut len = len as usize;
    for i in (0..len).rev() {
        if amplitudes[i] == 0.0 {
            len -= 1;
        } else {
            break;
        }
    }
    for i in 0..len {
        if amplitudes[i] == 0.0 {
            len -= 1;
        } else {
            break;
        }
    }

    static AMP_INI: [f64; 10] = [0.0, 5.0 / 6.0, 10.0 / 9.0, 15.0 / 12.0, 20.0 / 15.0, 25.0 / 18.0, 30.0 / 21.0, 35.0 / 24.0, 40.0 / 27.0, 45.0 / 30.0];
    noise.amplitude = AMP_INI[len];

    n
}