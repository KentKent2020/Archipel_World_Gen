use crate::noise::{DoublePerlinNoise, PerlinNoise, xDoublePerlinInit};
use crate::rng::{lerp, Xoroshiro, xSetSeed, xNextLong};
pub struct Range {
    scale: i32,
    x: i32,
    z: i32,
    sx: i32,
    sz: i32,
    y: i32,
    sy: i32,
}

pub struct NetherNoise {
    temperature: DoublePerlinNoise,
    humidity: DoublePerlinNoise,
    oct: [PerlinNoise; 8],
}

pub struct EndNoise {
    perlin: PerlinNoise,
    mc: i32,
}

pub struct Spline {
    len: i32,
    typ: i32,
    loc: [f32; 12],
    der: [f32; 12],
    val: [*mut Spline; 12],
}

pub struct FixSpline {
    len: i32,
    val: f32,
}

impl FixSpline {
    pub fn ToSpline(&mut self) -> Spline {
        Spline {
            len: self.len,
            typ: 0, 
            loc: [0.0; 12], 
            der: [0.0; 12],
            val: [std::ptr::null_mut(); 12],
        }
    }
}

pub struct SplineStack {
    stack: [Spline; 42],
    fstack: [FixSpline; 151],
    len: i32,
    flen: i32,
}

enum NoiseParameter {
    NP_Temperature,
    NP_Humidity,
    NP_Continentalness,
    NP_Erosion,
    NP_Shift,
    NP_Depth, // NP_DEPTH, not a real climate
    NP_Weirdness,
    // NP_Max, // Mis en const
}

const NP_MAX: usize = 6;

impl NoiseParameter {
    fn value(&self) -> usize { 
        match *self {
            NoiseParameter::NP_Temperature     => 0,
            NoiseParameter::NP_Humidity        => 1,
            NoiseParameter::NP_Continentalness => 2,
            NoiseParameter::NP_Erosion         => 3,
            NoiseParameter::NP_Shift           => 4,
            NoiseParameter::NP_Depth           => 4,
            NoiseParameter::NP_Weirdness       => 5,
            // NoiseParameter::NP_Max             => 6, // Mis en const

        }
    }
}

enum SplineParameter {
    SP_CONTINENTALNESS,
    SP_EROSION, 
    SP_RIDGES,
    SP_WEIRDNESS
}

impl SplineParameter {
    fn value(&self) -> i32 {
        match *self {
            SplineParameter::SP_CONTINENTALNESS => 0, // FIXME:
            SplineParameter::SP_EROSION => 0,         // FIXME: 
            SplineParameter::SP_RIDGES => 0,          // FIXME:
            SplineParameter::SP_WEIRDNESS => 0,       // FIXME:

        }
    }
}

// enum
// {
//     NP_TEMPERATURE      = 0,
//     NP_HUMIDITY         = 1,
//     NP_CONTINENTALNESS  = 2,
//     NP_EROSION          = 3,
//     NP_SHIFT            = 4, NP_DEPTH = NP_SHIFT, // not a real climate
//     NP_WEIRDNESS        = 5,
//     NP_MAX
// };

pub struct BiomeNoise {
    climate: [DoublePerlinNoise; NP_MAX],
    oct: [PerlinNoise; 2*23],
    sp: *mut Spline,
    ss: SplineStack,
    nptype: i32,
    mc: i32,
}



