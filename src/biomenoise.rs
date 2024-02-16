use crate::noise::{DoublePerlinNoise, PerlinNoise};
use crate::rng::{lerp};
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

    let sp1: Spline = createLandSpline(ss, -0.15, 0.00, 0.0, 0.1, 0.00, -0.03, 0); 
    let sp2: Spline = createLandSpline(ss, -0.10, 0.03, 0.1, 0.1, 0.01, -0.03, 0);
    let sp3: Spline = createLandSpline(ss, -0.10, 0.03, 0.1, 0.7, 0.01, -0.03, 1);
    let sp4: Spline = createLandSpline(ss, -0.05, 0.03, 0.1, 1.0, 0.01, 0.01, 1);

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
    sp as &mut Spline // Convertir la référence de FixSpline en Spline et la retourner
}


pub fn createSpline_38219(ss: SplineStack, f: f32, bl: i32) -> Spline {
    let sp:Spline = ss.stack[ss.len];
    // ss.len += 1; // TODO: ???
    sp.typ = SplineParameter::SP_RIDGES;

    let i: f32 = getOffsetValue(-1.0, f);
    let k: f32 = getOffsetValue( 1.0, f);
    let l: f32 = 1.0 - (1.0 - f) * 0.5;
    let u: f32 = 0.5 * (1.0 - f);
    l = u / (0.46082947 * l) - 1.17;
    if -0.65 < l && l < 1.0 {
        let p = getOffsetValue(-0.75, f);
        let q = (p - i) * 4.0;
        let r = getOffsetValue(l, f);
        let s = (k - r) / (1.0 - l);
    
        let u = getOffsetValue(-0.65, f);
        
        addSplineVal(sp, -1.0, createFixSpline(ss, i), q);
        addSplineVal(sp, -0.75, createFixSpline(ss, p), 0.0);
        addSplineVal(sp, -0.65, createFixSpline(ss, u), 0.0);
        addSplineVal(sp, l - 0.01, createFixSpline(ss, r), 0.0);
        addSplineVal(sp, l, createFixSpline(ss, r), s);
        addSplineVal(sp, 1.0, createFixSpline(ss, k), s);
    } else {
        let u = (k - i) * 0.5;
        if bl {
            addSplineVal(sp, -1.0, createFixSpline(ss, if i > 0.2 { i } else { 0.2 }), 0.0);
            addSplineVal(sp, 0.0, createFixSpline(ss, lerp(0.5, i, k)), u);
        } else {
            addSplineVal(sp, -1.0, createFixSpline(ss, i), u);
        }
        addSplineVal(sp, 1.0, createFixSpline(ss, k), u);
    }
    return sp;
}

pub fn createFlatOffsetSpline(ss: SplineStack, f: f32, g: f32, h: f32, i: f32, j: f32, k: f32) -> Spline{
    let sp: Spline = ss.stack[ss.len];
    // ss.len += 1; // TODO: ???
    sp.typ = SplineParameter::SP_RIDGES;

    let l: f32 = 0.5 * (g - f); 
    if l < k {l=k};
    let m = 5.0 * (h-g);

    add_spline_val(sp, -1.0, create_fix_spline(ss, f), l);
    addSplineVal(sp, -0.4, createFixSpline(ss, g), if l < m { l } else { m });
    addSplineVal(sp, 0.0,  createFixSpline(ss, h), m);
    addSplineVal(sp, 0.4,  createFixSpline(ss, i), 2.0 * (i - h));
    addSplineVal(sp, 1.0,  createFixSpline(ss, j), 0.7 * (j - i));

    return sp;

}

