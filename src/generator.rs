use crate::biomenoise::{BiomeNoise, NetherNoise, EndNoise, initBiomeNoise, setBiomeSeed};
use crate::biomes::{Dimension};
use crate::layers::{getVoronoiSHA};

enum GeneratorFlag {
    LARGE_BIOMES, //            = 0x1,
    NO_BETA_OCEAN, //     = 0x2,
    FORCE_OCEAN_VARIANTS,  //    = 0x4,
}

impl GeneratorFlag {
    fn value(&self) -> u32 {
        match *self {
            GeneratorFlag::LARGE_BIOMES => 1,
            GeneratorFlag::NO_BETA_OCEAN => 2,
            GeneratorFlag::FORCE_OCEAN_VARIANTS => 4,
        }
    }
}

struct Generator {
    mc: i32,
    dim: i32,
    flags: u32,
    seed: u64,
    sha: u64,
    bn: BiomeNoise, 
    nn: NetherNoise, 
    en: EndNoise, 
}

pub fn setupGenerator(mut g: Generator, mc: i32, flags: u32) {
    g.mc = mc;
    g.dim = Dimension::DIM_UNDEF.value(); // TODO: Créer DIM_UNDEF 
    g.flags = flags;
    g.seed = 0;
    g.sha = 0;

    initBiomeNoise(&mut g.bn, mc);

}

pub fn applySeed(mut g: Generator, dim: i32, seed: u64) {
    g.dim = dim;
    g.seed = seed;
    g.sha = 0;

    if dim == Dimension::DIM_OVERWORLD.value() {
        setBiomeSeed(&g.bn, seed, (g.flags & GeneratorFlag::LARGE_BIOMES.value()) as i32);
    }
    // else if dim == Dimension::DIM_NETHER.value() {
    //     setNetherSeed(&g.nn, seed);
    // }
    // else if dim == Dimension::DIM_END.value() {
    //     setEndSeed(&g.en, g.mc, seed);   
    // }

    g.sha = getVoronoiSHA(seed);
    

}