pub fn initBiomeNoise(bn: &mut BiomeNoise, mc: i32) {
    let mut ss: SplineStack = bn.ss;
    let mut sp: &mut Spline = &mut ss.stack[ss.len as usize];
    ss.len += 1;
    sp.typ = SplineParameter::SP_CONTINENTALNESS.value(); 

    let sp1: Spline = createLandSpline(&mut ss, -0.15, 0.00, 0.0, 0.1, 0.00, -0.03, 0); 
    let sp2: Spline = createLandSpline(&mut ss, -0.10, 0.03, 0.1, 0.1, 0.01, -0.03, 0);
    let sp3: Spline = createLandSpline(&mut ss, -0.10, 0.03, 0.1, 0.7, 0.01, -0.03, 1);
    let sp4: Spline = createLandSpline(&mut ss, -0.05, 0.03, 0.1, 1.0, 0.01,  0.01,  1);

    addSplineVal(&mut sp, -1.10, &createFixSpline(&mut ss, 0.044), 0.0);
    addSplineVal(&mut sp, -1.02, &createFixSpline(&mut ss, -0.2222), 0.0);
    addSplineVal(&mut sp, -0.51, &createFixSpline(&mut ss, -0.2222), 0.0);
    addSplineVal(&mut sp, -0.44, &createFixSpline(&mut ss, -0.12), 0.0);
    addSplineVal(&mut sp, -0.18, &createFixSpline(&mut ss, -0.12), 0.0);
    addSplineVal(&mut sp, -0.16, &sp1, 0.0);
    addSplineVal(&mut sp, -0.15, &sp1, 0.0);
    addSplineVal(&mut sp, -0.10, &sp2, 0.0);
    addSplineVal(&mut sp,  0.25, &sp3, 0.0);
    addSplineVal(&mut sp,  1.00, &sp4, 0.0);

    bn.sp = sp as *mut Spline;
    bn.mc = mc;
}

pub fn addSplineVal(rsp: &mut Spline, loc: f32, val: &Spline, der: f32) {
    rsp.loc[rsp.len as usize] = loc;
    rsp.val[rsp.len as usize] = val as *const Spline as *mut Spline;
    rsp.der[rsp.len as usize] = der;
    rsp.len += 1;

}

pub fn createFixSpline(ss: &mut SplineStack, val: f32) -> &mut Spline {
    let sp: &mut FixSpline = &mut ss.fstack[ss.flen as usize]; 
    ss.flen += 1;
    sp.len = 1;
    sp.val = val;
    sp as &mut Spline //FIXME: HOW CAN A TRANSFORM THIS FIXSPLINE IN SPLINE!!!!!!!
}





pub fn init_climate_seed(dpn: &mut DoublePerlinNoise, oct: PerlinNoise, xlo: u64, xhi: u64, large: bool, nptype: i32, nmax: i32) -> i32 { // Thx ChatGpt
    let mut pxr = Xoroshiro { lo: 0, hi: 0 };
    let mut n = 0;

    match nptype {
        NP_SHIFT => {
            static AMP: [f64; 4] = [1.0, 1.0, 1.0, 0.0];
            // md5 "minecraft:offset"
            pxr.lo = xlo ^ 0x080518cf6af25384;
            pxr.hi = xhi ^ 0x3f3dfb40a54febd5;
            n += xDoublePerlinInit(dpn, &mut pxr, &mut [oct], &AMP, -3, 4, nmax);
        }
        NP_TEMPERATURE => {
            static AMP: [f64; 6] = [1.5, 0.0, 1.0, 0.0, 0.0, 0.0];
            // md5 "minecraft:temperature" or "minecraft:temperature_large"
            pxr.lo = xlo ^ if large { 0x944b0073edf549db } else { 0x5c7e6b29735f0d7f };
            pxr.hi = xhi ^ if large { 0x4ff44347e9d22b96 } else { 0xf7d86f1bbc734988 };
            n += xDoublePerlinInit(dpn, &mut pxr, &mut [oct], &AMP, if large { -12 } else { -10 }, 6, nmax);
        }
        NP_HUMIDITY => {
            static AMP: [f64; 6] = [1.0, 1.0, 0.0, 0.0, 0.0, 0.0];
            // md5 "minecraft:vegetation" or "minecraft:vegetation_large"
            pxr.lo = xlo ^ if large { 0x71b8ab943dbd5301 } else { 0x81bb4d22e8dc168e };
            pxr.hi = xhi ^ if large { 0xbb63ddcf39ff7a2b } else { 0xf1c8b4bea16303cd };
            n += xDoublePerlinInit(dpn, &mut pxr, &mut [oct], &AMP, if large { -10 } else { -8 }, 6, nmax);
        }
        NP_CONTINENTALNESS => {
            static AMP: [f64; 9] = [1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0];
            // md5 "minecraft:continentalness" or "minecraft:continentalness_large"
            pxr.lo = xlo ^ if large { 0x9a3f51a113fce8dc } else { 0x83886c9d0ae3a662 };
            pxr.hi = xhi ^ if large { 0xee2dbd157e5dcdad } else { 0xafa638a61b42e8ad };
            n += xDoublePerlinInit(dpn, &mut pxr, &mut [oct], &AMP, if large { -11 } else { -9 }, 9, nmax);
        }
        NP_EROSION => {
            static AMP: [f64; 5] = [1.0, 1.0, 0.0, 1.0, 1.0];
            // md5 "minecraft:erosion" or "minecraft:erosion_large"
            pxr.lo = xlo ^ if large { 0x8c984b1f8702a951 } else { 0xd02491e6058f6fd8 };
            pxr.hi = xhi ^ if large { 0xead7b1f92bae535f } else { 0x4792512c94c17a80 };
            n += xDoublePerlinInit(dpn, &mut pxr, &mut [oct], &AMP, if large { -11 } else { -9 }, 5, nmax);
        }
        NP_WEIRDNESS => {
            static AMP: [f64; 6] = [1.0, 2.0, 1.0, 0.0, 0.0, 0.0];
            // md5 "minecraft:ridge"
            pxr.lo = xlo ^ 0xefc8ef4d36102b34;
            pxr.hi = xhi ^ 0x1beeeb324a0f24ea;
            n += xDoublePerlinInit(dpn, &mut pxr, &mut [oct], &AMP, -7, 6, nmax);
        }
        _ => {
            println!("unsupported climate parameter {}", nptype);
            std::process::exit(1);
        }
    }
    n
}