pub fn createLandSpline(ss: SplineStack, f: f32, g: f32, h: f32, i: f32, j: f32, k: f32, bl: i32) -> Spline{
    let sp1: Spline = createSpline_38219(ss, lerp(i, 0.6, 1.5), bl);
    let sp2: Spline = createSpline_38219(ss, lerp(i, 0.6, 1.0), bl);
    let sp3: Spline = createSpline_38219(ss, i, bl);
    const ih:f32 = 0.5 * i;
    let sp4: Spline = createFlatOffsetSpline(ss, f-0.15, ih, ih, ih, i*0.6, 0.5);
    let sp5: Spline = createFlatOffsetSpline(ss, f, j*i, g*i, ih, i*0.6, 0.5);
    let sp6: Spline = createFlatOffsetSpline(ss, f, j, j, g, h, 0.5);
    let sp7: Spline= createFlatOffsetSpline(ss, f, j, j, g, h, 0.5);

    let sp8: Spline = ss.stack[ss.len];
    // ss.len += 1; //TODO: ???

    sp8.typ = SplineParameter::SP_RIDGES; 
    addSplineVal(sp8, -1.0, createFixSpline(ss, f), 0.0);
    addSplineVal(sp8, -0.4, sp6, 0.0);
    addSplineVal(sp8, 0.0, createFixSpline(ss, h + 0.07), 0.0);

    let sp9: Spline = createFlatOffsetSpline(ss, -0.02, k, k, g, h, 0.0);
    let sp: Spline = ss.stack[ss.len];
    // ss.len += 1; // TODO: ???
    sp.typ = SplineParameter::SP_EROSION;
    addSplineVal(sp, -0.85, sp1, 0.0);
    addSplineVal(sp, -0.7,  sp2, 0.0);
    addSplineVal(sp, -0.4,  sp3, 0.0);
    addSplineVal(sp, -0.35, sp4, 0.0);
    addSplineVal(sp, -0.1,  sp5, 0.0);
    addSplineVal(sp,  0.2,  sp6, 0.0);

    if bl {
        addSplineVal(sp, 0.4,  sp7, 0.0);
        addSplineVal(sp, 0.45, sp8, 0.0);
        addSplineVal(sp, 0.55, sp8, 0.0);
        addSplineVal(sp, 0.58, sp7, 0.0);
    }
    addSplineVal(sp, 0.7, sp9, 0.0);
    return sp;
    
}   

// static Spline *createLandSpline(
//     SplineStack *ss, float f, float g, float h, float i, float j, float k, int bl)
// {
//     Spline *sp1 = createSpline_38219(ss, lerp(i, 0.6F, 1.5F), bl);
//     Spline *sp2 = createSpline_38219(ss, lerp(i, 0.6F, 1.0F), bl);
//     Spline *sp3 = createSpline_38219(ss, i, bl);
//     const float ih = 0.5F * i;
//     Spline *sp4 = createFlatOffsetSpline(ss, f-0.15F, ih, ih, ih, i*0.6F, 0.5F);
//     Spline *sp5 = createFlatOffsetSpline(ss, f, j*i, g*i, ih, i*0.6F, 0.5F);
//     Spline *sp6 = createFlatOffsetSpline(ss, f, j, j, g, h, 0.5F);
//     Spline *sp7 = createFlatOffsetSpline(ss, f, j, j, g, h, 0.5F);

//     Spline *sp8 = &ss->stack[ss->len++];
//     sp8->typ = SP_RIDGES;
//     addSplineVal(sp8, -1.0F, createFixSpline(ss, f), 0.0F);
//     addSplineVal(sp8, -0.4F, sp6, 0.0F);
//     addSplineVal(sp8,  0.0F, createFixSpline(ss, h + 0.07F), 0.0F);

//     Spline *sp9 = createFlatOffsetSpline(ss, -0.02F, k, k, g, h, 0.0F);
//     Spline *sp = &ss->stack[ss->len++];
//     sp->typ = SP_EROSION;
//     addSplineVal(sp, -0.85F, sp1, 0.0F);
//     addSplineVal(sp, -0.7F,  sp2, 0.0F);
//     addSplineVal(sp, -0.4F,  sp3, 0.0F);
//     addSplineVal(sp, -0.35F, sp4, 0.0F);
//     addSplineVal(sp, -0.1F,  sp5, 0.0F);
//     addSplineVal(sp,  0.2F,  sp6, 0.0F);
//     if (bl) {
//         addSplineVal(sp, 0.4F,  sp7, 0.0F);
//         addSplineVal(sp, 0.45F, sp8, 0.0F);
//         addSplineVal(sp, 0.55F, sp8, 0.0F);
//         addSplineVal(sp, 0.58F, sp7, 0.0F);
//     }
//     addSplineVal(sp, 0.7F, sp9, 0.0F);
//     return sp;
// }