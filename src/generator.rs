enum GeneratorFlag {
    LARGE_BIOMES, //            = 0x1,
    NO_BETA_OCEAN, //     = 0x2,
    FORCE_OCEAN_VARIANTS,  //    = 0x4,
}

impl GeneratorFlag {
    fn value(&self) -> i32 {
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
    bn: BiomeNoise, // TODO: Créer BiomeNoise
    nn: NetherNoise, // TODO: Créer NetherNoise
    en: EndNoise, // TODO: Créer EndNoise
}

pub fn setupGenerator(mut g: Generator, mc: i32, flags: u32) {
    g.mc = mc;
    g.dim = DIM_UNDEF; // TODO: Créer DIM_UNDEF 
    g.flags = flags;
    g.seed = 0;
    g.sha = 0;

    initBiomeNoise(g.bn, mc);

}