pub fn setBiomeSeed(bn: &BiomeNoise, seed: u64, large: i32) {
    let mut pxr: Xoroshiro;
    xSetSeed(&mut pxr, seed);
    let xlo: u64 = xNextLong(&mut pxr);
    let xhi: u64 = xNextLong(&mut pxr);

    let mut n: i32 = 0;
    let mut i: i32 = 0;

    loop {
        n += init_climate_seed(&mut bn.climate[i as usize], bn.oct[n as usize], xlo, xhi, if large != 0 {false} else {true}, i, -1);
        if i < NP_MAX as i32 {
            break;
        }
        i += 1;
    }

    if n > bn.oct.len() as i32 {
        println!("setBiomeSeed(): BiomeNoise is malformed, buffer too small");
        std::process::exit(1);
    }

    bn.nptype = -1;
}


pub fn getOffsetValue(weirdness: f32, continentalness: f32) -> f32 {
    let f0:  f32 = 1.0 - (1.0 - continentalness) * 0.5;
    let f1:  f32 = 0.5 * (1.0 - continentalness);
    let f2:  f32 = (weirdness + 1.17) * 0.46082947;
    let off: f32 = f2 * f0 - f1;
    if (weirdness < -0.7) {
        if off > -0.2222 {
            off
        } else {
            -0.2222
        }
    } else {
        if off > 0.0 {
            off
        } else {
            0.0
        }
    }
}

pub fn createSpline_38219(ss: &mut SplineStack, f: f32, bl: i32) -> Spline {
    let mut sp: Spline = ss.stack[ss.len as usize];
    sp.typ = SplineParameter::SP_RIDGES.value();

    let i: f32 = getOffsetValue(-1.0, f);
    let k: f32 = getOffsetValue( 1.0, f);
    let mut l: f32 = 1.0 - (1.0 - f) * 0.5;
    let u: f32 = 0.5 * (1.0 - f);
    l = u / (0.46082947 * l) - 1.17;
    if -0.65 < l && l < 1.0 {
        let p = getOffsetValue(-0.75, f);
        let q = (p - i) * 4.0;
        let r = getOffsetValue(l, f);
        let s = (k - r) / (1.0 - l);
    
        let u = getOffsetValue(-0.65, f);
        
        addSplineVal(&mut sp, -1.0, createFixSpline(ss, i), q);
        addSplineVal(&mut sp, -0.75, createFixSpline(ss, p), 0.0);
        addSplineVal(&mut sp, -0.65, createFixSpline(ss, u), 0.0);
        addSplineVal(&mut sp, l - 0.01, createFixSpline(ss, r), 0.0);
        addSplineVal(&mut sp, l, createFixSpline(ss, r), s);
        addSplineVal(&mut sp, 1.0, createFixSpline(ss, k), s);
    } else {
        let u = (k - i) * 0.5;
        if bl != 0 {
            addSplineVal(&mut sp, -1.0, createFixSpline(ss, if i > 0.2 { i } else { 0.2 }), 0.0);
            addSplineVal(&mut sp, 0.0, createFixSpline(ss, lerp(0.5, i as f64, k as f64) as f32), u); // TODO: Check if the f32 do not cause error
        } else {
            addSplineVal(&mut sp, -1.0, createFixSpline(ss, i), u);
        }
        addSplineVal(&mut sp, 1.0, createFixSpline(ss, k), u);
    }
    return sp;
}

