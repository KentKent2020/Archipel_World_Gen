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