struct Range {
    scale: i32,
    x: i32,
    z: i32,
    sx: i32,
    sz: i32,
    y: i32,
    sy: i32,
}

struct Spline {
    len: i32,
    typ: i32,
    loc: [f32; 12],
    der: [f32; 12],
    val: [*mut Spline; 12],
}

struct FixSpline {
    len: i32,
    val: f32,
}

struct SplineStack {
    stack: [*mut Spline; 42],
    fstack: [*mut FixSpline; 151],
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
    NP_Max,
}

impl NoiseParameter {
    fn value(&self) -> i32 { 
        match *self {
            NoiseParameter::NP_Temperature     => 0,
            NoiseParameter::NP_Humidity        => 1,
            NoiseParameter::NP_Continentalness => 2,
            NoiseParameter::NP_Erosion         => 3,
            NoiseParameter::NP_Shift           => 4,
            NoiseParameter::NP_Depth           => 4,
            NoiseParameter::NP_Weirdness       => 5,
            NoiseParameter::NP_Max             => 0, // FIXME: THAT NOT a 0 normally :(

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

struct BiomeNoise {
    climate: [DoublePerlinNoise; NP_MAX],
    oct: [PerlinNoise; 2*23],
    sp: Spline,
    ss: SplineStack,
    nptype: i32,
    mc: i32,
}



pub fn initBiomeNoise(bn: BiomeNoise, mc: i32) {
    let ss: SplineStack = bn.ss;
    ss.stack.clear();
    let sp: Spline = ss.stack[ss.len];
    sp.typ = SP_CONTINETALNESS; // TODO: Créer SP_CONTINETALNESS

    let sp1: Spline = createLandSpline(ss, -0.15, 0.00, 0.0, 0.1, 0.00, -0.03, 0); // TODO: Créer createLandSpline
    let sp2: Spline = createLandSpline(ss, -0.10, 0.03, 0.1, 0.1, 0.01, -0.03, 0);
    let sp3: Spline = createLandSpline(ss, -0.10, 0.03, 0.1, 0.7, 0.01, -0.03, 1);
    let sp4: Spline = createLandSpline(ss, -0.05, 0.03, 0.1, 1.0, 0.01, 0.01, 1);

    addSplineVal(sp, -1.10, createFixSpline(ss, 0.044), 0.0);
    addSplineVal(sp, -1.02, createFixSpline(ss, -0.2222), 0.0);
    addSplineVal(sp, -0.51, createFixSpline(ss, -0.2222), 0.0);
    addSplineVal(sp, -0.44, createFixSpline(ss, -0.12), 0.0);
    addSplineVal(sp, -0.18, createFixSpline(ss, -0.12), 0.0);
    addSplineVal(sp, -0.16, sp1, 0.0);
    addSplineVal(sp, -0.15, sp1, 0.0);
    addSplineVal(sp, -0.10, sp2, 0.0);
    addSplineVal(sp,  0.25, sp3, 0.0);
    addSplineVal(sp,  1.00, sp4, 0.0);

    bn.sp = sp;
    bn.mc = mc;
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