pub fn createFlatOffsetSpline(ss: &mut SplineStack, f: f32, g: f32, h: f32, i: f32, j: f32, k: f32) -> Spline{
    let mut sp: Spline = ss.stack[ss.len as usize];

    sp.typ = SplineParameter::SP_RIDGES.value();

    let mut l: f32 = 0.5 * (g - f); 
    if l < k {l=k};
    let m = 5.0 * (h-g);

    addSplineVal(&mut sp, -1.0, createFixSpline(ss, f), l);
    addSplineVal(&mut sp, -0.4, createFixSpline(ss, g), if l < m { l } else { m });
    addSplineVal(&mut sp, 0.0,  createFixSpline(ss, h), m);
    addSplineVal(&mut sp, 0.4,  createFixSpline(ss, i), 2.0 * (i - h));
    addSplineVal(&mut sp, 1.0,  createFixSpline(ss, j), 0.7 * (j - i));

    return sp;

}

pub fn createLandSpline(ss: &mut SplineStack, f: f32, g: f32, h: f32, i: f32, j: f32, k: f32, bl: i32) -> Spline{
    let sp1: Spline = createSpline_38219(ss, lerp(i as f64, 0.6, 1.5) as f32, bl); // TODO: Check if the f32 do not cause error
    let sp2: Spline = createSpline_38219(ss, lerp(i as f64, 0.6, 1.0) as f32, bl); // TODO: Check if the f32 do not cause error
    let sp3: Spline = createSpline_38219(ss, i, bl);
    let ih: f32 = 0.5 * i;
    let sp4: Spline = createFlatOffsetSpline(ss, f-0.15, ih, ih, ih, i*0.6, 0.5);
    let sp5: Spline = createFlatOffsetSpline(ss, f, j*i, g*i, ih, i*0.6, 0.5);
    let sp6: Spline = createFlatOffsetSpline(ss, f, j, j, g, h, 0.5);
    let sp7: Spline= createFlatOffsetSpline(ss, f, j, j, g, h, 0.5);

    let mut sp8: Spline = ss.stack[ss.len as usize];


    sp8.typ = SplineParameter::SP_RIDGES.value(); 
    addSplineVal(&mut sp8, -1.0, createFixSpline(ss, f), 0.0);
    addSplineVal(&mut sp8, -0.4, &sp6, 0.0);
    addSplineVal(&mut sp8, 0.0, createFixSpline(ss, h + 0.07), 0.0);

    let sp9: Spline = createFlatOffsetSpline(ss, -0.02, k, k, g, h, 0.0);
    let mut sp: Spline = ss.stack[ss.len as usize];

    sp.typ = SplineParameter::SP_EROSION.value();
    addSplineVal(&mut sp, -0.85, &sp1, 0.0);
    addSplineVal(&mut sp, -0.7,  &sp2, 0.0);
    addSplineVal(&mut sp, -0.4,  &sp3, 0.0);
    addSplineVal(&mut sp, -0.35, &sp4, 0.0);
    addSplineVal(&mut sp, -0.1,  &sp5, 0.0);
    addSplineVal(&mut sp,  0.2,  &sp6, 0.0);

    if bl != 0 {
        addSplineVal(&mut sp, 0.4,  &sp7, 0.0);
        addSplineVal(&mut sp, 0.45, &sp8, 0.0);
        addSplineVal(&mut sp, 0.55, &sp8, 0.0);
        addSplineVal(&mut sp, 0.58, &sp7, 0.0);
    }
    addSplineVal(&mut sp, 0.7, &sp9, 0.0);
    return sp;
    
}   