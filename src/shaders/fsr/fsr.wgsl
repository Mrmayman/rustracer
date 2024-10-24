struct const_buffer {
    Const0_: vec4<u32>,
    Const1_: vec4<u32>,
    Const2_: vec4<u32>,
    Const3_: vec4<u32>,
    Sample: vec4<u32>,
}

@group(0) @binding(0) 
var<uniform> global: const_buffer;
// @group(0) @binding(1) 
// var InputTexture: texture_2d<f32>;
// @group(0) @binding(2) 
// var OutputTexture: texture_storage_2d<rgba32float,read_write>;
// @group(0) @binding(3) 
// var InputSampler: sampler;
var<private> gl_LocalInvocationID_1: vec3<u32>;
var<private> gl_WorkGroupID_1: vec3<u32>;

fn AU1_AH1_AF1_x(a: f32) -> u32 {
    var a_1: f32;

    a_1 = a;
    let _e12 = a_1;
    let _e15 = a_1;
    return pack2x16float(vec2<f32>(_e15, 0f));
}

fn AF1_x(a_2: f32) -> f32 {
    var a_3: f32;

    a_3 = a_2;
    let _e12 = a_3;
    return f32(_e12);
}

fn AF2_x(a_4: f32) -> vec2<f32> {
    var a_5: f32;

    a_5 = a_4;
    let _e12 = a_5;
    let _e13 = a_5;
    return vec2<f32>(_e12, _e13);
}

fn AF3_x(a_6: f32) -> vec3<f32> {
    var a_7: f32;

    a_7 = a_6;
    let _e12 = a_7;
    let _e13 = a_7;
    let _e14 = a_7;
    return vec3<f32>(_e12, _e13, _e14);
}

fn AF4_x(a_8: f32) -> vec4<f32> {
    var a_9: f32;

    a_9 = a_8;
    let _e12 = a_9;
    let _e13 = a_9;
    let _e14 = a_9;
    let _e15 = a_9;
    return vec4<f32>(_e12, _e13, _e14, _e15);
}

fn AU1_x(a_10: u32) -> u32 {
    var a_11: u32;

    a_11 = a_10;
    let _e12 = a_11;
    return u32(_e12);
}

fn AU2_x(a_12: u32) -> vec2<u32> {
    var a_13: u32;

    a_13 = a_12;
    let _e12 = a_13;
    let _e13 = a_13;
    return vec2<u32>(_e12, _e13);
}

fn AU3_x(a_14: u32) -> vec3<u32> {
    var a_15: u32;

    a_15 = a_14;
    let _e12 = a_15;
    let _e13 = a_15;
    let _e14 = a_15;
    return vec3<u32>(_e12, _e13, _e14);
}

fn AU4_x(a_16: u32) -> vec4<u32> {
    var a_17: u32;

    a_17 = a_16;
    let _e12 = a_17;
    let _e13 = a_17;
    let _e14 = a_17;
    let _e15 = a_17;
    return vec4<u32>(_e12, _e13, _e14, _e15);
}

fn AAbsSU1_(a_18: u32) -> u32 {
    var a_19: u32;

    a_19 = a_18;
    let _e12 = a_19;
    let _e14 = a_19;
    return u32(abs(i32(_e14)));
}

fn AAbsSU2_(a_20: vec2<u32>) -> vec2<u32> {
    var a_21: vec2<u32>;

    a_21 = a_20;
    let _e12 = a_21;
    let _e14 = a_21;
    return vec2<u32>(abs(vec2<i32>(_e14)));
}

fn AAbsSU3_(a_22: vec3<u32>) -> vec3<u32> {
    var a_23: vec3<u32>;

    a_23 = a_22;
    let _e12 = a_23;
    let _e14 = a_23;
    return vec3<u32>(abs(vec3<i32>(_e14)));
}

fn AAbsSU4_(a_24: vec4<u32>) -> vec4<u32> {
    var a_25: vec4<u32>;

    a_25 = a_24;
    let _e12 = a_25;
    let _e14 = a_25;
    return vec4<u32>(abs(vec4<i32>(_e14)));
}

fn ABfe(src: u32, off: u32, bits: u32) -> u32 {
    var src_1: u32;
    var off_1: u32;
    var bits_1: u32;

    src_1 = src;
    off_1 = off;
    bits_1 = bits;
    let _e17 = off_1;
    let _e19 = bits_1;
    let _e21 = src_1;
    let _e22 = off_1;
    let _e24 = bits_1;
    return extractBits(_e21, u32(i32(_e22)), u32(i32(_e24)));
}

fn ABfi(src_2: u32, ins: u32, mask: u32) -> u32 {
    var src_3: u32;
    var ins_1: u32;
    var mask_1: u32;

    src_3 = src_2;
    ins_1 = ins;
    mask_1 = mask;
    let _e16 = ins_1;
    let _e17 = mask_1;
    let _e19 = src_3;
    let _e20 = mask_1;
    return ((_e16 & _e17) | (_e19 & ~(_e20)));
}

fn ABfiM(src_4: u32, ins_2: u32, bits_2: u32) -> u32 {
    var src_5: u32;
    var ins_3: u32;
    var bits_3: u32;

    src_5 = src_4;
    ins_3 = ins_2;
    bits_3 = bits_2;
    let _e19 = bits_3;
    let _e21 = src_5;
    let _e22 = ins_3;
    let _e24 = bits_3;
    return insertBits(_e21, _e22, 0u, u32(i32(_e24)));
}

fn AClampF1_(x: f32, n: f32, m: f32) -> f32 {
    var x_1: f32;
    var n_1: f32;
    var m_1: f32;

    x_1 = x;
    n_1 = n;
    m_1 = m;
    let _e19 = x_1;
    let _e20 = n_1;
    let _e21 = m_1;
    return clamp(_e19, _e20, _e21);
}

fn AClampF2_(x_2: vec2<f32>, n_2: vec2<f32>, m_2: vec2<f32>) -> vec2<f32> {
    var x_3: vec2<f32>;
    var n_3: vec2<f32>;
    var m_3: vec2<f32>;

    x_3 = x_2;
    n_3 = n_2;
    m_3 = m_2;
    let _e19 = x_3;
    let _e20 = n_3;
    let _e21 = m_3;
    return clamp(_e19, _e20, _e21);
}

fn AClampF3_(x_4: vec3<f32>, n_4: vec3<f32>, m_4: vec3<f32>) -> vec3<f32> {
    var x_5: vec3<f32>;
    var n_5: vec3<f32>;
    var m_5: vec3<f32>;

    x_5 = x_4;
    n_5 = n_4;
    m_5 = m_4;
    let _e19 = x_5;
    let _e20 = n_5;
    let _e21 = m_5;
    return clamp(_e19, _e20, _e21);
}

fn AClampF4_(x_6: vec4<f32>, n_6: vec4<f32>, m_6: vec4<f32>) -> vec4<f32> {
    var x_7: vec4<f32>;
    var n_7: vec4<f32>;
    var m_7: vec4<f32>;

    x_7 = x_6;
    n_7 = n_6;
    m_7 = m_6;
    let _e19 = x_7;
    let _e20 = n_7;
    let _e21 = m_7;
    return clamp(_e19, _e20, _e21);
}

fn AFractF1_(x_8: f32) -> f32 {
    var x_9: f32;

    x_9 = x_8;
    let _e13 = x_9;
    return fract(_e13);
}

fn AFractF2_(x_10: vec2<f32>) -> vec2<f32> {
    var x_11: vec2<f32>;

    x_11 = x_10;
    let _e13 = x_11;
    return fract(_e13);
}

fn AFractF3_(x_12: vec3<f32>) -> vec3<f32> {
    var x_13: vec3<f32>;

    x_13 = x_12;
    let _e13 = x_13;
    return fract(_e13);
}

fn AFractF4_(x_14: vec4<f32>) -> vec4<f32> {
    var x_15: vec4<f32>;

    x_15 = x_14;
    let _e13 = x_15;
    return fract(_e13);
}

fn ALerpF1_(x_16: f32, y: f32, a_26: f32) -> f32 {
    var x_17: f32;
    var y_1: f32;
    var a_27: f32;

    x_17 = x_16;
    y_1 = y;
    a_27 = a_26;
    let _e19 = x_17;
    let _e20 = y_1;
    let _e21 = a_27;
    return mix(_e19, _e20, _e21);
}

fn ALerpF2_(x_18: vec2<f32>, y_2: vec2<f32>, a_28: vec2<f32>) -> vec2<f32> {
    var x_19: vec2<f32>;
    var y_3: vec2<f32>;
    var a_29: vec2<f32>;

    x_19 = x_18;
    y_3 = y_2;
    a_29 = a_28;
    let _e19 = x_19;
    let _e20 = y_3;
    let _e21 = a_29;
    return mix(_e19, _e20, _e21);
}

fn ALerpF3_(x_20: vec3<f32>, y_4: vec3<f32>, a_30: vec3<f32>) -> vec3<f32> {
    var x_21: vec3<f32>;
    var y_5: vec3<f32>;
    var a_31: vec3<f32>;

    x_21 = x_20;
    y_5 = y_4;
    a_31 = a_30;
    let _e19 = x_21;
    let _e20 = y_5;
    let _e21 = a_31;
    return mix(_e19, _e20, _e21);
}

fn ALerpF4_(x_22: vec4<f32>, y_6: vec4<f32>, a_32: vec4<f32>) -> vec4<f32> {
    var x_23: vec4<f32>;
    var y_7: vec4<f32>;
    var a_33: vec4<f32>;

    x_23 = x_22;
    y_7 = y_6;
    a_33 = a_32;
    let _e19 = x_23;
    let _e20 = y_7;
    let _e21 = a_33;
    return mix(_e19, _e20, _e21);
}

fn AMax3F1_(x_24: f32, y_8: f32, z: f32) -> f32 {
    var x_25: f32;
    var y_9: f32;
    var z_1: f32;

    x_25 = x_24;
    y_9 = y_8;
    z_1 = z;
    let _e19 = y_9;
    let _e20 = z_1;
    let _e22 = x_25;
    let _e25 = y_9;
    let _e26 = z_1;
    return max(_e22, max(_e25, _e26));
}

fn AMax3F2_(x_26: vec2<f32>, y_10: vec2<f32>, z_2: vec2<f32>) -> vec2<f32> {
    var x_27: vec2<f32>;
    var y_11: vec2<f32>;
    var z_3: vec2<f32>;

    x_27 = x_26;
    y_11 = y_10;
    z_3 = z_2;
    let _e19 = y_11;
    let _e20 = z_3;
    let _e22 = x_27;
    let _e25 = y_11;
    let _e26 = z_3;
    return max(_e22, max(_e25, _e26));
}

fn AMax3F3_(x_28: vec3<f32>, y_12: vec3<f32>, z_4: vec3<f32>) -> vec3<f32> {
    var x_29: vec3<f32>;
    var y_13: vec3<f32>;
    var z_5: vec3<f32>;

    x_29 = x_28;
    y_13 = y_12;
    z_5 = z_4;
    let _e19 = y_13;
    let _e20 = z_5;
    let _e22 = x_29;
    let _e25 = y_13;
    let _e26 = z_5;
    return max(_e22, max(_e25, _e26));
}

fn AMax3F4_(x_30: vec4<f32>, y_14: vec4<f32>, z_6: vec4<f32>) -> vec4<f32> {
    var x_31: vec4<f32>;
    var y_15: vec4<f32>;
    var z_7: vec4<f32>;

    x_31 = x_30;
    y_15 = y_14;
    z_7 = z_6;
    let _e19 = y_15;
    let _e20 = z_7;
    let _e22 = x_31;
    let _e25 = y_15;
    let _e26 = z_7;
    return max(_e22, max(_e25, _e26));
}

fn AMax3SU1_(x_32: u32, y_16: u32, z_8: u32) -> u32 {
    var x_33: u32;
    var y_17: u32;
    var z_9: u32;

    x_33 = x_32;
    y_17 = y_16;
    z_9 = z_8;
    let _e16 = x_33;
    let _e18 = y_17;
    let _e20 = z_9;
    let _e22 = y_17;
    let _e24 = z_9;
    let _e27 = x_33;
    let _e29 = y_17;
    let _e31 = z_9;
    let _e33 = y_17;
    let _e35 = z_9;
    return u32(max(i32(_e27), max(i32(_e33), i32(_e35))));
}

fn AMax3SU2_(x_34: vec2<u32>, y_18: vec2<u32>, z_10: vec2<u32>) -> vec2<u32> {
    var x_35: vec2<u32>;
    var y_19: vec2<u32>;
    var z_11: vec2<u32>;

    x_35 = x_34;
    y_19 = y_18;
    z_11 = z_10;
    let _e16 = x_35;
    let _e18 = y_19;
    let _e20 = z_11;
    let _e22 = y_19;
    let _e24 = z_11;
    let _e27 = x_35;
    let _e29 = y_19;
    let _e31 = z_11;
    let _e33 = y_19;
    let _e35 = z_11;
    return vec2<u32>(max(vec2<i32>(_e27), max(vec2<i32>(_e33), vec2<i32>(_e35))));
}

fn AMax3SU3_(x_36: vec3<u32>, y_20: vec3<u32>, z_12: vec3<u32>) -> vec3<u32> {
    var x_37: vec3<u32>;
    var y_21: vec3<u32>;
    var z_13: vec3<u32>;

    x_37 = x_36;
    y_21 = y_20;
    z_13 = z_12;
    let _e16 = x_37;
    let _e18 = y_21;
    let _e20 = z_13;
    let _e22 = y_21;
    let _e24 = z_13;
    let _e27 = x_37;
    let _e29 = y_21;
    let _e31 = z_13;
    let _e33 = y_21;
    let _e35 = z_13;
    return vec3<u32>(max(vec3<i32>(_e27), max(vec3<i32>(_e33), vec3<i32>(_e35))));
}

fn AMax3SU4_(x_38: vec4<u32>, y_22: vec4<u32>, z_14: vec4<u32>) -> vec4<u32> {
    var x_39: vec4<u32>;
    var y_23: vec4<u32>;
    var z_15: vec4<u32>;

    x_39 = x_38;
    y_23 = y_22;
    z_15 = z_14;
    let _e16 = x_39;
    let _e18 = y_23;
    let _e20 = z_15;
    let _e22 = y_23;
    let _e24 = z_15;
    let _e27 = x_39;
    let _e29 = y_23;
    let _e31 = z_15;
    let _e33 = y_23;
    let _e35 = z_15;
    return vec4<u32>(max(vec4<i32>(_e27), max(vec4<i32>(_e33), vec4<i32>(_e35))));
}

fn AMax3U1_(x_40: u32, y_24: u32, z_16: u32) -> u32 {
    var x_41: u32;
    var y_25: u32;
    var z_17: u32;

    x_41 = x_40;
    y_25 = y_24;
    z_17 = z_16;
    let _e19 = y_25;
    let _e20 = z_17;
    let _e22 = x_41;
    let _e25 = y_25;
    let _e26 = z_17;
    return max(_e22, max(_e25, _e26));
}

fn AMax3U2_(x_42: vec2<u32>, y_26: vec2<u32>, z_18: vec2<u32>) -> vec2<u32> {
    var x_43: vec2<u32>;
    var y_27: vec2<u32>;
    var z_19: vec2<u32>;

    x_43 = x_42;
    y_27 = y_26;
    z_19 = z_18;
    let _e19 = y_27;
    let _e20 = z_19;
    let _e22 = x_43;
    let _e25 = y_27;
    let _e26 = z_19;
    return max(_e22, max(_e25, _e26));
}

fn AMax3U3_(x_44: vec3<u32>, y_28: vec3<u32>, z_20: vec3<u32>) -> vec3<u32> {
    var x_45: vec3<u32>;
    var y_29: vec3<u32>;
    var z_21: vec3<u32>;

    x_45 = x_44;
    y_29 = y_28;
    z_21 = z_20;
    let _e19 = y_29;
    let _e20 = z_21;
    let _e22 = x_45;
    let _e25 = y_29;
    let _e26 = z_21;
    return max(_e22, max(_e25, _e26));
}

fn AMax3U4_(x_46: vec4<u32>, y_30: vec4<u32>, z_22: vec4<u32>) -> vec4<u32> {
    var x_47: vec4<u32>;
    var y_31: vec4<u32>;
    var z_23: vec4<u32>;

    x_47 = x_46;
    y_31 = y_30;
    z_23 = z_22;
    let _e19 = y_31;
    let _e20 = z_23;
    let _e22 = x_47;
    let _e25 = y_31;
    let _e26 = z_23;
    return max(_e22, max(_e25, _e26));
}

fn AMaxSU1_(a_34: u32, b: u32) -> u32 {
    var a_35: u32;
    var b_1: u32;

    a_35 = a_34;
    b_1 = b;
    let _e14 = a_35;
    let _e16 = b_1;
    let _e18 = a_35;
    let _e20 = b_1;
    return u32(max(i32(_e18), i32(_e20)));
}

fn AMaxSU2_(a_36: vec2<u32>, b_2: vec2<u32>) -> vec2<u32> {
    var a_37: vec2<u32>;
    var b_3: vec2<u32>;

    a_37 = a_36;
    b_3 = b_2;
    let _e14 = a_37;
    let _e16 = b_3;
    let _e18 = a_37;
    let _e20 = b_3;
    return vec2<u32>(max(vec2<i32>(_e18), vec2<i32>(_e20)));
}

fn AMaxSU3_(a_38: vec3<u32>, b_4: vec3<u32>) -> vec3<u32> {
    var a_39: vec3<u32>;
    var b_5: vec3<u32>;

    a_39 = a_38;
    b_5 = b_4;
    let _e14 = a_39;
    let _e16 = b_5;
    let _e18 = a_39;
    let _e20 = b_5;
    return vec3<u32>(max(vec3<i32>(_e18), vec3<i32>(_e20)));
}

fn AMaxSU4_(a_40: vec4<u32>, b_6: vec4<u32>) -> vec4<u32> {
    var a_41: vec4<u32>;
    var b_7: vec4<u32>;

    a_41 = a_40;
    b_7 = b_6;
    let _e14 = a_41;
    let _e16 = b_7;
    let _e18 = a_41;
    let _e20 = b_7;
    return vec4<u32>(max(vec4<i32>(_e18), vec4<i32>(_e20)));
}

fn AMed3F1_(x_48: f32, y_32: f32, z_24: f32) -> f32 {
    var x_49: f32;
    var y_33: f32;
    var z_25: f32;

    x_49 = x_48;
    y_33 = y_32;
    z_25 = z_24;
    let _e18 = x_49;
    let _e19 = y_33;
    let _e23 = x_49;
    let _e24 = y_33;
    let _e29 = x_49;
    let _e30 = y_33;
    let _e32 = z_25;
    let _e36 = x_49;
    let _e37 = y_33;
    let _e41 = x_49;
    let _e42 = y_33;
    let _e47 = x_49;
    let _e48 = y_33;
    let _e50 = z_25;
    return max(min(_e36, _e37), min(max(_e47, _e48), _e50));
}

fn AMed3F2_(x_50: vec2<f32>, y_34: vec2<f32>, z_26: vec2<f32>) -> vec2<f32> {
    var x_51: vec2<f32>;
    var y_35: vec2<f32>;
    var z_27: vec2<f32>;

    x_51 = x_50;
    y_35 = y_34;
    z_27 = z_26;
    let _e18 = x_51;
    let _e19 = y_35;
    let _e23 = x_51;
    let _e24 = y_35;
    let _e29 = x_51;
    let _e30 = y_35;
    let _e32 = z_27;
    let _e36 = x_51;
    let _e37 = y_35;
    let _e41 = x_51;
    let _e42 = y_35;
    let _e47 = x_51;
    let _e48 = y_35;
    let _e50 = z_27;
    return max(min(_e36, _e37), min(max(_e47, _e48), _e50));
}

fn AMed3F3_(x_52: vec3<f32>, y_36: vec3<f32>, z_28: vec3<f32>) -> vec3<f32> {
    var x_53: vec3<f32>;
    var y_37: vec3<f32>;
    var z_29: vec3<f32>;

    x_53 = x_52;
    y_37 = y_36;
    z_29 = z_28;
    let _e18 = x_53;
    let _e19 = y_37;
    let _e23 = x_53;
    let _e24 = y_37;
    let _e29 = x_53;
    let _e30 = y_37;
    let _e32 = z_29;
    let _e36 = x_53;
    let _e37 = y_37;
    let _e41 = x_53;
    let _e42 = y_37;
    let _e47 = x_53;
    let _e48 = y_37;
    let _e50 = z_29;
    return max(min(_e36, _e37), min(max(_e47, _e48), _e50));
}

fn AMed3F4_(x_54: vec4<f32>, y_38: vec4<f32>, z_30: vec4<f32>) -> vec4<f32> {
    var x_55: vec4<f32>;
    var y_39: vec4<f32>;
    var z_31: vec4<f32>;

    x_55 = x_54;
    y_39 = y_38;
    z_31 = z_30;
    let _e18 = x_55;
    let _e19 = y_39;
    let _e23 = x_55;
    let _e24 = y_39;
    let _e29 = x_55;
    let _e30 = y_39;
    let _e32 = z_31;
    let _e36 = x_55;
    let _e37 = y_39;
    let _e41 = x_55;
    let _e42 = y_39;
    let _e47 = x_55;
    let _e48 = y_39;
    let _e50 = z_31;
    return max(min(_e36, _e37), min(max(_e47, _e48), _e50));
}

fn AMin3F1_(x_56: f32, y_40: f32, z_32: f32) -> f32 {
    var x_57: f32;
    var y_41: f32;
    var z_33: f32;

    x_57 = x_56;
    y_41 = y_40;
    z_33 = z_32;
    let _e19 = y_41;
    let _e20 = z_33;
    let _e22 = x_57;
    let _e25 = y_41;
    let _e26 = z_33;
    return min(_e22, min(_e25, _e26));
}

fn AMin3F2_(x_58: vec2<f32>, y_42: vec2<f32>, z_34: vec2<f32>) -> vec2<f32> {
    var x_59: vec2<f32>;
    var y_43: vec2<f32>;
    var z_35: vec2<f32>;

    x_59 = x_58;
    y_43 = y_42;
    z_35 = z_34;
    let _e19 = y_43;
    let _e20 = z_35;
    let _e22 = x_59;
    let _e25 = y_43;
    let _e26 = z_35;
    return min(_e22, min(_e25, _e26));
}

fn AMin3F3_(x_60: vec3<f32>, y_44: vec3<f32>, z_36: vec3<f32>) -> vec3<f32> {
    var x_61: vec3<f32>;
    var y_45: vec3<f32>;
    var z_37: vec3<f32>;

    x_61 = x_60;
    y_45 = y_44;
    z_37 = z_36;
    let _e19 = y_45;
    let _e20 = z_37;
    let _e22 = x_61;
    let _e25 = y_45;
    let _e26 = z_37;
    return min(_e22, min(_e25, _e26));
}

fn AMin3F4_(x_62: vec4<f32>, y_46: vec4<f32>, z_38: vec4<f32>) -> vec4<f32> {
    var x_63: vec4<f32>;
    var y_47: vec4<f32>;
    var z_39: vec4<f32>;

    x_63 = x_62;
    y_47 = y_46;
    z_39 = z_38;
    let _e19 = y_47;
    let _e20 = z_39;
    let _e22 = x_63;
    let _e25 = y_47;
    let _e26 = z_39;
    return min(_e22, min(_e25, _e26));
}

fn AMin3SU1_(x_64: u32, y_48: u32, z_40: u32) -> u32 {
    var x_65: u32;
    var y_49: u32;
    var z_41: u32;

    x_65 = x_64;
    y_49 = y_48;
    z_41 = z_40;
    let _e16 = x_65;
    let _e18 = y_49;
    let _e20 = z_41;
    let _e22 = y_49;
    let _e24 = z_41;
    let _e27 = x_65;
    let _e29 = y_49;
    let _e31 = z_41;
    let _e33 = y_49;
    let _e35 = z_41;
    return u32(min(i32(_e27), min(i32(_e33), i32(_e35))));
}

fn AMin3SU2_(x_66: vec2<u32>, y_50: vec2<u32>, z_42: vec2<u32>) -> vec2<u32> {
    var x_67: vec2<u32>;
    var y_51: vec2<u32>;
    var z_43: vec2<u32>;

    x_67 = x_66;
    y_51 = y_50;
    z_43 = z_42;
    let _e16 = x_67;
    let _e18 = y_51;
    let _e20 = z_43;
    let _e22 = y_51;
    let _e24 = z_43;
    let _e27 = x_67;
    let _e29 = y_51;
    let _e31 = z_43;
    let _e33 = y_51;
    let _e35 = z_43;
    return vec2<u32>(min(vec2<i32>(_e27), min(vec2<i32>(_e33), vec2<i32>(_e35))));
}

fn AMin3SU3_(x_68: vec3<u32>, y_52: vec3<u32>, z_44: vec3<u32>) -> vec3<u32> {
    var x_69: vec3<u32>;
    var y_53: vec3<u32>;
    var z_45: vec3<u32>;

    x_69 = x_68;
    y_53 = y_52;
    z_45 = z_44;
    let _e16 = x_69;
    let _e18 = y_53;
    let _e20 = z_45;
    let _e22 = y_53;
    let _e24 = z_45;
    let _e27 = x_69;
    let _e29 = y_53;
    let _e31 = z_45;
    let _e33 = y_53;
    let _e35 = z_45;
    return vec3<u32>(min(vec3<i32>(_e27), min(vec3<i32>(_e33), vec3<i32>(_e35))));
}

fn AMin3SU4_(x_70: vec4<u32>, y_54: vec4<u32>, z_46: vec4<u32>) -> vec4<u32> {
    var x_71: vec4<u32>;
    var y_55: vec4<u32>;
    var z_47: vec4<u32>;

    x_71 = x_70;
    y_55 = y_54;
    z_47 = z_46;
    let _e16 = x_71;
    let _e18 = y_55;
    let _e20 = z_47;
    let _e22 = y_55;
    let _e24 = z_47;
    let _e27 = x_71;
    let _e29 = y_55;
    let _e31 = z_47;
    let _e33 = y_55;
    let _e35 = z_47;
    return vec4<u32>(min(vec4<i32>(_e27), min(vec4<i32>(_e33), vec4<i32>(_e35))));
}

fn AMin3U1_(x_72: u32, y_56: u32, z_48: u32) -> u32 {
    var x_73: u32;
    var y_57: u32;
    var z_49: u32;

    x_73 = x_72;
    y_57 = y_56;
    z_49 = z_48;
    let _e19 = y_57;
    let _e20 = z_49;
    let _e22 = x_73;
    let _e25 = y_57;
    let _e26 = z_49;
    return min(_e22, min(_e25, _e26));
}

fn AMin3U2_(x_74: vec2<u32>, y_58: vec2<u32>, z_50: vec2<u32>) -> vec2<u32> {
    var x_75: vec2<u32>;
    var y_59: vec2<u32>;
    var z_51: vec2<u32>;

    x_75 = x_74;
    y_59 = y_58;
    z_51 = z_50;
    let _e19 = y_59;
    let _e20 = z_51;
    let _e22 = x_75;
    let _e25 = y_59;
    let _e26 = z_51;
    return min(_e22, min(_e25, _e26));
}

fn AMin3U3_(x_76: vec3<u32>, y_60: vec3<u32>, z_52: vec3<u32>) -> vec3<u32> {
    var x_77: vec3<u32>;
    var y_61: vec3<u32>;
    var z_53: vec3<u32>;

    x_77 = x_76;
    y_61 = y_60;
    z_53 = z_52;
    let _e19 = y_61;
    let _e20 = z_53;
    let _e22 = x_77;
    let _e25 = y_61;
    let _e26 = z_53;
    return min(_e22, min(_e25, _e26));
}

fn AMin3U4_(x_78: vec4<u32>, y_62: vec4<u32>, z_54: vec4<u32>) -> vec4<u32> {
    var x_79: vec4<u32>;
    var y_63: vec4<u32>;
    var z_55: vec4<u32>;

    x_79 = x_78;
    y_63 = y_62;
    z_55 = z_54;
    let _e19 = y_63;
    let _e20 = z_55;
    let _e22 = x_79;
    let _e25 = y_63;
    let _e26 = z_55;
    return min(_e22, min(_e25, _e26));
}

fn AMinSU1_(a_42: u32, b_8: u32) -> u32 {
    var a_43: u32;
    var b_9: u32;

    a_43 = a_42;
    b_9 = b_8;
    let _e14 = a_43;
    let _e16 = b_9;
    let _e18 = a_43;
    let _e20 = b_9;
    return u32(min(i32(_e18), i32(_e20)));
}

fn AMinSU2_(a_44: vec2<u32>, b_10: vec2<u32>) -> vec2<u32> {
    var a_45: vec2<u32>;
    var b_11: vec2<u32>;

    a_45 = a_44;
    b_11 = b_10;
    let _e14 = a_45;
    let _e16 = b_11;
    let _e18 = a_45;
    let _e20 = b_11;
    return vec2<u32>(min(vec2<i32>(_e18), vec2<i32>(_e20)));
}

fn AMinSU3_(a_46: vec3<u32>, b_12: vec3<u32>) -> vec3<u32> {
    var a_47: vec3<u32>;
    var b_13: vec3<u32>;

    a_47 = a_46;
    b_13 = b_12;
    let _e14 = a_47;
    let _e16 = b_13;
    let _e18 = a_47;
    let _e20 = b_13;
    return vec3<u32>(min(vec3<i32>(_e18), vec3<i32>(_e20)));
}

fn AMinSU4_(a_48: vec4<u32>, b_14: vec4<u32>) -> vec4<u32> {
    var a_49: vec4<u32>;
    var b_15: vec4<u32>;

    a_49 = a_48;
    b_15 = b_14;
    let _e14 = a_49;
    let _e16 = b_15;
    let _e18 = a_49;
    let _e20 = b_15;
    return vec4<u32>(min(vec4<i32>(_e18), vec4<i32>(_e20)));
}

fn ANCosF1_(x_80: f32) -> f32 {
    var x_81: f32;

    x_81 = x_80;
    let _e12 = x_81;
    let _e17 = AF1_x(6.2831855f);
    let _e19 = x_81;
    let _e24 = AF1_x(6.2831855f);
    return cos((_e19 * _e24));
}

fn ANCosF2_(x_82: vec2<f32>) -> vec2<f32> {
    var x_83: vec2<f32>;

    x_83 = x_82;
    let _e12 = x_83;
    let _e17 = AF2_x(6.2831855f);
    let _e19 = x_83;
    let _e24 = AF2_x(6.2831855f);
    return cos((_e19 * _e24));
}

fn ANCosF3_(x_84: vec3<f32>) -> vec3<f32> {
    var x_85: vec3<f32>;

    x_85 = x_84;
    let _e12 = x_85;
    let _e17 = AF3_x(6.2831855f);
    let _e19 = x_85;
    let _e24 = AF3_x(6.2831855f);
    return cos((_e19 * _e24));
}

fn ANCosF4_(x_86: vec4<f32>) -> vec4<f32> {
    var x_87: vec4<f32>;

    x_87 = x_86;
    let _e12 = x_87;
    let _e17 = AF4_x(6.2831855f);
    let _e19 = x_87;
    let _e24 = AF4_x(6.2831855f);
    return cos((_e19 * _e24));
}

fn ANSinF1_(x_88: f32) -> f32 {
    var x_89: f32;

    x_89 = x_88;
    let _e12 = x_89;
    let _e17 = AF1_x(6.2831855f);
    let _e19 = x_89;
    let _e24 = AF1_x(6.2831855f);
    return sin((_e19 * _e24));
}

fn ANSinF2_(x_90: vec2<f32>) -> vec2<f32> {
    var x_91: vec2<f32>;

    x_91 = x_90;
    let _e12 = x_91;
    let _e17 = AF2_x(6.2831855f);
    let _e19 = x_91;
    let _e24 = AF2_x(6.2831855f);
    return sin((_e19 * _e24));
}

fn ANSinF3_(x_92: vec3<f32>) -> vec3<f32> {
    var x_93: vec3<f32>;

    x_93 = x_92;
    let _e12 = x_93;
    let _e17 = AF3_x(6.2831855f);
    let _e19 = x_93;
    let _e24 = AF3_x(6.2831855f);
    return sin((_e19 * _e24));
}

fn ANSinF4_(x_94: vec4<f32>) -> vec4<f32> {
    var x_95: vec4<f32>;

    x_95 = x_94;
    let _e12 = x_95;
    let _e17 = AF4_x(6.2831855f);
    let _e19 = x_95;
    let _e24 = AF4_x(6.2831855f);
    return sin((_e19 * _e24));
}

fn ARcpF1_(x_96: f32) -> f32 {
    var x_97: f32;

    x_97 = x_96;
    let _e16 = AF1_x(1f);
    let _e17 = x_97;
    return (_e16 / _e17);
}

fn ARcpF2_(x_98: vec2<f32>) -> vec2<f32> {
    var x_99: vec2<f32>;

    x_99 = x_98;
    let _e16 = AF2_x(1f);
    let _e17 = x_99;
    return (_e16 / _e17);
}

fn ARcpF3_(x_100: vec3<f32>) -> vec3<f32> {
    var x_101: vec3<f32>;

    x_101 = x_100;
    let _e16 = AF3_x(1f);
    let _e17 = x_101;
    return (_e16 / _e17);
}

fn ARcpF4_(x_102: vec4<f32>) -> vec4<f32> {
    var x_103: vec4<f32>;

    x_103 = x_102;
    let _e16 = AF4_x(1f);
    let _e17 = x_103;
    return (_e16 / _e17);
}

fn ARsqF1_(x_104: f32) -> f32 {
    var x_105: f32;

    x_105 = x_104;
    let _e16 = AF1_x(1f);
    let _e18 = x_105;
    return (_e16 / sqrt(_e18));
}

fn ARsqF2_(x_106: vec2<f32>) -> vec2<f32> {
    var x_107: vec2<f32>;

    x_107 = x_106;
    let _e16 = AF2_x(1f);
    let _e18 = x_107;
    return (_e16 / sqrt(_e18));
}

fn ARsqF3_(x_108: vec3<f32>) -> vec3<f32> {
    var x_109: vec3<f32>;

    x_109 = x_108;
    let _e16 = AF3_x(1f);
    let _e18 = x_109;
    return (_e16 / sqrt(_e18));
}

fn ARsqF4_(x_110: vec4<f32>) -> vec4<f32> {
    var x_111: vec4<f32>;

    x_111 = x_110;
    let _e16 = AF4_x(1f);
    let _e18 = x_111;
    return (_e16 / sqrt(_e18));
}

fn ASatF1_(x_112: f32) -> f32 {
    var x_113: f32;

    x_113 = x_112;
    let _e17 = AF1_x(0f);
    let _e22 = AF1_x(1f);
    let _e23 = x_113;
    let _e28 = AF1_x(0f);
    let _e33 = AF1_x(1f);
    return clamp(_e23, _e28, _e33);
}

fn ASatF2_(x_114: vec2<f32>) -> vec2<f32> {
    var x_115: vec2<f32>;

    x_115 = x_114;
    let _e17 = AF2_x(0f);
    let _e22 = AF2_x(1f);
    let _e23 = x_115;
    let _e28 = AF2_x(0f);
    let _e33 = AF2_x(1f);
    return clamp(_e23, _e28, _e33);
}

fn ASatF3_(x_116: vec3<f32>) -> vec3<f32> {
    var x_117: vec3<f32>;

    x_117 = x_116;
    let _e17 = AF3_x(0f);
    let _e22 = AF3_x(1f);
    let _e23 = x_117;
    let _e28 = AF3_x(0f);
    let _e33 = AF3_x(1f);
    return clamp(_e23, _e28, _e33);
}

fn ASatF4_(x_118: vec4<f32>) -> vec4<f32> {
    var x_119: vec4<f32>;

    x_119 = x_118;
    let _e17 = AF4_x(0f);
    let _e22 = AF4_x(1f);
    let _e23 = x_119;
    let _e28 = AF4_x(0f);
    let _e33 = AF4_x(1f);
    return clamp(_e23, _e28, _e33);
}

fn AShrSU1_(a_50: u32, b_16: u32) -> u32 {
    var a_51: u32;
    var b_17: u32;

    a_51 = a_50;
    b_17 = b_16;
    let _e14 = a_51;
    let _e16 = b_17;
    return u32((i32(_e14) >> u32(i32(_e16))));
}

fn AShrSU2_(a_52: vec2<u32>, b_18: vec2<u32>) -> vec2<u32> {
    var a_53: vec2<u32>;
    var b_19: vec2<u32>;

    a_53 = a_52;
    b_19 = b_18;
    let _e14 = a_53;
    let _e16 = b_19;
    return vec2<u32>((vec2<i32>(_e14) >> vec2<u32>(vec2<i32>(_e16))));
}

fn AShrSU3_(a_54: vec3<u32>, b_20: vec3<u32>) -> vec3<u32> {
    var a_55: vec3<u32>;
    var b_21: vec3<u32>;

    a_55 = a_54;
    b_21 = b_20;
    let _e14 = a_55;
    let _e16 = b_21;
    return vec3<u32>((vec3<i32>(_e14) >> vec3<u32>(vec3<i32>(_e16))));
}

fn AShrSU4_(a_56: vec4<u32>, b_22: vec4<u32>) -> vec4<u32> {
    var a_57: vec4<u32>;
    var b_23: vec4<u32>;

    a_57 = a_56;
    b_23 = b_22;
    let _e14 = a_57;
    let _e16 = b_23;
    return vec4<u32>((vec4<i32>(_e14) >> vec4<u32>(vec4<i32>(_e16))));
}

fn ACpySgnF1_(d: f32, s: f32) -> f32 {
    var d_1: f32;
    var s_1: f32;

    d_1 = d;
    s_1 = s;
    let _e14 = d_1;
    let _e16 = d_1;
    let _e19 = s_1;
    let _e21 = s_1;
    let _e28 = AU1_x(2147483648u);
    let _e32 = d_1;
    let _e34 = d_1;
    let _e37 = s_1;
    let _e39 = s_1;
    let _e46 = AU1_x(2147483648u);
    return bitcast<f32>(u32((bitcast<u32>(f32(_e34)) | (bitcast<u32>(f32(_e39)) & _e46))));
}

fn ACpySgnF2_(d_2: vec2<f32>, s_2: vec2<f32>) -> vec2<f32> {
    var d_3: vec2<f32>;
    var s_3: vec2<f32>;

    d_3 = d_2;
    s_3 = s_2;
    let _e14 = d_3;
    let _e16 = d_3;
    let _e19 = s_3;
    let _e21 = s_3;
    let _e28 = AU2_x(2147483648u);
    let _e32 = d_3;
    let _e34 = d_3;
    let _e37 = s_3;
    let _e39 = s_3;
    let _e46 = AU2_x(2147483648u);
    return bitcast<vec2<f32>>(vec2<u32>((bitcast<vec2<u32>>(vec2<f32>(_e34)) | (bitcast<vec2<u32>>(vec2<f32>(_e39)) & _e46))));
}

fn ACpySgnF3_(d_4: vec3<f32>, s_4: vec3<f32>) -> vec3<f32> {
    var d_5: vec3<f32>;
    var s_5: vec3<f32>;

    d_5 = d_4;
    s_5 = s_4;
    let _e14 = d_5;
    let _e16 = d_5;
    let _e19 = s_5;
    let _e21 = s_5;
    let _e28 = AU3_x(2147483648u);
    let _e32 = d_5;
    let _e34 = d_5;
    let _e37 = s_5;
    let _e39 = s_5;
    let _e46 = AU3_x(2147483648u);
    return bitcast<vec3<f32>>(vec3<u32>((bitcast<vec3<u32>>(vec3<f32>(_e34)) | (bitcast<vec3<u32>>(vec3<f32>(_e39)) & _e46))));
}

fn ACpySgnF4_(d_6: vec4<f32>, s_6: vec4<f32>) -> vec4<f32> {
    var d_7: vec4<f32>;
    var s_7: vec4<f32>;

    d_7 = d_6;
    s_7 = s_6;
    let _e14 = d_7;
    let _e16 = d_7;
    let _e19 = s_7;
    let _e21 = s_7;
    let _e28 = AU4_x(2147483648u);
    let _e32 = d_7;
    let _e34 = d_7;
    let _e37 = s_7;
    let _e39 = s_7;
    let _e46 = AU4_x(2147483648u);
    return bitcast<vec4<f32>>(vec4<u32>((bitcast<vec4<u32>>(vec4<f32>(_e34)) | (bitcast<vec4<u32>>(vec4<f32>(_e39)) & _e46))));
}

fn ASignedF1_(m_8: f32) -> f32 {
    var m_9: f32;

    m_9 = m_8;
    let _e12 = m_9;
    let _e25 = AF1_x(f32(bitcast<f32>(4286578688u)));
    let _e27 = m_9;
    let _e40 = AF1_x(f32(bitcast<f32>(4286578688u)));
    let _e42 = ASatF1_((_e27 * _e40));
    return _e42;
}

fn ASignedF2_(m_10: vec2<f32>) -> vec2<f32> {
    var m_11: vec2<f32>;

    m_11 = m_10;
    let _e12 = m_11;
    let _e25 = AF2_x(f32(bitcast<f32>(4286578688u)));
    let _e27 = m_11;
    let _e40 = AF2_x(f32(bitcast<f32>(4286578688u)));
    let _e42 = ASatF2_((_e27 * _e40));
    return _e42;
}

fn ASignedF3_(m_12: vec3<f32>) -> vec3<f32> {
    var m_13: vec3<f32>;

    m_13 = m_12;
    let _e12 = m_13;
    let _e25 = AF3_x(f32(bitcast<f32>(4286578688u)));
    let _e27 = m_13;
    let _e40 = AF3_x(f32(bitcast<f32>(4286578688u)));
    let _e42 = ASatF3_((_e27 * _e40));
    return _e42;
}

fn ASignedF4_(m_14: vec4<f32>) -> vec4<f32> {
    var m_15: vec4<f32>;

    m_15 = m_14;
    let _e12 = m_15;
    let _e25 = AF4_x(f32(bitcast<f32>(4286578688u)));
    let _e27 = m_15;
    let _e40 = AF4_x(f32(bitcast<f32>(4286578688u)));
    let _e42 = ASatF4_((_e27 * _e40));
    return _e42;
}

fn AGtZeroF1_(m_16: f32) -> f32 {
    var m_17: f32;

    m_17 = m_16;
    let _e12 = m_17;
    let _e25 = AF1_x(f32(bitcast<f32>(2139095040u)));
    let _e27 = m_17;
    let _e40 = AF1_x(f32(bitcast<f32>(2139095040u)));
    let _e42 = ASatF1_((_e27 * _e40));
    return _e42;
}

fn AGtZeroF2_(m_18: vec2<f32>) -> vec2<f32> {
    var m_19: vec2<f32>;

    m_19 = m_18;
    let _e12 = m_19;
    let _e25 = AF2_x(f32(bitcast<f32>(2139095040u)));
    let _e27 = m_19;
    let _e40 = AF2_x(f32(bitcast<f32>(2139095040u)));
    let _e42 = ASatF2_((_e27 * _e40));
    return _e42;
}

fn AGtZeroF3_(m_20: vec3<f32>) -> vec3<f32> {
    var m_21: vec3<f32>;

    m_21 = m_20;
    let _e12 = m_21;
    let _e25 = AF3_x(f32(bitcast<f32>(2139095040u)));
    let _e27 = m_21;
    let _e40 = AF3_x(f32(bitcast<f32>(2139095040u)));
    let _e42 = ASatF3_((_e27 * _e40));
    return _e42;
}

fn AGtZeroF4_(m_22: vec4<f32>) -> vec4<f32> {
    var m_23: vec4<f32>;

    m_23 = m_22;
    let _e12 = m_23;
    let _e25 = AF4_x(f32(bitcast<f32>(2139095040u)));
    let _e27 = m_23;
    let _e40 = AF4_x(f32(bitcast<f32>(2139095040u)));
    let _e42 = ASatF4_((_e27 * _e40));
    return _e42;
}

fn AFisToU1_(x_120: u32) -> u32 {
    var x_121: u32;

    x_121 = x_120;
    let _e12 = x_121;
    let _e18 = AU1_x(31u);
    let _e19 = x_121;
    let _e24 = AU1_x(31u);
    let _e25 = AShrSU1_(_e19, _e24);
    let _e30 = AU1_x(2147483648u);
    return (_e12 ^ (_e25 | _e30));
}

fn AFisFromU1_(x_122: u32) -> u32 {
    var x_123: u32;

    x_123 = x_122;
    let _e12 = x_123;
    let _e18 = AU1_x(31u);
    let _e19 = x_123;
    let _e24 = AU1_x(31u);
    let _e25 = AShrSU1_(_e19, _e24);
    let _e31 = AU1_x(2147483648u);
    return (_e12 ^ (~(_e25) | _e31));
}

fn AFisToHiU1_(x_124: u32) -> u32 {
    var x_125: u32;

    x_125 = x_124;
    let _e12 = x_125;
    let _e18 = AU1_x(15u);
    let _e19 = x_125;
    let _e24 = AU1_x(15u);
    let _e25 = AShrSU1_(_e19, _e24);
    let _e30 = AU1_x(2147483648u);
    return (_e12 ^ (_e25 | _e30));
}

fn AFisFromHiU1_(x_126: u32) -> u32 {
    var x_127: u32;

    x_127 = x_126;
    let _e12 = x_127;
    let _e18 = AU1_x(15u);
    let _e19 = x_127;
    let _e24 = AU1_x(15u);
    let _e25 = AShrSU1_(_e19, _e24);
    let _e31 = AU1_x(2147483648u);
    return (_e12 ^ (~(_e25) | _e31));
}

fn ABuc0ToU1_(d_8: u32, i: f32) -> u32 {
    var d_9: u32;
    var i_1: f32;

    d_9 = d_8;
    i_1 = i;
    let _e14 = d_9;
    let _e17 = i_1;
    let _e20 = i_1;
    return ((_e14 & 4294967040u) | (min(u32(_e20), 255u) & 255u));
}

fn ABuc1ToU1_(d_10: u32, i_2: f32) -> u32 {
    var d_11: u32;
    var i_3: f32;

    d_11 = d_10;
    i_3 = i_2;
    let _e14 = d_11;
    let _e17 = i_3;
    let _e20 = i_3;
    return ((_e14 & 4294902015u) | ((min(u32(_e20), 255u) << 8u) & 65280u));
}

fn ABuc2ToU1_(d_12: u32, i_4: f32) -> u32 {
    var d_13: u32;
    var i_5: f32;

    d_13 = d_12;
    i_5 = i_4;
    let _e14 = d_13;
    let _e17 = i_5;
    let _e20 = i_5;
    return ((_e14 & 4278255615u) | ((min(u32(_e20), 255u) << 16u) & 16711680u));
}

fn ABuc3ToU1_(d_14: u32, i_6: f32) -> u32 {
    var d_15: u32;
    var i_7: f32;

    d_15 = d_14;
    i_7 = i_6;
    let _e14 = d_15;
    let _e17 = i_7;
    let _e20 = i_7;
    return ((_e14 & 16777215u) | ((min(u32(_e20), 255u) << 24u) & 4278190080u));
}

fn ABuc0FromU1_(i_8: u32) -> f32 {
    var i_9: u32;

    i_9 = i_8;
    let _e12 = i_9;
    return f32((_e12 & 255u));
}

fn ABuc1FromU1_(i_10: u32) -> f32 {
    var i_11: u32;

    i_11 = i_10;
    let _e12 = i_11;
    return f32(((_e12 >> 8u) & 255u));
}

fn ABuc2FromU1_(i_12: u32) -> f32 {
    var i_13: u32;

    i_13 = i_12;
    let _e12 = i_13;
    return f32(((_e12 >> 16u) & 255u));
}

fn ABuc3FromU1_(i_14: u32) -> f32 {
    var i_15: u32;

    i_15 = i_14;
    let _e12 = i_15;
    return f32(((_e12 >> 24u) & 255u));
}

fn ABsc0ToU1_(d_16: u32, i_16: f32) -> u32 {
    var d_17: u32;
    var i_17: f32;

    d_17 = d_16;
    i_17 = i_16;
    let _e14 = d_17;
    let _e17 = i_17;
    let _e22 = i_17;
    return ((_e14 & 4294967040u) | (min(u32((_e22 + 128f)), 255u) & 255u));
}

fn ABsc1ToU1_(d_18: u32, i_18: f32) -> u32 {
    var d_19: u32;
    var i_19: f32;

    d_19 = d_18;
    i_19 = i_18;
    let _e14 = d_19;
    let _e17 = i_19;
    let _e22 = i_19;
    return ((_e14 & 4294902015u) | ((min(u32((_e22 + 128f)), 255u) << 8u) & 65280u));
}

fn ABsc2ToU1_(d_20: u32, i_20: f32) -> u32 {
    var d_21: u32;
    var i_21: f32;

    d_21 = d_20;
    i_21 = i_20;
    let _e14 = d_21;
    let _e17 = i_21;
    let _e22 = i_21;
    return ((_e14 & 4278255615u) | ((min(u32((_e22 + 128f)), 255u) << 16u) & 16711680u));
}

fn ABsc3ToU1_(d_22: u32, i_22: f32) -> u32 {
    var d_23: u32;
    var i_23: f32;

    d_23 = d_22;
    i_23 = i_22;
    let _e14 = d_23;
    let _e17 = i_23;
    let _e22 = i_23;
    return ((_e14 & 16777215u) | ((min(u32((_e22 + 128f)), 255u) << 24u) & 4278190080u));
}

fn ABsc0ToZbU1_(d_24: u32, i_24: f32) -> u32 {
    var d_25: u32;
    var i_25: f32;

    d_25 = d_24;
    i_25 = i_24;
    let _e14 = d_25;
    let _e18 = i_25;
    let _e25 = i_25;
    return (((_e14 & 4294967040u) | (min(u32((trunc(_e25) + 128f)), 255u) & 255u)) ^ 128u);
}

fn ABsc1ToZbU1_(d_26: u32, i_26: f32) -> u32 {
    var d_27: u32;
    var i_27: f32;

    d_27 = d_26;
    i_27 = i_26;
    let _e14 = d_27;
    let _e18 = i_27;
    let _e25 = i_27;
    return (((_e14 & 4294902015u) | ((min(u32((trunc(_e25) + 128f)), 255u) << 8u) & 65280u)) ^ 32768u);
}

fn ABsc2ToZbU1_(d_28: u32, i_28: f32) -> u32 {
    var d_29: u32;
    var i_29: f32;

    d_29 = d_28;
    i_29 = i_28;
    let _e14 = d_29;
    let _e18 = i_29;
    let _e25 = i_29;
    return (((_e14 & 4278255615u) | ((min(u32((trunc(_e25) + 128f)), 255u) << 16u) & 16711680u)) ^ 8388608u);
}

fn ABsc3ToZbU1_(d_30: u32, i_30: f32) -> u32 {
    var d_31: u32;
    var i_31: f32;

    d_31 = d_30;
    i_31 = i_30;
    let _e14 = d_31;
    let _e18 = i_31;
    let _e25 = i_31;
    return (((_e14 & 16777215u) | ((min(u32((trunc(_e25) + 128f)), 255u) << 24u) & 4278190080u)) ^ 2147483648u);
}

fn ABsc0FromU1_(i_32: u32) -> f32 {
    var i_33: u32;

    i_33 = i_32;
    let _e12 = i_33;
    return (f32((_e12 & 255u)) - 128f);
}

fn ABsc1FromU1_(i_34: u32) -> f32 {
    var i_35: u32;

    i_35 = i_34;
    let _e12 = i_35;
    return (f32(((_e12 >> 8u) & 255u)) - 128f);
}

fn ABsc2FromU1_(i_36: u32) -> f32 {
    var i_37: u32;

    i_37 = i_36;
    let _e12 = i_37;
    return (f32(((_e12 >> 16u) & 255u)) - 128f);
}

fn ABsc3FromU1_(i_38: u32) -> f32 {
    var i_39: u32;

    i_39 = i_38;
    let _e12 = i_39;
    return (f32(((_e12 >> 24u) & 255u)) - 128f);
}

fn ABsc0FromZbU1_(i_40: u32) -> f32 {
    var i_41: u32;

    i_41 = i_40;
    let _e12 = i_41;
    return (f32(((_e12 & 255u) ^ 128u)) - 128f);
}

fn ABsc1FromZbU1_(i_42: u32) -> f32 {
    var i_43: u32;

    i_43 = i_42;
    let _e12 = i_43;
    return (f32((((_e12 >> 8u) & 255u) ^ 128u)) - 128f);
}

fn ABsc2FromZbU1_(i_44: u32) -> f32 {
    var i_45: u32;

    i_45 = i_44;
    let _e12 = i_45;
    return (f32((((_e12 >> 16u) & 255u) ^ 128u)) - 128f);
}

fn ABsc3FromZbU1_(i_46: u32) -> f32 {
    var i_47: u32;

    i_47 = i_46;
    let _e12 = i_47;
    return (f32((((_e12 >> 24u) & 255u) ^ 128u)) - 128f);
}

fn APrxLoSqrtF1_(a_58: f32) -> f32 {
    var a_59: f32;

    a_59 = a_58;
    let _e12 = a_59;
    let _e14 = a_59;
    let _e21 = AU1_x(1u);
    let _e27 = AU1_x(532432441u);
    let _e30 = a_59;
    let _e32 = a_59;
    let _e39 = AU1_x(1u);
    let _e45 = AU1_x(532432441u);
    return bitcast<f32>(u32(((bitcast<u32>(f32(_e32)) >> _e39) + _e45)));
}

fn APrxLoRcpF1_(a_60: f32) -> f32 {
    var a_61: f32;

    a_61 = a_60;
    let _e16 = AU1_x(2129690299u);
    let _e17 = a_61;
    let _e19 = a_61;
    let _e28 = AU1_x(2129690299u);
    let _e29 = a_61;
    let _e31 = a_61;
    return bitcast<f32>(u32((_e28 - bitcast<u32>(f32(_e31)))));
}

fn APrxMedRcpF1_(a_62: f32) -> f32 {
    var a_63: f32;
    var b_24: f32;

    a_63 = a_62;
    let _e16 = AU1_x(2129764351u);
    let _e17 = a_63;
    let _e19 = a_63;
    let _e28 = AU1_x(2129764351u);
    let _e29 = a_63;
    let _e31 = a_63;
    b_24 = bitcast<f32>(u32((_e28 - bitcast<u32>(f32(_e31)))));
    let _e38 = b_24;
    let _e39 = b_24;
    let _e41 = a_63;
    let _e47 = AF1_x(2f);
    return (_e38 * ((-(_e39) * _e41) + _e47));
}

fn APrxLoRsqF1_(a_64: f32) -> f32 {
    var a_65: f32;

    a_65 = a_64;
    let _e16 = AU1_x(1597275508u);
    let _e17 = a_65;
    let _e19 = a_65;
    let _e26 = AU1_x(1u);
    let _e34 = AU1_x(1597275508u);
    let _e35 = a_65;
    let _e37 = a_65;
    let _e44 = AU1_x(1u);
    return bitcast<f32>(u32((_e34 - (bitcast<u32>(f32(_e37)) >> _e44))));
}

fn APrxLoSqrtF2_(a_66: vec2<f32>) -> vec2<f32> {
    var a_67: vec2<f32>;

    a_67 = a_66;
    let _e12 = a_67;
    let _e14 = a_67;
    let _e21 = AU2_x(1u);
    let _e27 = AU2_x(532432441u);
    let _e30 = a_67;
    let _e32 = a_67;
    let _e39 = AU2_x(1u);
    let _e45 = AU2_x(532432441u);
    return bitcast<vec2<f32>>(vec2<u32>(((bitcast<vec2<u32>>(vec2<f32>(_e32)) >> _e39) + _e45)));
}

fn APrxLoRcpF2_(a_68: vec2<f32>) -> vec2<f32> {
    var a_69: vec2<f32>;

    a_69 = a_68;
    let _e16 = AU2_x(2129690299u);
    let _e17 = a_69;
    let _e19 = a_69;
    let _e28 = AU2_x(2129690299u);
    let _e29 = a_69;
    let _e31 = a_69;
    return bitcast<vec2<f32>>(vec2<u32>((_e28 - bitcast<vec2<u32>>(vec2<f32>(_e31)))));
}

fn APrxMedRcpF2_(a_70: vec2<f32>) -> vec2<f32> {
    var a_71: vec2<f32>;
    var b_25: vec2<f32>;

    a_71 = a_70;
    let _e16 = AU2_x(2129764351u);
    let _e17 = a_71;
    let _e19 = a_71;
    let _e28 = AU2_x(2129764351u);
    let _e29 = a_71;
    let _e31 = a_71;
    b_25 = bitcast<vec2<f32>>(vec2<u32>((_e28 - bitcast<vec2<u32>>(vec2<f32>(_e31)))));
    let _e38 = b_25;
    let _e39 = b_25;
    let _e41 = a_71;
    let _e47 = AF2_x(2f);
    return (_e38 * ((-(_e39) * _e41) + _e47));
}

fn APrxLoRsqF2_(a_72: vec2<f32>) -> vec2<f32> {
    var a_73: vec2<f32>;

    a_73 = a_72;
    let _e16 = AU2_x(1597275508u);
    let _e17 = a_73;
    let _e19 = a_73;
    let _e26 = AU2_x(1u);
    let _e34 = AU2_x(1597275508u);
    let _e35 = a_73;
    let _e37 = a_73;
    let _e44 = AU2_x(1u);
    return bitcast<vec2<f32>>(vec2<u32>((_e34 - (bitcast<vec2<u32>>(vec2<f32>(_e37)) >> _e44))));
}

fn APrxLoSqrtF3_(a_74: vec3<f32>) -> vec3<f32> {
    var a_75: vec3<f32>;

    a_75 = a_74;
    let _e12 = a_75;
    let _e14 = a_75;
    let _e21 = AU3_x(1u);
    let _e27 = AU3_x(532432441u);
    let _e30 = a_75;
    let _e32 = a_75;
    let _e39 = AU3_x(1u);
    let _e45 = AU3_x(532432441u);
    return bitcast<vec3<f32>>(vec3<u32>(((bitcast<vec3<u32>>(vec3<f32>(_e32)) >> _e39) + _e45)));
}

fn APrxLoRcpF3_(a_76: vec3<f32>) -> vec3<f32> {
    var a_77: vec3<f32>;

    a_77 = a_76;
    let _e16 = AU3_x(2129690299u);
    let _e17 = a_77;
    let _e19 = a_77;
    let _e28 = AU3_x(2129690299u);
    let _e29 = a_77;
    let _e31 = a_77;
    return bitcast<vec3<f32>>(vec3<u32>((_e28 - bitcast<vec3<u32>>(vec3<f32>(_e31)))));
}

fn APrxMedRcpF3_(a_78: vec3<f32>) -> vec3<f32> {
    var a_79: vec3<f32>;
    var b_26: vec3<f32>;

    a_79 = a_78;
    let _e16 = AU3_x(2129764351u);
    let _e17 = a_79;
    let _e19 = a_79;
    let _e28 = AU3_x(2129764351u);
    let _e29 = a_79;
    let _e31 = a_79;
    b_26 = bitcast<vec3<f32>>(vec3<u32>((_e28 - bitcast<vec3<u32>>(vec3<f32>(_e31)))));
    let _e38 = b_26;
    let _e39 = b_26;
    let _e41 = a_79;
    let _e47 = AF3_x(2f);
    return (_e38 * ((-(_e39) * _e41) + _e47));
}

fn APrxLoRsqF3_(a_80: vec3<f32>) -> vec3<f32> {
    var a_81: vec3<f32>;

    a_81 = a_80;
    let _e16 = AU3_x(1597275508u);
    let _e17 = a_81;
    let _e19 = a_81;
    let _e26 = AU3_x(1u);
    let _e34 = AU3_x(1597275508u);
    let _e35 = a_81;
    let _e37 = a_81;
    let _e44 = AU3_x(1u);
    return bitcast<vec3<f32>>(vec3<u32>((_e34 - (bitcast<vec3<u32>>(vec3<f32>(_e37)) >> _e44))));
}

fn APrxLoSqrtF4_(a_82: vec4<f32>) -> vec4<f32> {
    var a_83: vec4<f32>;

    a_83 = a_82;
    let _e12 = a_83;
    let _e14 = a_83;
    let _e21 = AU4_x(1u);
    let _e27 = AU4_x(532432441u);
    let _e30 = a_83;
    let _e32 = a_83;
    let _e39 = AU4_x(1u);
    let _e45 = AU4_x(532432441u);
    return bitcast<vec4<f32>>(vec4<u32>(((bitcast<vec4<u32>>(vec4<f32>(_e32)) >> _e39) + _e45)));
}

fn APrxLoRcpF4_(a_84: vec4<f32>) -> vec4<f32> {
    var a_85: vec4<f32>;

    a_85 = a_84;
    let _e16 = AU4_x(2129690299u);
    let _e17 = a_85;
    let _e19 = a_85;
    let _e28 = AU4_x(2129690299u);
    let _e29 = a_85;
    let _e31 = a_85;
    return bitcast<vec4<f32>>(vec4<u32>((_e28 - bitcast<vec4<u32>>(vec4<f32>(_e31)))));
}

fn APrxMedRcpF4_(a_86: vec4<f32>) -> vec4<f32> {
    var a_87: vec4<f32>;
    var b_27: vec4<f32>;

    a_87 = a_86;
    let _e16 = AU4_x(2129764351u);
    let _e17 = a_87;
    let _e19 = a_87;
    let _e28 = AU4_x(2129764351u);
    let _e29 = a_87;
    let _e31 = a_87;
    b_27 = bitcast<vec4<f32>>(vec4<u32>((_e28 - bitcast<vec4<u32>>(vec4<f32>(_e31)))));
    let _e38 = b_27;
    let _e39 = b_27;
    let _e41 = a_87;
    let _e47 = AF4_x(2f);
    return (_e38 * ((-(_e39) * _e41) + _e47));
}

fn APrxLoRsqF4_(a_88: vec4<f32>) -> vec4<f32> {
    var a_89: vec4<f32>;

    a_89 = a_88;
    let _e16 = AU4_x(1597275508u);
    let _e17 = a_89;
    let _e19 = a_89;
    let _e26 = AU4_x(1u);
    let _e34 = AU4_x(1597275508u);
    let _e35 = a_89;
    let _e37 = a_89;
    let _e44 = AU4_x(1u);
    return bitcast<vec4<f32>>(vec4<u32>((_e34 - (bitcast<vec4<u32>>(vec4<f32>(_e37)) >> _e44))));
}

fn Quart(a_90: f32) -> f32 {
    var a_91: f32;

    a_91 = a_90;
    let _e12 = a_91;
    let _e13 = a_91;
    a_91 = (_e12 * _e13);
    let _e15 = a_91;
    let _e16 = a_91;
    return (_e15 * _e16);
}

fn Oct(a_92: f32) -> f32 {
    var a_93: f32;

    a_93 = a_92;
    let _e12 = a_93;
    let _e13 = a_93;
    a_93 = (_e12 * _e13);
    let _e15 = a_93;
    let _e16 = a_93;
    a_93 = (_e15 * _e16);
    let _e18 = a_93;
    let _e19 = a_93;
    return (_e18 * _e19);
}

fn Quart_1(a_94: vec2<f32>) -> vec2<f32> {
    var a_95: vec2<f32>;

    a_95 = a_94;
    let _e12 = a_95;
    let _e13 = a_95;
    a_95 = (_e12 * _e13);
    let _e15 = a_95;
    let _e16 = a_95;
    return (_e15 * _e16);
}

fn Oct_1(a_96: vec2<f32>) -> vec2<f32> {
    var a_97: vec2<f32>;

    a_97 = a_96;
    let _e12 = a_97;
    let _e13 = a_97;
    a_97 = (_e12 * _e13);
    let _e15 = a_97;
    let _e16 = a_97;
    a_97 = (_e15 * _e16);
    let _e18 = a_97;
    let _e19 = a_97;
    return (_e18 * _e19);
}

fn Quart_2(a_98: vec3<f32>) -> vec3<f32> {
    var a_99: vec3<f32>;

    a_99 = a_98;
    let _e12 = a_99;
    let _e13 = a_99;
    a_99 = (_e12 * _e13);
    let _e15 = a_99;
    let _e16 = a_99;
    return (_e15 * _e16);
}

fn Oct_2(a_100: vec3<f32>) -> vec3<f32> {
    var a_101: vec3<f32>;

    a_101 = a_100;
    let _e12 = a_101;
    let _e13 = a_101;
    a_101 = (_e12 * _e13);
    let _e15 = a_101;
    let _e16 = a_101;
    a_101 = (_e15 * _e16);
    let _e18 = a_101;
    let _e19 = a_101;
    return (_e18 * _e19);
}

fn Quart_3(a_102: vec4<f32>) -> vec4<f32> {
    var a_103: vec4<f32>;

    a_103 = a_102;
    let _e12 = a_103;
    let _e13 = a_103;
    a_103 = (_e12 * _e13);
    let _e15 = a_103;
    let _e16 = a_103;
    return (_e15 * _e16);
}

fn Oct_3(a_104: vec4<f32>) -> vec4<f32> {
    var a_105: vec4<f32>;

    a_105 = a_104;
    let _e12 = a_105;
    let _e13 = a_105;
    a_105 = (_e12 * _e13);
    let _e15 = a_105;
    let _e16 = a_105;
    a_105 = (_e15 * _e16);
    let _e18 = a_105;
    let _e19 = a_105;
    return (_e18 * _e19);
}

fn APrxPQToGamma2_(a_106: f32) -> f32 {
    var a_107: f32;

    a_107 = a_106;
    let _e13 = a_107;
    let _e14 = Quart(_e13);
    return _e14;
}

fn APrxPQToLinear(a_108: f32) -> f32 {
    var a_109: f32;

    a_109 = a_108;
    let _e13 = a_109;
    let _e14 = Oct(_e13);
    return _e14;
}

fn APrxLoGamma2ToPQ(a_110: f32) -> f32 {
    var a_111: f32;

    a_111 = a_110;
    let _e12 = a_111;
    let _e14 = a_111;
    let _e21 = AU1_x(2u);
    let _e27 = AU1_x(798641734u);
    let _e30 = a_111;
    let _e32 = a_111;
    let _e39 = AU1_x(2u);
    let _e45 = AU1_x(798641734u);
    return bitcast<f32>(u32(((bitcast<u32>(f32(_e32)) >> _e39) + _e45)));
}

fn APrxMedGamma2ToPQ(a_112: f32) -> f32 {
    var a_113: f32;
    var b_28: f32;
    var b4_: f32;

    a_113 = a_112;
    let _e12 = a_113;
    let _e14 = a_113;
    let _e21 = AU1_x(2u);
    let _e27 = AU1_x(798641734u);
    let _e30 = a_113;
    let _e32 = a_113;
    let _e39 = AU1_x(2u);
    let _e45 = AU1_x(798641734u);
    b_28 = bitcast<f32>(u32(((bitcast<u32>(f32(_e32)) >> _e39) + _e45)));
    let _e51 = b_28;
    let _e52 = Quart(_e51);
    b4_ = _e52;
    let _e54 = b_28;
    let _e55 = b_28;
    let _e56 = b4_;
    let _e57 = a_113;
    let _e64 = AF1_x(4f);
    let _e65 = b4_;
    return (_e54 - ((_e55 * (_e56 - _e57)) / (_e64 * _e65)));
}

fn APrxHighGamma2ToPQ(a_114: f32) -> f32 {
    var a_115: f32;

    a_115 = a_114;
    let _e13 = a_115;
    let _e16 = a_115;
    return sqrt(sqrt(_e16));
}

fn APrxLoLinearToPQ(a_116: f32) -> f32 {
    var a_117: f32;

    a_117 = a_116;
    let _e12 = a_117;
    let _e14 = a_117;
    let _e21 = AU1_x(3u);
    let _e27 = AU1_x(932022051u);
    let _e30 = a_117;
    let _e32 = a_117;
    let _e39 = AU1_x(3u);
    let _e45 = AU1_x(932022051u);
    return bitcast<f32>(u32(((bitcast<u32>(f32(_e32)) >> _e39) + _e45)));
}

fn APrxMedLinearToPQ(a_118: f32) -> f32 {
    var a_119: f32;
    var b_29: f32;
    var b8_: f32;

    a_119 = a_118;
    let _e12 = a_119;
    let _e14 = a_119;
    let _e21 = AU1_x(3u);
    let _e27 = AU1_x(932022051u);
    let _e30 = a_119;
    let _e32 = a_119;
    let _e39 = AU1_x(3u);
    let _e45 = AU1_x(932022051u);
    b_29 = bitcast<f32>(u32(((bitcast<u32>(f32(_e32)) >> _e39) + _e45)));
    let _e51 = b_29;
    let _e52 = Oct(_e51);
    b8_ = _e52;
    let _e54 = b_29;
    let _e55 = b_29;
    let _e56 = b8_;
    let _e57 = a_119;
    let _e64 = AF1_x(8f);
    let _e65 = b8_;
    return (_e54 - ((_e55 * (_e56 - _e57)) / (_e64 * _e65)));
}

fn APrxHighLinearToPQ(a_120: f32) -> f32 {
    var a_121: f32;

    a_121 = a_120;
    let _e13 = a_121;
    let _e16 = a_121;
    let _e20 = a_121;
    let _e23 = a_121;
    return sqrt(sqrt(sqrt(_e23)));
}

fn APrxPQToGamma2_1(a_122: vec2<f32>) -> vec2<f32> {
    var a_123: vec2<f32>;

    a_123 = a_122;
    let _e13 = a_123;
    let _e14 = Quart_1(_e13);
    return _e14;
}

fn APrxPQToLinear_1(a_124: vec2<f32>) -> vec2<f32> {
    var a_125: vec2<f32>;

    a_125 = a_124;
    let _e13 = a_125;
    let _e14 = Oct_1(_e13);
    return _e14;
}

fn APrxLoGamma2ToPQ_1(a_126: vec2<f32>) -> vec2<f32> {
    var a_127: vec2<f32>;

    a_127 = a_126;
    let _e12 = a_127;
    let _e14 = a_127;
    let _e21 = AU2_x(2u);
    let _e27 = AU2_x(798641734u);
    let _e30 = a_127;
    let _e32 = a_127;
    let _e39 = AU2_x(2u);
    let _e45 = AU2_x(798641734u);
    return bitcast<vec2<f32>>(vec2<u32>(((bitcast<vec2<u32>>(vec2<f32>(_e32)) >> _e39) + _e45)));
}

fn APrxMedGamma2ToPQ_1(a_128: vec2<f32>) -> vec2<f32> {
    var a_129: vec2<f32>;
    var b_30: vec2<f32>;
    var b4_1: vec2<f32>;

    a_129 = a_128;
    let _e12 = a_129;
    let _e14 = a_129;
    let _e21 = AU2_x(2u);
    let _e27 = AU2_x(798641734u);
    let _e30 = a_129;
    let _e32 = a_129;
    let _e39 = AU2_x(2u);
    let _e45 = AU2_x(798641734u);
    b_30 = bitcast<vec2<f32>>(vec2<u32>(((bitcast<vec2<u32>>(vec2<f32>(_e32)) >> _e39) + _e45)));
    let _e51 = b_30;
    let _e52 = Quart_1(_e51);
    b4_1 = _e52;
    let _e54 = b_30;
    let _e55 = b_30;
    let _e56 = b4_1;
    let _e57 = a_129;
    let _e64 = AF1_x(4f);
    let _e65 = b4_1;
    return (_e54 - ((_e55 * (_e56 - _e57)) / (_e64 * _e65)));
}

fn APrxHighGamma2ToPQ_1(a_130: vec2<f32>) -> vec2<f32> {
    var a_131: vec2<f32>;

    a_131 = a_130;
    let _e13 = a_131;
    let _e16 = a_131;
    return sqrt(sqrt(_e16));
}

fn APrxLoLinearToPQ_1(a_132: vec2<f32>) -> vec2<f32> {
    var a_133: vec2<f32>;

    a_133 = a_132;
    let _e12 = a_133;
    let _e14 = a_133;
    let _e21 = AU2_x(3u);
    let _e27 = AU2_x(932022051u);
    let _e30 = a_133;
    let _e32 = a_133;
    let _e39 = AU2_x(3u);
    let _e45 = AU2_x(932022051u);
    return bitcast<vec2<f32>>(vec2<u32>(((bitcast<vec2<u32>>(vec2<f32>(_e32)) >> _e39) + _e45)));
}

fn APrxMedLinearToPQ_1(a_134: vec2<f32>) -> vec2<f32> {
    var a_135: vec2<f32>;
    var b_31: vec2<f32>;
    var b8_1: vec2<f32>;

    a_135 = a_134;
    let _e12 = a_135;
    let _e14 = a_135;
    let _e21 = AU2_x(3u);
    let _e27 = AU2_x(932022051u);
    let _e30 = a_135;
    let _e32 = a_135;
    let _e39 = AU2_x(3u);
    let _e45 = AU2_x(932022051u);
    b_31 = bitcast<vec2<f32>>(vec2<u32>(((bitcast<vec2<u32>>(vec2<f32>(_e32)) >> _e39) + _e45)));
    let _e51 = b_31;
    let _e52 = Oct_1(_e51);
    b8_1 = _e52;
    let _e54 = b_31;
    let _e55 = b_31;
    let _e56 = b8_1;
    let _e57 = a_135;
    let _e64 = AF1_x(8f);
    let _e65 = b8_1;
    return (_e54 - ((_e55 * (_e56 - _e57)) / (_e64 * _e65)));
}

fn APrxHighLinearToPQ_1(a_136: vec2<f32>) -> vec2<f32> {
    var a_137: vec2<f32>;

    a_137 = a_136;
    let _e13 = a_137;
    let _e16 = a_137;
    let _e20 = a_137;
    let _e23 = a_137;
    return sqrt(sqrt(sqrt(_e23)));
}

fn APrxPQToGamma2_2(a_138: vec3<f32>) -> vec3<f32> {
    var a_139: vec3<f32>;

    a_139 = a_138;
    let _e13 = a_139;
    let _e14 = Quart_2(_e13);
    return _e14;
}

fn APrxPQToLinear_2(a_140: vec3<f32>) -> vec3<f32> {
    var a_141: vec3<f32>;

    a_141 = a_140;
    let _e13 = a_141;
    let _e14 = Oct_2(_e13);
    return _e14;
}

fn APrxLoGamma2ToPQ_2(a_142: vec3<f32>) -> vec3<f32> {
    var a_143: vec3<f32>;

    a_143 = a_142;
    let _e12 = a_143;
    let _e14 = a_143;
    let _e21 = AU3_x(2u);
    let _e27 = AU3_x(798641734u);
    let _e30 = a_143;
    let _e32 = a_143;
    let _e39 = AU3_x(2u);
    let _e45 = AU3_x(798641734u);
    return bitcast<vec3<f32>>(vec3<u32>(((bitcast<vec3<u32>>(vec3<f32>(_e32)) >> _e39) + _e45)));
}

fn APrxMedGamma2ToPQ_2(a_144: vec3<f32>) -> vec3<f32> {
    var a_145: vec3<f32>;
    var b_32: vec3<f32>;
    var b4_2: vec3<f32>;

    a_145 = a_144;
    let _e12 = a_145;
    let _e14 = a_145;
    let _e21 = AU3_x(2u);
    let _e27 = AU3_x(798641734u);
    let _e30 = a_145;
    let _e32 = a_145;
    let _e39 = AU3_x(2u);
    let _e45 = AU3_x(798641734u);
    b_32 = bitcast<vec3<f32>>(vec3<u32>(((bitcast<vec3<u32>>(vec3<f32>(_e32)) >> _e39) + _e45)));
    let _e51 = b_32;
    let _e52 = Quart_2(_e51);
    b4_2 = _e52;
    let _e54 = b_32;
    let _e55 = b_32;
    let _e56 = b4_2;
    let _e57 = a_145;
    let _e64 = AF1_x(4f);
    let _e65 = b4_2;
    return (_e54 - ((_e55 * (_e56 - _e57)) / (_e64 * _e65)));
}

fn APrxHighGamma2ToPQ_2(a_146: vec3<f32>) -> vec3<f32> {
    var a_147: vec3<f32>;

    a_147 = a_146;
    let _e13 = a_147;
    let _e16 = a_147;
    return sqrt(sqrt(_e16));
}

fn APrxLoLinearToPQ_2(a_148: vec3<f32>) -> vec3<f32> {
    var a_149: vec3<f32>;

    a_149 = a_148;
    let _e12 = a_149;
    let _e14 = a_149;
    let _e21 = AU3_x(3u);
    let _e27 = AU3_x(932022051u);
    let _e30 = a_149;
    let _e32 = a_149;
    let _e39 = AU3_x(3u);
    let _e45 = AU3_x(932022051u);
    return bitcast<vec3<f32>>(vec3<u32>(((bitcast<vec3<u32>>(vec3<f32>(_e32)) >> _e39) + _e45)));
}

fn APrxMedLinearToPQ_2(a_150: vec3<f32>) -> vec3<f32> {
    var a_151: vec3<f32>;
    var b_33: vec3<f32>;
    var b8_2: vec3<f32>;

    a_151 = a_150;
    let _e12 = a_151;
    let _e14 = a_151;
    let _e21 = AU3_x(3u);
    let _e27 = AU3_x(932022051u);
    let _e30 = a_151;
    let _e32 = a_151;
    let _e39 = AU3_x(3u);
    let _e45 = AU3_x(932022051u);
    b_33 = bitcast<vec3<f32>>(vec3<u32>(((bitcast<vec3<u32>>(vec3<f32>(_e32)) >> _e39) + _e45)));
    let _e51 = b_33;
    let _e52 = Oct_2(_e51);
    b8_2 = _e52;
    let _e54 = b_33;
    let _e55 = b_33;
    let _e56 = b8_2;
    let _e57 = a_151;
    let _e64 = AF1_x(8f);
    let _e65 = b8_2;
    return (_e54 - ((_e55 * (_e56 - _e57)) / (_e64 * _e65)));
}

fn APrxHighLinearToPQ_2(a_152: vec3<f32>) -> vec3<f32> {
    var a_153: vec3<f32>;

    a_153 = a_152;
    let _e13 = a_153;
    let _e16 = a_153;
    let _e20 = a_153;
    let _e23 = a_153;
    return sqrt(sqrt(sqrt(_e23)));
}

fn APrxPQToGamma2_3(a_154: vec4<f32>) -> vec4<f32> {
    var a_155: vec4<f32>;

    a_155 = a_154;
    let _e13 = a_155;
    let _e14 = Quart_3(_e13);
    return _e14;
}

fn APrxPQToLinear_3(a_156: vec4<f32>) -> vec4<f32> {
    var a_157: vec4<f32>;

    a_157 = a_156;
    let _e13 = a_157;
    let _e14 = Oct_3(_e13);
    return _e14;
}

fn APrxLoGamma2ToPQ_3(a_158: vec4<f32>) -> vec4<f32> {
    var a_159: vec4<f32>;

    a_159 = a_158;
    let _e12 = a_159;
    let _e14 = a_159;
    let _e21 = AU4_x(2u);
    let _e27 = AU4_x(798641734u);
    let _e30 = a_159;
    let _e32 = a_159;
    let _e39 = AU4_x(2u);
    let _e45 = AU4_x(798641734u);
    return bitcast<vec4<f32>>(vec4<u32>(((bitcast<vec4<u32>>(vec4<f32>(_e32)) >> _e39) + _e45)));
}

fn APrxMedGamma2ToPQ_3(a_160: vec4<f32>) -> vec4<f32> {
    var a_161: vec4<f32>;
    var b_34: vec4<f32>;
    var b4_3: vec4<f32>;

    a_161 = a_160;
    let _e12 = a_161;
    let _e14 = a_161;
    let _e21 = AU4_x(2u);
    let _e27 = AU4_x(798641734u);
    let _e30 = a_161;
    let _e32 = a_161;
    let _e39 = AU4_x(2u);
    let _e45 = AU4_x(798641734u);
    b_34 = bitcast<vec4<f32>>(vec4<u32>(((bitcast<vec4<u32>>(vec4<f32>(_e32)) >> _e39) + _e45)));
    let _e51 = b_34;
    let _e52 = Quart_3(_e51);
    b4_3 = _e52;
    let _e54 = b_34;
    let _e55 = b_34;
    let _e56 = b4_3;
    let _e57 = a_161;
    let _e64 = AF1_x(4f);
    let _e65 = b4_3;
    return (_e54 - ((_e55 * (_e56 - _e57)) / (_e64 * _e65)));
}

fn APrxHighGamma2ToPQ_3(a_162: vec4<f32>) -> vec4<f32> {
    var a_163: vec4<f32>;

    a_163 = a_162;
    let _e13 = a_163;
    let _e16 = a_163;
    return sqrt(sqrt(_e16));
}

fn APrxLoLinearToPQ_3(a_164: vec4<f32>) -> vec4<f32> {
    var a_165: vec4<f32>;

    a_165 = a_164;
    let _e12 = a_165;
    let _e14 = a_165;
    let _e21 = AU4_x(3u);
    let _e27 = AU4_x(932022051u);
    let _e30 = a_165;
    let _e32 = a_165;
    let _e39 = AU4_x(3u);
    let _e45 = AU4_x(932022051u);
    return bitcast<vec4<f32>>(vec4<u32>(((bitcast<vec4<u32>>(vec4<f32>(_e32)) >> _e39) + _e45)));
}

fn APrxMedLinearToPQ_3(a_166: vec4<f32>) -> vec4<f32> {
    var a_167: vec4<f32>;
    var b_35: vec4<f32>;
    var b8_3: vec4<f32>;

    a_167 = a_166;
    let _e12 = a_167;
    let _e14 = a_167;
    let _e21 = AU4_x(3u);
    let _e27 = AU4_x(932022051u);
    let _e30 = a_167;
    let _e32 = a_167;
    let _e39 = AU4_x(3u);
    let _e45 = AU4_x(932022051u);
    b_35 = bitcast<vec4<f32>>(vec4<u32>(((bitcast<vec4<u32>>(vec4<f32>(_e32)) >> _e39) + _e45)));
    let _e51 = b_35;
    let _e52 = Oct_3(_e51);
    b8_3 = _e52;
    let _e54 = b_35;
    let _e55 = b_35;
    let _e56 = b8_3;
    let _e57 = a_167;
    let _e64 = AF1_x(8f);
    let _e65 = b8_3;
    return (_e54 - ((_e55 * (_e56 - _e57)) / (_e64 * _e65)));
}

fn APrxHighLinearToPQ_3(a_168: vec4<f32>) -> vec4<f32> {
    var a_169: vec4<f32>;

    a_169 = a_168;
    let _e13 = a_169;
    let _e16 = a_169;
    let _e20 = a_169;
    let _e23 = a_169;
    return sqrt(sqrt(sqrt(_e23)));
}

fn APSinF1_(x_128: f32) -> f32 {
    var x_129: f32;

    x_129 = x_128;
    let _e12 = x_129;
    let _e14 = x_129;
    let _e17 = x_129;
    return ((_e12 * abs(_e14)) - _e17);
}

fn APSinF2_(x_130: vec2<f32>) -> vec2<f32> {
    var x_131: vec2<f32>;

    x_131 = x_130;
    let _e12 = x_131;
    let _e14 = x_131;
    let _e17 = x_131;
    return ((_e12 * abs(_e14)) - _e17);
}

fn APCosF1_(x_132: f32) -> f32 {
    var x_133: f32;

    x_133 = x_132;
    let _e12 = x_133;
    let _e17 = AF1_x(0.5f);
    let _e23 = AF1_x(0.75f);
    let _e25 = x_133;
    let _e30 = AF1_x(0.5f);
    let _e36 = AF1_x(0.75f);
    let _e38 = AFractF1_(((_e25 * _e30) + _e36));
    x_133 = _e38;
    let _e39 = x_133;
    let _e44 = AF1_x(2f);
    let _e50 = AF1_x(1f);
    x_133 = ((_e39 * _e44) - _e50);
    let _e53 = x_133;
    let _e54 = APSinF1_(_e53);
    return _e54;
}

fn APCosF2_(x_134: vec2<f32>) -> vec2<f32> {
    var x_135: vec2<f32>;

    x_135 = x_134;
    let _e12 = x_135;
    let _e17 = AF2_x(0.5f);
    let _e23 = AF2_x(0.75f);
    let _e25 = x_135;
    let _e30 = AF2_x(0.5f);
    let _e36 = AF2_x(0.75f);
    let _e38 = AFractF2_(((_e25 * _e30) + _e36));
    x_135 = _e38;
    let _e39 = x_135;
    let _e44 = AF2_x(2f);
    let _e50 = AF2_x(1f);
    x_135 = ((_e39 * _e44) - _e50);
    let _e53 = x_135;
    let _e54 = APSinF2_(_e53);
    return _e54;
}

fn APSinCosF1_(x_136: f32) -> vec2<f32> {
    var x_137: f32;
    var y_64: f32;

    x_137 = x_136;
    let _e12 = x_137;
    let _e17 = AF1_x(0.5f);
    let _e23 = AF1_x(0.75f);
    let _e25 = x_137;
    let _e30 = AF1_x(0.5f);
    let _e36 = AF1_x(0.75f);
    let _e38 = AFractF1_(((_e25 * _e30) + _e36));
    y_64 = _e38;
    let _e40 = y_64;
    let _e45 = AF1_x(2f);
    let _e51 = AF1_x(1f);
    y_64 = ((_e40 * _e45) - _e51);
    let _e53 = x_137;
    let _e54 = y_64;
    let _e56 = x_137;
    let _e57 = y_64;
    let _e59 = APSinF2_(vec2<f32>(_e56, _e57));
    return _e59;
}

fn AZolAndU1_(x_138: u32, y_65: u32) -> u32 {
    var x_139: u32;
    var y_66: u32;

    x_139 = x_138;
    y_66 = y_65;
    let _e16 = x_139;
    let _e17 = y_66;
    return min(_e16, _e17);
}

fn AZolAndU2_(x_140: vec2<u32>, y_67: vec2<u32>) -> vec2<u32> {
    var x_141: vec2<u32>;
    var y_68: vec2<u32>;

    x_141 = x_140;
    y_68 = y_67;
    let _e16 = x_141;
    let _e17 = y_68;
    return min(_e16, _e17);
}

fn AZolAndU3_(x_142: vec3<u32>, y_69: vec3<u32>) -> vec3<u32> {
    var x_143: vec3<u32>;
    var y_70: vec3<u32>;

    x_143 = x_142;
    y_70 = y_69;
    let _e16 = x_143;
    let _e17 = y_70;
    return min(_e16, _e17);
}

fn AZolAndU4_(x_144: vec4<u32>, y_71: vec4<u32>) -> vec4<u32> {
    var x_145: vec4<u32>;
    var y_72: vec4<u32>;

    x_145 = x_144;
    y_72 = y_71;
    let _e16 = x_145;
    let _e17 = y_72;
    return min(_e16, _e17);
}

fn AZolNotU1_(x_146: u32) -> u32 {
    var x_147: u32;

    x_147 = x_146;
    let _e12 = x_147;
    let _e17 = AU1_x(1u);
    return (_e12 ^ _e17);
}

fn AZolNotU2_(x_148: vec2<u32>) -> vec2<u32> {
    var x_149: vec2<u32>;

    x_149 = x_148;
    let _e12 = x_149;
    let _e17 = AU2_x(1u);
    return (_e12 ^ _e17);
}

fn AZolNotU3_(x_150: vec3<u32>) -> vec3<u32> {
    var x_151: vec3<u32>;

    x_151 = x_150;
    let _e12 = x_151;
    let _e17 = AU3_x(1u);
    return (_e12 ^ _e17);
}

fn AZolNotU4_(x_152: vec4<u32>) -> vec4<u32> {
    var x_153: vec4<u32>;

    x_153 = x_152;
    let _e12 = x_153;
    let _e17 = AU4_x(1u);
    return (_e12 ^ _e17);
}

fn AZolOrU1_(x_154: u32, y_73: u32) -> u32 {
    var x_155: u32;
    var y_74: u32;

    x_155 = x_154;
    y_74 = y_73;
    let _e16 = x_155;
    let _e17 = y_74;
    return max(_e16, _e17);
}

fn AZolOrU2_(x_156: vec2<u32>, y_75: vec2<u32>) -> vec2<u32> {
    var x_157: vec2<u32>;
    var y_76: vec2<u32>;

    x_157 = x_156;
    y_76 = y_75;
    let _e16 = x_157;
    let _e17 = y_76;
    return max(_e16, _e17);
}

fn AZolOrU3_(x_158: vec3<u32>, y_77: vec3<u32>) -> vec3<u32> {
    var x_159: vec3<u32>;
    var y_78: vec3<u32>;

    x_159 = x_158;
    y_78 = y_77;
    let _e16 = x_159;
    let _e17 = y_78;
    return max(_e16, _e17);
}

fn AZolOrU4_(x_160: vec4<u32>, y_79: vec4<u32>) -> vec4<u32> {
    var x_161: vec4<u32>;
    var y_80: vec4<u32>;

    x_161 = x_160;
    y_80 = y_79;
    let _e16 = x_161;
    let _e17 = y_80;
    return max(_e16, _e17);
}

fn AZolF1ToU1_(x_162: f32) -> u32 {
    var x_163: f32;

    x_163 = x_162;
    let _e12 = x_163;
    return u32(_e12);
}

fn AZolF2ToU2_(x_164: vec2<f32>) -> vec2<u32> {
    var x_165: vec2<f32>;

    x_165 = x_164;
    let _e12 = x_165;
    return vec2<u32>(_e12);
}

fn AZolF3ToU3_(x_166: vec3<f32>) -> vec3<u32> {
    var x_167: vec3<f32>;

    x_167 = x_166;
    let _e12 = x_167;
    return vec3<u32>(_e12);
}

fn AZolF4ToU4_(x_168: vec4<f32>) -> vec4<u32> {
    var x_169: vec4<f32>;

    x_169 = x_168;
    let _e12 = x_169;
    return vec4<u32>(_e12);
}

fn AZolNotF1ToU1_(x_170: f32) -> u32 {
    var x_171: f32;

    x_171 = x_170;
    let _e16 = AF1_x(1f);
    let _e17 = x_171;
    return u32((_e16 - _e17));
}

fn AZolNotF2ToU2_(x_172: vec2<f32>) -> vec2<u32> {
    var x_173: vec2<f32>;

    x_173 = x_172;
    let _e16 = AF2_x(1f);
    let _e17 = x_173;
    return vec2<u32>((_e16 - _e17));
}

fn AZolNotF3ToU3_(x_174: vec3<f32>) -> vec3<u32> {
    var x_175: vec3<f32>;

    x_175 = x_174;
    let _e16 = AF3_x(1f);
    let _e17 = x_175;
    return vec3<u32>((_e16 - _e17));
}

fn AZolNotF4ToU4_(x_176: vec4<f32>) -> vec4<u32> {
    var x_177: vec4<f32>;

    x_177 = x_176;
    let _e16 = AF4_x(1f);
    let _e17 = x_177;
    return vec4<u32>((_e16 - _e17));
}

fn AZolU1ToF1_(x_178: u32) -> f32 {
    var x_179: u32;

    x_179 = x_178;
    let _e12 = x_179;
    return f32(_e12);
}

fn AZolU2ToF2_(x_180: vec2<u32>) -> vec2<f32> {
    var x_181: vec2<u32>;

    x_181 = x_180;
    let _e12 = x_181;
    return vec2<f32>(_e12);
}

fn AZolU3ToF3_(x_182: vec3<u32>) -> vec3<f32> {
    var x_183: vec3<u32>;

    x_183 = x_182;
    let _e12 = x_183;
    return vec3<f32>(_e12);
}

fn AZolU4ToF4_(x_184: vec4<u32>) -> vec4<f32> {
    var x_185: vec4<u32>;

    x_185 = x_184;
    let _e12 = x_185;
    return vec4<f32>(_e12);
}

fn AZolAndF1_(x_186: f32, y_81: f32) -> f32 {
    var x_187: f32;
    var y_82: f32;

    x_187 = x_186;
    y_82 = y_81;
    let _e16 = x_187;
    let _e17 = y_82;
    return min(_e16, _e17);
}

fn AZolAndF2_(x_188: vec2<f32>, y_83: vec2<f32>) -> vec2<f32> {
    var x_189: vec2<f32>;
    var y_84: vec2<f32>;

    x_189 = x_188;
    y_84 = y_83;
    let _e16 = x_189;
    let _e17 = y_84;
    return min(_e16, _e17);
}

fn AZolAndF3_(x_190: vec3<f32>, y_85: vec3<f32>) -> vec3<f32> {
    var x_191: vec3<f32>;
    var y_86: vec3<f32>;

    x_191 = x_190;
    y_86 = y_85;
    let _e16 = x_191;
    let _e17 = y_86;
    return min(_e16, _e17);
}

fn AZolAndF4_(x_192: vec4<f32>, y_87: vec4<f32>) -> vec4<f32> {
    var x_193: vec4<f32>;
    var y_88: vec4<f32>;

    x_193 = x_192;
    y_88 = y_87;
    let _e16 = x_193;
    let _e17 = y_88;
    return min(_e16, _e17);
}

fn ASolAndNotF1_(x_194: f32, y_89: f32) -> f32 {
    var x_195: f32;
    var y_90: f32;

    x_195 = x_194;
    y_90 = y_89;
    let _e14 = x_195;
    let _e16 = y_90;
    let _e22 = AF1_x(1f);
    return ((-(_e14) * _e16) + _e22);
}

fn ASolAndNotF2_(x_196: vec2<f32>, y_91: vec2<f32>) -> vec2<f32> {
    var x_197: vec2<f32>;
    var y_92: vec2<f32>;

    x_197 = x_196;
    y_92 = y_91;
    let _e14 = x_197;
    let _e16 = y_92;
    let _e22 = AF2_x(1f);
    return ((-(_e14) * _e16) + _e22);
}

fn ASolAndNotF3_(x_198: vec3<f32>, y_93: vec3<f32>) -> vec3<f32> {
    var x_199: vec3<f32>;
    var y_94: vec3<f32>;

    x_199 = x_198;
    y_94 = y_93;
    let _e14 = x_199;
    let _e16 = y_94;
    let _e22 = AF3_x(1f);
    return ((-(_e14) * _e16) + _e22);
}

fn ASolAndNotF4_(x_200: vec4<f32>, y_95: vec4<f32>) -> vec4<f32> {
    var x_201: vec4<f32>;
    var y_96: vec4<f32>;

    x_201 = x_200;
    y_96 = y_95;
    let _e14 = x_201;
    let _e16 = y_96;
    let _e22 = AF4_x(1f);
    return ((-(_e14) * _e16) + _e22);
}

fn AZolAndOrF1_(x_202: f32, y_97: f32, z_56: f32) -> f32 {
    var x_203: f32;
    var y_98: f32;
    var z_57: f32;

    x_203 = x_202;
    y_98 = y_97;
    z_57 = z_56;
    let _e16 = x_203;
    let _e17 = y_98;
    let _e19 = z_57;
    let _e21 = x_203;
    let _e22 = y_98;
    let _e24 = z_57;
    let _e26 = ASatF1_(((_e21 * _e22) + _e24));
    return _e26;
}

fn AZolAndOrF2_(x_204: vec2<f32>, y_99: vec2<f32>, z_58: vec2<f32>) -> vec2<f32> {
    var x_205: vec2<f32>;
    var y_100: vec2<f32>;
    var z_59: vec2<f32>;

    x_205 = x_204;
    y_100 = y_99;
    z_59 = z_58;
    let _e16 = x_205;
    let _e17 = y_100;
    let _e19 = z_59;
    let _e21 = x_205;
    let _e22 = y_100;
    let _e24 = z_59;
    let _e26 = ASatF2_(((_e21 * _e22) + _e24));
    return _e26;
}

fn AZolAndOrF3_(x_206: vec3<f32>, y_101: vec3<f32>, z_60: vec3<f32>) -> vec3<f32> {
    var x_207: vec3<f32>;
    var y_102: vec3<f32>;
    var z_61: vec3<f32>;

    x_207 = x_206;
    y_102 = y_101;
    z_61 = z_60;
    let _e16 = x_207;
    let _e17 = y_102;
    let _e19 = z_61;
    let _e21 = x_207;
    let _e22 = y_102;
    let _e24 = z_61;
    let _e26 = ASatF3_(((_e21 * _e22) + _e24));
    return _e26;
}

fn AZolAndOrF4_(x_208: vec4<f32>, y_103: vec4<f32>, z_62: vec4<f32>) -> vec4<f32> {
    var x_209: vec4<f32>;
    var y_104: vec4<f32>;
    var z_63: vec4<f32>;

    x_209 = x_208;
    y_104 = y_103;
    z_63 = z_62;
    let _e16 = x_209;
    let _e17 = y_104;
    let _e19 = z_63;
    let _e21 = x_209;
    let _e22 = y_104;
    let _e24 = z_63;
    let _e26 = ASatF4_(((_e21 * _e22) + _e24));
    return _e26;
}

fn AZolGtZeroF1_(x_210: f32) -> f32 {
    var x_211: f32;

    x_211 = x_210;
    let _e12 = x_211;
    let _e25 = AF1_x(f32(bitcast<f32>(2139095040u)));
    let _e27 = x_211;
    let _e40 = AF1_x(f32(bitcast<f32>(2139095040u)));
    let _e42 = ASatF1_((_e27 * _e40));
    return _e42;
}

fn AZolGtZeroF2_(x_212: vec2<f32>) -> vec2<f32> {
    var x_213: vec2<f32>;

    x_213 = x_212;
    let _e12 = x_213;
    let _e25 = AF2_x(f32(bitcast<f32>(2139095040u)));
    let _e27 = x_213;
    let _e40 = AF2_x(f32(bitcast<f32>(2139095040u)));
    let _e42 = ASatF2_((_e27 * _e40));
    return _e42;
}

fn AZolGtZeroF3_(x_214: vec3<f32>) -> vec3<f32> {
    var x_215: vec3<f32>;

    x_215 = x_214;
    let _e12 = x_215;
    let _e25 = AF3_x(f32(bitcast<f32>(2139095040u)));
    let _e27 = x_215;
    let _e40 = AF3_x(f32(bitcast<f32>(2139095040u)));
    let _e42 = ASatF3_((_e27 * _e40));
    return _e42;
}

fn AZolGtZeroF4_(x_216: vec4<f32>) -> vec4<f32> {
    var x_217: vec4<f32>;

    x_217 = x_216;
    let _e12 = x_217;
    let _e25 = AF4_x(f32(bitcast<f32>(2139095040u)));
    let _e27 = x_217;
    let _e40 = AF4_x(f32(bitcast<f32>(2139095040u)));
    let _e42 = ASatF4_((_e27 * _e40));
    return _e42;
}

fn AZolNotF1_(x_218: f32) -> f32 {
    var x_219: f32;

    x_219 = x_218;
    let _e16 = AF1_x(1f);
    let _e17 = x_219;
    return (_e16 - _e17);
}

fn AZolNotF2_(x_220: vec2<f32>) -> vec2<f32> {
    var x_221: vec2<f32>;

    x_221 = x_220;
    let _e16 = AF2_x(1f);
    let _e17 = x_221;
    return (_e16 - _e17);
}

fn AZolNotF3_(x_222: vec3<f32>) -> vec3<f32> {
    var x_223: vec3<f32>;

    x_223 = x_222;
    let _e16 = AF3_x(1f);
    let _e17 = x_223;
    return (_e16 - _e17);
}

fn AZolNotF4_(x_224: vec4<f32>) -> vec4<f32> {
    var x_225: vec4<f32>;

    x_225 = x_224;
    let _e16 = AF4_x(1f);
    let _e17 = x_225;
    return (_e16 - _e17);
}

fn AZolOrF1_(x_226: f32, y_105: f32) -> f32 {
    var x_227: f32;
    var y_106: f32;

    x_227 = x_226;
    y_106 = y_105;
    let _e16 = x_227;
    let _e17 = y_106;
    return max(_e16, _e17);
}

fn AZolOrF2_(x_228: vec2<f32>, y_107: vec2<f32>) -> vec2<f32> {
    var x_229: vec2<f32>;
    var y_108: vec2<f32>;

    x_229 = x_228;
    y_108 = y_107;
    let _e16 = x_229;
    let _e17 = y_108;
    return max(_e16, _e17);
}

fn AZolOrF3_(x_230: vec3<f32>, y_109: vec3<f32>) -> vec3<f32> {
    var x_231: vec3<f32>;
    var y_110: vec3<f32>;

    x_231 = x_230;
    y_110 = y_109;
    let _e16 = x_231;
    let _e17 = y_110;
    return max(_e16, _e17);
}

fn AZolOrF4_(x_232: vec4<f32>, y_111: vec4<f32>) -> vec4<f32> {
    var x_233: vec4<f32>;
    var y_112: vec4<f32>;

    x_233 = x_232;
    y_112 = y_111;
    let _e16 = x_233;
    let _e17 = y_112;
    return max(_e16, _e17);
}

fn AZolSelF1_(x_234: f32, y_113: f32, z_64: f32) -> f32 {
    var x_235: f32;
    var y_114: f32;
    var z_65: f32;
    var r: f32;

    x_235 = x_234;
    y_114 = y_113;
    z_65 = z_64;
    let _e16 = x_235;
    let _e18 = z_65;
    let _e20 = z_65;
    r = ((-(_e16) * _e18) + _e20);
    let _e23 = x_235;
    let _e24 = y_114;
    let _e26 = r;
    return ((_e23 * _e24) + _e26);
}

fn AZolSelF2_(x_236: vec2<f32>, y_115: vec2<f32>, z_66: vec2<f32>) -> vec2<f32> {
    var x_237: vec2<f32>;
    var y_116: vec2<f32>;
    var z_67: vec2<f32>;
    var r_1: vec2<f32>;

    x_237 = x_236;
    y_116 = y_115;
    z_67 = z_66;
    let _e16 = x_237;
    let _e18 = z_67;
    let _e20 = z_67;
    r_1 = ((-(_e16) * _e18) + _e20);
    let _e23 = x_237;
    let _e24 = y_116;
    let _e26 = r_1;
    return ((_e23 * _e24) + _e26);
}

fn AZolSelF3_(x_238: vec3<f32>, y_117: vec3<f32>, z_68: vec3<f32>) -> vec3<f32> {
    var x_239: vec3<f32>;
    var y_118: vec3<f32>;
    var z_69: vec3<f32>;
    var r_2: vec3<f32>;

    x_239 = x_238;
    y_118 = y_117;
    z_69 = z_68;
    let _e16 = x_239;
    let _e18 = z_69;
    let _e20 = z_69;
    r_2 = ((-(_e16) * _e18) + _e20);
    let _e23 = x_239;
    let _e24 = y_118;
    let _e26 = r_2;
    return ((_e23 * _e24) + _e26);
}

fn AZolSelF4_(x_240: vec4<f32>, y_119: vec4<f32>, z_70: vec4<f32>) -> vec4<f32> {
    var x_241: vec4<f32>;
    var y_120: vec4<f32>;
    var z_71: vec4<f32>;
    var r_3: vec4<f32>;

    x_241 = x_240;
    y_120 = y_119;
    z_71 = z_70;
    let _e16 = x_241;
    let _e18 = z_71;
    let _e20 = z_71;
    r_3 = ((-(_e16) * _e18) + _e20);
    let _e23 = x_241;
    let _e24 = y_120;
    let _e26 = r_3;
    return ((_e23 * _e24) + _e26);
}

fn AZolSignedF1_(x_242: f32) -> f32 {
    var x_243: f32;

    x_243 = x_242;
    let _e12 = x_243;
    let _e25 = AF1_x(f32(bitcast<f32>(4286578688u)));
    let _e27 = x_243;
    let _e40 = AF1_x(f32(bitcast<f32>(4286578688u)));
    let _e42 = ASatF1_((_e27 * _e40));
    return _e42;
}

fn AZolSignedF2_(x_244: vec2<f32>) -> vec2<f32> {
    var x_245: vec2<f32>;

    x_245 = x_244;
    let _e12 = x_245;
    let _e25 = AF2_x(f32(bitcast<f32>(4286578688u)));
    let _e27 = x_245;
    let _e40 = AF2_x(f32(bitcast<f32>(4286578688u)));
    let _e42 = ASatF2_((_e27 * _e40));
    return _e42;
}

fn AZolSignedF3_(x_246: vec3<f32>) -> vec3<f32> {
    var x_247: vec3<f32>;

    x_247 = x_246;
    let _e12 = x_247;
    let _e25 = AF3_x(f32(bitcast<f32>(4286578688u)));
    let _e27 = x_247;
    let _e40 = AF3_x(f32(bitcast<f32>(4286578688u)));
    let _e42 = ASatF3_((_e27 * _e40));
    return _e42;
}

fn AZolSignedF4_(x_248: vec4<f32>) -> vec4<f32> {
    var x_249: vec4<f32>;

    x_249 = x_248;
    let _e12 = x_249;
    let _e25 = AF4_x(f32(bitcast<f32>(4286578688u)));
    let _e27 = x_249;
    let _e40 = AF4_x(f32(bitcast<f32>(4286578688u)));
    let _e42 = ASatF4_((_e27 * _e40));
    return _e42;
}

fn AZolZeroPassF1_(x_250: f32, y_121: f32) -> f32 {
    var x_251: f32;
    var y_122: f32;
    var local: u32;
    var local_1: u32;

    x_251 = x_250;
    y_122 = y_121;
    let _e14 = x_251;
    let _e16 = x_251;
    let _e23 = AU1_x(0u);
    if bitcast<u32>(f32(_e16)) != _e23 {
        let _e29 = AU1_x(0u);
        local = _e29;
    } else {
        let _e30 = y_122;
        let _e32 = y_122;
        local = bitcast<u32>(f32(_e32));
    }
    let _e36 = local;
    let _e38 = x_251;
    let _e40 = x_251;
    let _e47 = AU1_x(0u);
    if bitcast<u32>(f32(_e40)) != _e47 {
        let _e53 = AU1_x(0u);
        local_1 = _e53;
    } else {
        let _e54 = y_122;
        let _e56 = y_122;
        local_1 = bitcast<u32>(f32(_e56));
    }
    let _e60 = local_1;
    return bitcast<f32>(u32(_e60));
}

fn AZolZeroPassF2_(x_252: vec2<f32>, y_123: vec2<f32>) -> vec2<f32> {
    var x_253: vec2<f32>;
    var y_124: vec2<f32>;
    var local_2: vec2<u32>;
    var local_3: vec2<u32>;

    x_253 = x_252;
    y_124 = y_123;
    let _e14 = x_253;
    let _e16 = x_253;
    let _e23 = AU2_x(0u);
    if any((bitcast<vec2<u32>>(vec2<f32>(_e16)) != _e23)) {
        let _e30 = AU2_x(0u);
        local_2 = _e30;
    } else {
        let _e31 = y_124;
        let _e33 = y_124;
        local_2 = bitcast<vec2<u32>>(vec2<f32>(_e33));
    }
    let _e37 = local_2;
    let _e39 = x_253;
    let _e41 = x_253;
    let _e48 = AU2_x(0u);
    if any((bitcast<vec2<u32>>(vec2<f32>(_e41)) != _e48)) {
        let _e55 = AU2_x(0u);
        local_3 = _e55;
    } else {
        let _e56 = y_124;
        let _e58 = y_124;
        local_3 = bitcast<vec2<u32>>(vec2<f32>(_e58));
    }
    let _e62 = local_3;
    return bitcast<vec2<f32>>(vec2<u32>(_e62));
}

fn AZolZeroPassF3_(x_254: vec3<f32>, y_125: vec3<f32>) -> vec3<f32> {
    var x_255: vec3<f32>;
    var y_126: vec3<f32>;
    var local_4: vec3<u32>;
    var local_5: vec3<u32>;

    x_255 = x_254;
    y_126 = y_125;
    let _e14 = x_255;
    let _e16 = x_255;
    let _e23 = AU3_x(0u);
    if any((bitcast<vec3<u32>>(vec3<f32>(_e16)) != _e23)) {
        let _e30 = AU3_x(0u);
        local_4 = _e30;
    } else {
        let _e31 = y_126;
        let _e33 = y_126;
        local_4 = bitcast<vec3<u32>>(vec3<f32>(_e33));
    }
    let _e37 = local_4;
    let _e39 = x_255;
    let _e41 = x_255;
    let _e48 = AU3_x(0u);
    if any((bitcast<vec3<u32>>(vec3<f32>(_e41)) != _e48)) {
        let _e55 = AU3_x(0u);
        local_5 = _e55;
    } else {
        let _e56 = y_126;
        let _e58 = y_126;
        local_5 = bitcast<vec3<u32>>(vec3<f32>(_e58));
    }
    let _e62 = local_5;
    return bitcast<vec3<f32>>(vec3<u32>(_e62));
}

fn AZolZeroPassF4_(x_256: vec4<f32>, y_127: vec4<f32>) -> vec4<f32> {
    var x_257: vec4<f32>;
    var y_128: vec4<f32>;
    var local_6: vec4<u32>;
    var local_7: vec4<u32>;

    x_257 = x_256;
    y_128 = y_127;
    let _e14 = x_257;
    let _e16 = x_257;
    let _e23 = AU4_x(0u);
    if any((bitcast<vec4<u32>>(vec4<f32>(_e16)) != _e23)) {
        let _e30 = AU4_x(0u);
        local_6 = _e30;
    } else {
        let _e31 = y_128;
        let _e33 = y_128;
        local_6 = bitcast<vec4<u32>>(vec4<f32>(_e33));
    }
    let _e37 = local_6;
    let _e39 = x_257;
    let _e41 = x_257;
    let _e48 = AU4_x(0u);
    if any((bitcast<vec4<u32>>(vec4<f32>(_e41)) != _e48)) {
        let _e55 = AU4_x(0u);
        local_7 = _e55;
    } else {
        let _e56 = y_128;
        let _e58 = y_128;
        local_7 = bitcast<vec4<u32>>(vec4<f32>(_e58));
    }
    let _e62 = local_7;
    return bitcast<vec4<f32>>(vec4<u32>(_e62));
}

fn ATo709F1_(c: f32) -> f32 {
    var c_1: f32;
    var j: vec3<f32> = vec3<f32>(0.081f, 4.5f, 0.45f);
    var k: vec2<f32> = vec2<f32>(1.099f, -0.099f);

    c_1 = c;
    let _e24 = j;
    let _e26 = c_1;
    let _e27 = j;
    let _e31 = j;
    let _e33 = c_1;
    let _e34 = j;
    let _e37 = k;
    let _e40 = k;
    let _e43 = j;
    let _e45 = c_1;
    let _e46 = j;
    let _e50 = j;
    let _e52 = c_1;
    let _e53 = j;
    let _e56 = k;
    let _e59 = k;
    return clamp(_e43.x, (_e45 * _e46.y), ((pow(_e52, _e53.z) * _e56.x) + _e59.y));
}

fn ATo709F2_(c_2: vec2<f32>) -> vec2<f32> {
    var c_3: vec2<f32>;
    var j_1: vec3<f32> = vec3<f32>(0.081f, 4.5f, 0.45f);
    var k_1: vec2<f32> = vec2<f32>(1.099f, -0.099f);

    c_3 = c_2;
    let _e24 = j_1;
    let _e26 = c_3;
    let _e27 = j_1;
    let _e31 = j_1;
    let _e33 = c_3;
    let _e34 = j_1;
    let _e37 = k_1;
    let _e40 = k_1;
    let _e43 = j_1;
    let _e45 = c_3;
    let _e46 = j_1;
    let _e50 = j_1;
    let _e52 = c_3;
    let _e53 = j_1;
    let _e56 = k_1;
    let _e59 = k_1;
    return clamp(_e43.xx, (_e45 * _e46.yy), ((pow(_e52, _e53.zz) * _e56.xx) + _e59.yy));
}

fn ATo709F3_(c_4: vec3<f32>) -> vec3<f32> {
    var c_5: vec3<f32>;
    var j_2: vec3<f32> = vec3<f32>(0.081f, 4.5f, 0.45f);
    var k_2: vec2<f32> = vec2<f32>(1.099f, -0.099f);

    c_5 = c_4;
    let _e24 = j_2;
    let _e26 = c_5;
    let _e27 = j_2;
    let _e31 = j_2;
    let _e33 = c_5;
    let _e34 = j_2;
    let _e37 = k_2;
    let _e40 = k_2;
    let _e43 = j_2;
    let _e45 = c_5;
    let _e46 = j_2;
    let _e50 = j_2;
    let _e52 = c_5;
    let _e53 = j_2;
    let _e56 = k_2;
    let _e59 = k_2;
    return clamp(_e43.xxx, (_e45 * _e46.yyy), ((pow(_e52, _e53.zzz) * _e56.xxx) + _e59.yyy));
}

fn AToGammaF1_(c_6: f32, rcpX: f32) -> f32 {
    var c_7: f32;
    var rcpX_1: f32;

    c_7 = c_6;
    rcpX_1 = rcpX;
    let _e15 = rcpX_1;
    let _e17 = rcpX_1;
    let _e19 = AF1_x(f32(_e17));
    let _e20 = c_7;
    let _e21 = rcpX_1;
    let _e23 = rcpX_1;
    let _e25 = AF1_x(f32(_e23));
    return pow(_e20, _e25);
}

fn AToGammaF2_(c_8: vec2<f32>, rcpX_2: f32) -> vec2<f32> {
    var c_9: vec2<f32>;
    var rcpX_3: f32;

    c_9 = c_8;
    rcpX_3 = rcpX_2;
    let _e15 = rcpX_3;
    let _e17 = rcpX_3;
    let _e19 = AF2_x(f32(_e17));
    let _e20 = c_9;
    let _e21 = rcpX_3;
    let _e23 = rcpX_3;
    let _e25 = AF2_x(f32(_e23));
    return pow(_e20, _e25);
}

fn AToGammaF3_(c_10: vec3<f32>, rcpX_4: f32) -> vec3<f32> {
    var c_11: vec3<f32>;
    var rcpX_5: f32;

    c_11 = c_10;
    rcpX_5 = rcpX_4;
    let _e15 = rcpX_5;
    let _e17 = rcpX_5;
    let _e19 = AF3_x(f32(_e17));
    let _e20 = c_11;
    let _e21 = rcpX_5;
    let _e23 = rcpX_5;
    let _e25 = AF3_x(f32(_e23));
    return pow(_e20, _e25);
}

fn AToPqF1_(x_258: f32) -> f32 {
    var x_259: f32;
    var p: f32;

    x_259 = x_258;
    let _e17 = AF1_x(0.159302f);
    let _e18 = x_259;
    let _e23 = AF1_x(0.159302f);
    p = pow(_e18, _e23);
    let _e30 = AF1_x(0.835938f);
    let _e35 = AF1_x(18.8516f);
    let _e36 = p;
    let _e43 = AF1_x(1f);
    let _e48 = AF1_x(18.6875f);
    let _e49 = p;
    let _e57 = AF1_x(78.8438f);
    let _e62 = AF1_x(0.835938f);
    let _e67 = AF1_x(18.8516f);
    let _e68 = p;
    let _e75 = AF1_x(1f);
    let _e80 = AF1_x(18.6875f);
    let _e81 = p;
    let _e89 = AF1_x(78.8438f);
    return pow(((_e62 + (_e67 * _e68)) / (_e75 + (_e80 * _e81))), _e89);
}

fn AToPqF1_1(x_260: vec2<f32>) -> vec2<f32> {
    var x_261: vec2<f32>;
    var p_1: vec2<f32>;

    x_261 = x_260;
    let _e17 = AF2_x(0.159302f);
    let _e18 = x_261;
    let _e23 = AF2_x(0.159302f);
    p_1 = pow(_e18, _e23);
    let _e30 = AF2_x(0.835938f);
    let _e35 = AF2_x(18.8516f);
    let _e36 = p_1;
    let _e43 = AF2_x(1f);
    let _e48 = AF2_x(18.6875f);
    let _e49 = p_1;
    let _e57 = AF2_x(78.8438f);
    let _e62 = AF2_x(0.835938f);
    let _e67 = AF2_x(18.8516f);
    let _e68 = p_1;
    let _e75 = AF2_x(1f);
    let _e80 = AF2_x(18.6875f);
    let _e81 = p_1;
    let _e89 = AF2_x(78.8438f);
    return pow(((_e62 + (_e67 * _e68)) / (_e75 + (_e80 * _e81))), _e89);
}

fn AToPqF1_2(x_262: vec3<f32>) -> vec3<f32> {
    var x_263: vec3<f32>;
    var p_2: vec3<f32>;

    x_263 = x_262;
    let _e17 = AF3_x(0.159302f);
    let _e18 = x_263;
    let _e23 = AF3_x(0.159302f);
    p_2 = pow(_e18, _e23);
    let _e30 = AF3_x(0.835938f);
    let _e35 = AF3_x(18.8516f);
    let _e36 = p_2;
    let _e43 = AF3_x(1f);
    let _e48 = AF3_x(18.6875f);
    let _e49 = p_2;
    let _e57 = AF3_x(78.8438f);
    let _e62 = AF3_x(0.835938f);
    let _e67 = AF3_x(18.8516f);
    let _e68 = p_2;
    let _e75 = AF3_x(1f);
    let _e80 = AF3_x(18.6875f);
    let _e81 = p_2;
    let _e89 = AF3_x(78.8438f);
    return pow(((_e62 + (_e67 * _e68)) / (_e75 + (_e80 * _e81))), _e89);
}

fn AToSrgbF1_(c_12: f32) -> f32 {
    var c_13: f32;
    var j_3: vec3<f32> = vec3<f32>(0.040449936f, 12.92f, 0.41666666f);
    var k_3: vec2<f32> = vec2<f32>(1.055f, -0.055f);

    c_13 = c_12;
    let _e26 = j_3;
    let _e28 = c_13;
    let _e29 = j_3;
    let _e33 = j_3;
    let _e35 = c_13;
    let _e36 = j_3;
    let _e39 = k_3;
    let _e42 = k_3;
    let _e45 = j_3;
    let _e47 = c_13;
    let _e48 = j_3;
    let _e52 = j_3;
    let _e54 = c_13;
    let _e55 = j_3;
    let _e58 = k_3;
    let _e61 = k_3;
    return clamp(_e45.x, (_e47 * _e48.y), ((pow(_e54, _e55.z) * _e58.x) + _e61.y));
}

fn AToSrgbF2_(c_14: vec2<f32>) -> vec2<f32> {
    var c_15: vec2<f32>;
    var j_4: vec3<f32> = vec3<f32>(0.040449936f, 12.92f, 0.41666666f);
    var k_4: vec2<f32> = vec2<f32>(1.055f, -0.055f);

    c_15 = c_14;
    let _e26 = j_4;
    let _e28 = c_15;
    let _e29 = j_4;
    let _e33 = j_4;
    let _e35 = c_15;
    let _e36 = j_4;
    let _e39 = k_4;
    let _e42 = k_4;
    let _e45 = j_4;
    let _e47 = c_15;
    let _e48 = j_4;
    let _e52 = j_4;
    let _e54 = c_15;
    let _e55 = j_4;
    let _e58 = k_4;
    let _e61 = k_4;
    return clamp(_e45.xx, (_e47 * _e48.yy), ((pow(_e54, _e55.zz) * _e58.xx) + _e61.yy));
}

fn AToSrgbF3_(c_16: vec3<f32>) -> vec3<f32> {
    var c_17: vec3<f32>;
    var j_5: vec3<f32> = vec3<f32>(0.040449936f, 12.92f, 0.41666666f);
    var k_5: vec2<f32> = vec2<f32>(1.055f, -0.055f);

    c_17 = c_16;
    let _e26 = j_5;
    let _e28 = c_17;
    let _e29 = j_5;
    let _e33 = j_5;
    let _e35 = c_17;
    let _e36 = j_5;
    let _e39 = k_5;
    let _e42 = k_5;
    let _e45 = j_5;
    let _e47 = c_17;
    let _e48 = j_5;
    let _e52 = j_5;
    let _e54 = c_17;
    let _e55 = j_5;
    let _e58 = k_5;
    let _e61 = k_5;
    return clamp(_e45.xxx, (_e47 * _e48.yyy), ((pow(_e54, _e55.zzz) * _e58.xxx) + _e61.yyy));
}

fn AToTwoF1_(c_18: f32) -> f32 {
    var c_19: f32;

    c_19 = c_18;
    let _e13 = c_19;
    return sqrt(_e13);
}

fn AToTwoF2_(c_20: vec2<f32>) -> vec2<f32> {
    var c_21: vec2<f32>;

    c_21 = c_20;
    let _e13 = c_21;
    return sqrt(_e13);
}

fn AToTwoF3_(c_22: vec3<f32>) -> vec3<f32> {
    var c_23: vec3<f32>;

    c_23 = c_22;
    let _e13 = c_23;
    return sqrt(_e13);
}

fn AToThreeF1_(c_24: f32) -> f32 {
    var c_25: f32;

    c_25 = c_24;
    let _e21 = AF1_x(0.33333334f);
    let _e22 = c_25;
    let _e31 = AF1_x(0.33333334f);
    return pow(_e22, _e31);
}

fn AToThreeF2_(c_26: vec2<f32>) -> vec2<f32> {
    var c_27: vec2<f32>;

    c_27 = c_26;
    let _e21 = AF2_x(0.33333334f);
    let _e22 = c_27;
    let _e31 = AF2_x(0.33333334f);
    return pow(_e22, _e31);
}

fn AToThreeF3_(c_28: vec3<f32>) -> vec3<f32> {
    var c_29: vec3<f32>;

    c_29 = c_28;
    let _e21 = AF3_x(0.33333334f);
    let _e22 = c_29;
    let _e31 = AF3_x(0.33333334f);
    return pow(_e22, _e31);
}

fn AFrom709F1_(c_30: f32) -> f32 {
    var c_31: f32;
    var j_6: vec3<f32> = vec3<f32>(0.018f, 0.22222222f, 2.2222223f);
    var k_6: vec2<f32> = vec2<f32>(0.9099181f, 0.09008189f);

    c_31 = c_30;
    let _e31 = c_31;
    let _e32 = j_6;
    let _e35 = c_31;
    let _e36 = j_6;
    let _e39 = AZolSignedF1_((_e35 - _e36.x));
    let _e40 = c_31;
    let _e41 = j_6;
    let _e44 = c_31;
    let _e45 = k_6;
    let _e48 = k_6;
    let _e51 = j_6;
    let _e53 = c_31;
    let _e54 = k_6;
    let _e57 = k_6;
    let _e60 = j_6;
    let _e63 = c_31;
    let _e64 = j_6;
    let _e67 = c_31;
    let _e68 = j_6;
    let _e71 = AZolSignedF1_((_e67 - _e68.x));
    let _e72 = c_31;
    let _e73 = j_6;
    let _e76 = c_31;
    let _e77 = k_6;
    let _e80 = k_6;
    let _e83 = j_6;
    let _e85 = c_31;
    let _e86 = k_6;
    let _e89 = k_6;
    let _e92 = j_6;
    let _e95 = AZolSelF1_(_e71, (_e72 * _e73.y), pow(((_e85 * _e86.x) + _e89.y), _e92.z));
    return _e95;
}

fn AFrom709F2_(c_32: vec2<f32>) -> vec2<f32> {
    var c_33: vec2<f32>;
    var j_7: vec3<f32> = vec3<f32>(0.018f, 0.22222222f, 2.2222223f);
    var k_7: vec2<f32> = vec2<f32>(0.9099181f, 0.09008189f);

    c_33 = c_32;
    let _e31 = c_33;
    let _e32 = j_7;
    let _e35 = c_33;
    let _e36 = j_7;
    let _e39 = AZolSignedF2_((_e35 - _e36.xx));
    let _e40 = c_33;
    let _e41 = j_7;
    let _e44 = c_33;
    let _e45 = k_7;
    let _e48 = k_7;
    let _e51 = j_7;
    let _e53 = c_33;
    let _e54 = k_7;
    let _e57 = k_7;
    let _e60 = j_7;
    let _e63 = c_33;
    let _e64 = j_7;
    let _e67 = c_33;
    let _e68 = j_7;
    let _e71 = AZolSignedF2_((_e67 - _e68.xx));
    let _e72 = c_33;
    let _e73 = j_7;
    let _e76 = c_33;
    let _e77 = k_7;
    let _e80 = k_7;
    let _e83 = j_7;
    let _e85 = c_33;
    let _e86 = k_7;
    let _e89 = k_7;
    let _e92 = j_7;
    let _e95 = AZolSelF2_(_e71, (_e72 * _e73.yy), pow(((_e85 * _e86.xx) + _e89.yy), _e92.zz));
    return _e95;
}

fn AFrom709F3_(c_34: vec3<f32>) -> vec3<f32> {
    var c_35: vec3<f32>;
    var j_8: vec3<f32> = vec3<f32>(0.018f, 0.22222222f, 2.2222223f);
    var k_8: vec2<f32> = vec2<f32>(0.9099181f, 0.09008189f);

    c_35 = c_34;
    let _e31 = c_35;
    let _e32 = j_8;
    let _e35 = c_35;
    let _e36 = j_8;
    let _e39 = AZolSignedF3_((_e35 - _e36.xxx));
    let _e40 = c_35;
    let _e41 = j_8;
    let _e44 = c_35;
    let _e45 = k_8;
    let _e48 = k_8;
    let _e51 = j_8;
    let _e53 = c_35;
    let _e54 = k_8;
    let _e57 = k_8;
    let _e60 = j_8;
    let _e63 = c_35;
    let _e64 = j_8;
    let _e67 = c_35;
    let _e68 = j_8;
    let _e71 = AZolSignedF3_((_e67 - _e68.xxx));
    let _e72 = c_35;
    let _e73 = j_8;
    let _e76 = c_35;
    let _e77 = k_8;
    let _e80 = k_8;
    let _e83 = j_8;
    let _e85 = c_35;
    let _e86 = k_8;
    let _e89 = k_8;
    let _e92 = j_8;
    let _e95 = AZolSelF3_(_e71, (_e72 * _e73.yyy), pow(((_e85 * _e86.xxx) + _e89.yyy), _e92.zzz));
    return _e95;
}

fn AFromGammaF1_(c_36: f32, x_264: f32) -> f32 {
    var c_37: f32;
    var x_265: f32;

    c_37 = c_36;
    x_265 = x_264;
    let _e15 = x_265;
    let _e17 = x_265;
    let _e19 = AF1_x(f32(_e17));
    let _e20 = c_37;
    let _e21 = x_265;
    let _e23 = x_265;
    let _e25 = AF1_x(f32(_e23));
    return pow(_e20, _e25);
}

fn AFromGammaF2_(c_38: vec2<f32>, x_266: f32) -> vec2<f32> {
    var c_39: vec2<f32>;
    var x_267: f32;

    c_39 = c_38;
    x_267 = x_266;
    let _e15 = x_267;
    let _e17 = x_267;
    let _e19 = AF2_x(f32(_e17));
    let _e20 = c_39;
    let _e21 = x_267;
    let _e23 = x_267;
    let _e25 = AF2_x(f32(_e23));
    return pow(_e20, _e25);
}

fn AFromGammaF3_(c_40: vec3<f32>, x_268: f32) -> vec3<f32> {
    var c_41: vec3<f32>;
    var x_269: f32;

    c_41 = c_40;
    x_269 = x_268;
    let _e15 = x_269;
    let _e17 = x_269;
    let _e19 = AF3_x(f32(_e17));
    let _e20 = c_41;
    let _e21 = x_269;
    let _e23 = x_269;
    let _e25 = AF3_x(f32(_e23));
    return pow(_e20, _e25);
}

fn AFromPqF1_(x_270: f32) -> f32 {
    var x_271: f32;
    var p_3: f32;

    x_271 = x_270;
    let _e17 = AF1_x(0.0126833f);
    let _e18 = x_271;
    let _e23 = AF1_x(0.0126833f);
    p_3 = pow(_e18, _e23);
    let _e26 = p_3;
    let _e31 = AF1_x(0.835938f);
    let _e33 = p_3;
    let _e38 = AF1_x(0.835938f);
    let _e40 = ASatF1_((_e33 - _e38));
    let _e45 = AF1_x(18.8516f);
    let _e50 = AF1_x(18.6875f);
    let _e51 = p_3;
    let _e59 = AF1_x(6.27739f);
    let _e60 = p_3;
    let _e65 = AF1_x(0.835938f);
    let _e67 = p_3;
    let _e72 = AF1_x(0.835938f);
    let _e74 = ASatF1_((_e67 - _e72));
    let _e79 = AF1_x(18.8516f);
    let _e84 = AF1_x(18.6875f);
    let _e85 = p_3;
    let _e93 = AF1_x(6.27739f);
    return pow((_e74 / (_e79 - (_e84 * _e85))), _e93);
}

fn AFromPqF1_1(x_272: vec2<f32>) -> vec2<f32> {
    var x_273: vec2<f32>;
    var p_4: vec2<f32>;

    x_273 = x_272;
    let _e17 = AF2_x(0.0126833f);
    let _e18 = x_273;
    let _e23 = AF2_x(0.0126833f);
    p_4 = pow(_e18, _e23);
    let _e26 = p_4;
    let _e31 = AF2_x(0.835938f);
    let _e33 = p_4;
    let _e38 = AF2_x(0.835938f);
    let _e40 = ASatF2_((_e33 - _e38));
    let _e45 = AF2_x(18.8516f);
    let _e50 = AF2_x(18.6875f);
    let _e51 = p_4;
    let _e59 = AF2_x(6.27739f);
    let _e60 = p_4;
    let _e65 = AF2_x(0.835938f);
    let _e67 = p_4;
    let _e72 = AF2_x(0.835938f);
    let _e74 = ASatF2_((_e67 - _e72));
    let _e79 = AF2_x(18.8516f);
    let _e84 = AF2_x(18.6875f);
    let _e85 = p_4;
    let _e93 = AF2_x(6.27739f);
    return pow((_e74 / (_e79 - (_e84 * _e85))), _e93);
}

fn AFromPqF1_2(x_274: vec3<f32>) -> vec3<f32> {
    var x_275: vec3<f32>;
    var p_5: vec3<f32>;

    x_275 = x_274;
    let _e17 = AF3_x(0.0126833f);
    let _e18 = x_275;
    let _e23 = AF3_x(0.0126833f);
    p_5 = pow(_e18, _e23);
    let _e26 = p_5;
    let _e31 = AF3_x(0.835938f);
    let _e33 = p_5;
    let _e38 = AF3_x(0.835938f);
    let _e40 = ASatF3_((_e33 - _e38));
    let _e45 = AF3_x(18.8516f);
    let _e50 = AF3_x(18.6875f);
    let _e51 = p_5;
    let _e59 = AF3_x(6.27739f);
    let _e60 = p_5;
    let _e65 = AF3_x(0.835938f);
    let _e67 = p_5;
    let _e72 = AF3_x(0.835938f);
    let _e74 = ASatF3_((_e67 - _e72));
    let _e79 = AF3_x(18.8516f);
    let _e84 = AF3_x(18.6875f);
    let _e85 = p_5;
    let _e93 = AF3_x(6.27739f);
    return pow((_e74 / (_e79 - (_e84 * _e85))), _e93);
}

fn AFromSrgbF1_(c_42: f32) -> f32 {
    var c_43: f32;
    var j_9: vec3<f32> = vec3<f32>(0.003130805f, 0.07739938f, 2.4f);
    var k_9: vec2<f32> = vec2<f32>(0.94786733f, 0.052132703f);

    c_43 = c_42;
    let _e29 = c_43;
    let _e30 = j_9;
    let _e33 = c_43;
    let _e34 = j_9;
    let _e37 = AZolSignedF1_((_e33 - _e34.x));
    let _e38 = c_43;
    let _e39 = j_9;
    let _e42 = c_43;
    let _e43 = k_9;
    let _e46 = k_9;
    let _e49 = j_9;
    let _e51 = c_43;
    let _e52 = k_9;
    let _e55 = k_9;
    let _e58 = j_9;
    let _e61 = c_43;
    let _e62 = j_9;
    let _e65 = c_43;
    let _e66 = j_9;
    let _e69 = AZolSignedF1_((_e65 - _e66.x));
    let _e70 = c_43;
    let _e71 = j_9;
    let _e74 = c_43;
    let _e75 = k_9;
    let _e78 = k_9;
    let _e81 = j_9;
    let _e83 = c_43;
    let _e84 = k_9;
    let _e87 = k_9;
    let _e90 = j_9;
    let _e93 = AZolSelF1_(_e69, (_e70 * _e71.y), pow(((_e83 * _e84.x) + _e87.y), _e90.z));
    return _e93;
}

fn AFromSrgbF2_(c_44: vec2<f32>) -> vec2<f32> {
    var c_45: vec2<f32>;
    var j_10: vec3<f32> = vec3<f32>(0.003130805f, 0.07739938f, 2.4f);
    var k_10: vec2<f32> = vec2<f32>(0.94786733f, 0.052132703f);

    c_45 = c_44;
    let _e29 = c_45;
    let _e30 = j_10;
    let _e33 = c_45;
    let _e34 = j_10;
    let _e37 = AZolSignedF2_((_e33 - _e34.xx));
    let _e38 = c_45;
    let _e39 = j_10;
    let _e42 = c_45;
    let _e43 = k_10;
    let _e46 = k_10;
    let _e49 = j_10;
    let _e51 = c_45;
    let _e52 = k_10;
    let _e55 = k_10;
    let _e58 = j_10;
    let _e61 = c_45;
    let _e62 = j_10;
    let _e65 = c_45;
    let _e66 = j_10;
    let _e69 = AZolSignedF2_((_e65 - _e66.xx));
    let _e70 = c_45;
    let _e71 = j_10;
    let _e74 = c_45;
    let _e75 = k_10;
    let _e78 = k_10;
    let _e81 = j_10;
    let _e83 = c_45;
    let _e84 = k_10;
    let _e87 = k_10;
    let _e90 = j_10;
    let _e93 = AZolSelF2_(_e69, (_e70 * _e71.yy), pow(((_e83 * _e84.xx) + _e87.yy), _e90.zz));
    return _e93;
}

fn AFromSrgbF3_(c_46: vec3<f32>) -> vec3<f32> {
    var c_47: vec3<f32>;
    var j_11: vec3<f32> = vec3<f32>(0.003130805f, 0.07739938f, 2.4f);
    var k_11: vec2<f32> = vec2<f32>(0.94786733f, 0.052132703f);

    c_47 = c_46;
    let _e29 = c_47;
    let _e30 = j_11;
    let _e33 = c_47;
    let _e34 = j_11;
    let _e37 = AZolSignedF3_((_e33 - _e34.xxx));
    let _e38 = c_47;
    let _e39 = j_11;
    let _e42 = c_47;
    let _e43 = k_11;
    let _e46 = k_11;
    let _e49 = j_11;
    let _e51 = c_47;
    let _e52 = k_11;
    let _e55 = k_11;
    let _e58 = j_11;
    let _e61 = c_47;
    let _e62 = j_11;
    let _e65 = c_47;
    let _e66 = j_11;
    let _e69 = AZolSignedF3_((_e65 - _e66.xxx));
    let _e70 = c_47;
    let _e71 = j_11;
    let _e74 = c_47;
    let _e75 = k_11;
    let _e78 = k_11;
    let _e81 = j_11;
    let _e83 = c_47;
    let _e84 = k_11;
    let _e87 = k_11;
    let _e90 = j_11;
    let _e93 = AZolSelF3_(_e69, (_e70 * _e71.yyy), pow(((_e83 * _e84.xxx) + _e87.yyy), _e90.zzz));
    return _e93;
}

fn AFromTwoF1_(c_48: f32) -> f32 {
    var c_49: f32;

    c_49 = c_48;
    let _e12 = c_49;
    let _e13 = c_49;
    return (_e12 * _e13);
}

fn AFromTwoF2_(c_50: vec2<f32>) -> vec2<f32> {
    var c_51: vec2<f32>;

    c_51 = c_50;
    let _e12 = c_51;
    let _e13 = c_51;
    return (_e12 * _e13);
}

fn AFromTwoF3_(c_52: vec3<f32>) -> vec3<f32> {
    var c_53: vec3<f32>;

    c_53 = c_52;
    let _e12 = c_53;
    let _e13 = c_53;
    return (_e12 * _e13);
}

fn AFromThreeF1_(c_54: f32) -> f32 {
    var c_55: f32;

    c_55 = c_54;
    let _e12 = c_55;
    let _e13 = c_55;
    let _e15 = c_55;
    return ((_e12 * _e13) * _e15);
}

fn AFromThreeF2_(c_56: vec2<f32>) -> vec2<f32> {
    var c_57: vec2<f32>;

    c_57 = c_56;
    let _e12 = c_57;
    let _e13 = c_57;
    let _e15 = c_57;
    return ((_e12 * _e13) * _e15);
}

fn AFromThreeF3_(c_58: vec3<f32>) -> vec3<f32> {
    var c_59: vec3<f32>;

    c_59 = c_58;
    let _e12 = c_59;
    let _e13 = c_59;
    let _e15 = c_59;
    return ((_e12 * _e13) * _e15);
}

fn ARmp8x8_(a_170: u32) -> vec2<u32> {
    var a_171: u32;

    a_171 = a_170;
    let _e15 = a_171;
    let _e18 = ABfe(_e15, 1u, 3u);
    let _e22 = a_171;
    let _e25 = ABfe(_e22, 3u, 3u);
    let _e31 = a_171;
    let _e34 = ABfe(_e31, 3u, 3u);
    let _e35 = a_171;
    let _e37 = ABfiM(_e34, _e35, 1u);
    return vec2<u32>(_e18, _e37);
}

fn ARmpRed8x8_(a_172: u32) -> vec2<u32> {
    var a_173: u32;

    a_173 = a_172;
    let _e15 = a_173;
    let _e18 = ABfe(_e15, 2u, 3u);
    let _e24 = a_173;
    let _e27 = ABfe(_e24, 2u, 3u);
    let _e28 = a_173;
    let _e30 = ABfiM(_e27, _e28, 1u);
    let _e34 = a_173;
    let _e37 = ABfe(_e34, 3u, 3u);
    let _e41 = a_173;
    let _e44 = ABfe(_e41, 1u, 2u);
    let _e49 = a_173;
    let _e52 = ABfe(_e49, 3u, 3u);
    let _e56 = a_173;
    let _e59 = ABfe(_e56, 1u, 2u);
    let _e61 = ABfiM(_e52, _e59, 2u);
    return vec2<u32>(_e30, _e61);
}

fn opAAbsF2_(d_32: ptr<function, vec2<f32>>, a_174: vec2<f32>) -> vec2<f32> {
    var a_175: vec2<f32>;

    a_175 = a_174;
    let _e14 = a_175;
    (*d_32) = abs(_e14);
    let _e16 = (*d_32);
    return _e16;
}

fn opAAbsF3_(d_33: ptr<function, vec3<f32>>, a_176: vec3<f32>) -> vec3<f32> {
    var a_177: vec3<f32>;

    a_177 = a_176;
    let _e14 = a_177;
    (*d_33) = abs(_e14);
    let _e16 = (*d_33);
    return _e16;
}

fn opAAbsF4_(d_34: ptr<function, vec4<f32>>, a_178: vec4<f32>) -> vec4<f32> {
    var a_179: vec4<f32>;

    a_179 = a_178;
    let _e14 = a_179;
    (*d_34) = abs(_e14);
    let _e16 = (*d_34);
    return _e16;
}

fn opAAddF2_(d_35: ptr<function, vec2<f32>>, a_180: vec2<f32>, b_36: vec2<f32>) -> vec2<f32> {
    var a_181: vec2<f32>;
    var b_37: vec2<f32>;

    a_181 = a_180;
    b_37 = b_36;
    let _e15 = a_181;
    let _e16 = b_37;
    (*d_35) = (_e15 + _e16);
    let _e18 = (*d_35);
    return _e18;
}

fn opAAddF3_(d_36: ptr<function, vec3<f32>>, a_182: vec3<f32>, b_38: vec3<f32>) -> vec3<f32> {
    var a_183: vec3<f32>;
    var b_39: vec3<f32>;

    a_183 = a_182;
    b_39 = b_38;
    let _e15 = a_183;
    let _e16 = b_39;
    (*d_36) = (_e15 + _e16);
    let _e18 = (*d_36);
    return _e18;
}

fn opAAddF4_(d_37: ptr<function, vec4<f32>>, a_184: vec4<f32>, b_40: vec4<f32>) -> vec4<f32> {
    var a_185: vec4<f32>;
    var b_41: vec4<f32>;

    a_185 = a_184;
    b_41 = b_40;
    let _e15 = a_185;
    let _e16 = b_41;
    (*d_37) = (_e15 + _e16);
    let _e18 = (*d_37);
    return _e18;
}

fn opAAddOneF2_(d_38: ptr<function, vec2<f32>>, a_186: vec2<f32>, b_42: f32) -> vec2<f32> {
    var a_187: vec2<f32>;
    var b_43: f32;

    a_187 = a_186;
    b_43 = b_42;
    let _e15 = a_187;
    let _e16 = b_43;
    let _e18 = b_43;
    let _e20 = AF2_x(f32(_e18));
    (*d_38) = (_e15 + _e20);
    let _e22 = (*d_38);
    return _e22;
}

fn opAAddOneF3_(d_39: ptr<function, vec3<f32>>, a_188: vec3<f32>, b_44: f32) -> vec3<f32> {
    var a_189: vec3<f32>;
    var b_45: f32;

    a_189 = a_188;
    b_45 = b_44;
    let _e15 = a_189;
    let _e16 = b_45;
    let _e18 = b_45;
    let _e20 = AF3_x(f32(_e18));
    (*d_39) = (_e15 + _e20);
    let _e22 = (*d_39);
    return _e22;
}

fn opAAddOneF4_(d_40: ptr<function, vec4<f32>>, a_190: vec4<f32>, b_46: f32) -> vec4<f32> {
    var a_191: vec4<f32>;
    var b_47: f32;

    a_191 = a_190;
    b_47 = b_46;
    let _e15 = a_191;
    let _e16 = b_47;
    let _e18 = b_47;
    let _e20 = AF4_x(f32(_e18));
    (*d_40) = (_e15 + _e20);
    let _e22 = (*d_40);
    return _e22;
}

fn opACpyF2_(d_41: ptr<function, vec2<f32>>, a_192: vec2<f32>) -> vec2<f32> {
    var a_193: vec2<f32>;

    a_193 = a_192;
    let _e13 = a_193;
    (*d_41) = _e13;
    let _e14 = (*d_41);
    return _e14;
}

fn opACpyF3_(d_42: ptr<function, vec3<f32>>, a_194: vec3<f32>) -> vec3<f32> {
    var a_195: vec3<f32>;

    a_195 = a_194;
    let _e13 = a_195;
    (*d_42) = _e13;
    let _e14 = (*d_42);
    return _e14;
}

fn opACpyF4_(d_43: ptr<function, vec4<f32>>, a_196: vec4<f32>) -> vec4<f32> {
    var a_197: vec4<f32>;

    a_197 = a_196;
    let _e13 = a_197;
    (*d_43) = _e13;
    let _e14 = (*d_43);
    return _e14;
}

fn opALerpF2_(d_44: ptr<function, vec2<f32>>, a_198: vec2<f32>, b_48: vec2<f32>, c_60: vec2<f32>) -> vec2<f32> {
    var a_199: vec2<f32>;
    var b_49: vec2<f32>;
    var c_61: vec2<f32>;

    a_199 = a_198;
    b_49 = b_48;
    c_61 = c_60;
    let _e20 = a_199;
    let _e21 = b_49;
    let _e22 = c_61;
    let _e23 = ALerpF2_(_e20, _e21, _e22);
    (*d_44) = _e23;
    let _e24 = (*d_44);
    return _e24;
}

fn opALerpF3_(d_45: ptr<function, vec3<f32>>, a_200: vec3<f32>, b_50: vec3<f32>, c_62: vec3<f32>) -> vec3<f32> {
    var a_201: vec3<f32>;
    var b_51: vec3<f32>;
    var c_63: vec3<f32>;

    a_201 = a_200;
    b_51 = b_50;
    c_63 = c_62;
    let _e20 = a_201;
    let _e21 = b_51;
    let _e22 = c_63;
    let _e23 = ALerpF3_(_e20, _e21, _e22);
    (*d_45) = _e23;
    let _e24 = (*d_45);
    return _e24;
}

fn opALerpF4_(d_46: ptr<function, vec4<f32>>, a_202: vec4<f32>, b_52: vec4<f32>, c_64: vec4<f32>) -> vec4<f32> {
    var a_203: vec4<f32>;
    var b_53: vec4<f32>;
    var c_65: vec4<f32>;

    a_203 = a_202;
    b_53 = b_52;
    c_65 = c_64;
    let _e20 = a_203;
    let _e21 = b_53;
    let _e22 = c_65;
    let _e23 = ALerpF4_(_e20, _e21, _e22);
    (*d_46) = _e23;
    let _e24 = (*d_46);
    return _e24;
}

fn opALerpOneF2_(d_47: ptr<function, vec2<f32>>, a_204: vec2<f32>, b_54: vec2<f32>, c_66: f32) -> vec2<f32> {
    var a_205: vec2<f32>;
    var b_55: vec2<f32>;
    var c_67: f32;

    a_205 = a_204;
    b_55 = b_54;
    c_67 = c_66;
    let _e19 = c_67;
    let _e21 = c_67;
    let _e23 = AF2_x(f32(_e21));
    let _e24 = a_205;
    let _e25 = b_55;
    let _e26 = c_67;
    let _e28 = c_67;
    let _e30 = AF2_x(f32(_e28));
    let _e31 = ALerpF2_(_e24, _e25, _e30);
    (*d_47) = _e31;
    let _e32 = (*d_47);
    return _e32;
}

fn opALerpOneF3_(d_48: ptr<function, vec3<f32>>, a_206: vec3<f32>, b_56: vec3<f32>, c_68: f32) -> vec3<f32> {
    var a_207: vec3<f32>;
    var b_57: vec3<f32>;
    var c_69: f32;

    a_207 = a_206;
    b_57 = b_56;
    c_69 = c_68;
    let _e19 = c_69;
    let _e21 = c_69;
    let _e23 = AF3_x(f32(_e21));
    let _e24 = a_207;
    let _e25 = b_57;
    let _e26 = c_69;
    let _e28 = c_69;
    let _e30 = AF3_x(f32(_e28));
    let _e31 = ALerpF3_(_e24, _e25, _e30);
    (*d_48) = _e31;
    let _e32 = (*d_48);
    return _e32;
}

fn opALerpOneF4_(d_49: ptr<function, vec4<f32>>, a_208: vec4<f32>, b_58: vec4<f32>, c_70: f32) -> vec4<f32> {
    var a_209: vec4<f32>;
    var b_59: vec4<f32>;
    var c_71: f32;

    a_209 = a_208;
    b_59 = b_58;
    c_71 = c_70;
    let _e19 = c_71;
    let _e21 = c_71;
    let _e23 = AF4_x(f32(_e21));
    let _e24 = a_209;
    let _e25 = b_59;
    let _e26 = c_71;
    let _e28 = c_71;
    let _e30 = AF4_x(f32(_e28));
    let _e31 = ALerpF4_(_e24, _e25, _e30);
    (*d_49) = _e31;
    let _e32 = (*d_49);
    return _e32;
}

fn opAMaxF2_(d_50: ptr<function, vec2<f32>>, a_210: vec2<f32>, b_60: vec2<f32>) -> vec2<f32> {
    var a_211: vec2<f32>;
    var b_61: vec2<f32>;

    a_211 = a_210;
    b_61 = b_60;
    let _e17 = a_211;
    let _e18 = b_61;
    (*d_50) = max(_e17, _e18);
    let _e20 = (*d_50);
    return _e20;
}

fn opAMaxF3_(d_51: ptr<function, vec3<f32>>, a_212: vec3<f32>, b_62: vec3<f32>) -> vec3<f32> {
    var a_213: vec3<f32>;
    var b_63: vec3<f32>;

    a_213 = a_212;
    b_63 = b_62;
    let _e17 = a_213;
    let _e18 = b_63;
    (*d_51) = max(_e17, _e18);
    let _e20 = (*d_51);
    return _e20;
}

fn opAMaxF4_(d_52: ptr<function, vec4<f32>>, a_214: vec4<f32>, b_64: vec4<f32>) -> vec4<f32> {
    var a_215: vec4<f32>;
    var b_65: vec4<f32>;

    a_215 = a_214;
    b_65 = b_64;
    let _e17 = a_215;
    let _e18 = b_65;
    (*d_52) = max(_e17, _e18);
    let _e20 = (*d_52);
    return _e20;
}

fn opAMinF2_(d_53: ptr<function, vec2<f32>>, a_216: vec2<f32>, b_66: vec2<f32>) -> vec2<f32> {
    var a_217: vec2<f32>;
    var b_67: vec2<f32>;

    a_217 = a_216;
    b_67 = b_66;
    let _e17 = a_217;
    let _e18 = b_67;
    (*d_53) = min(_e17, _e18);
    let _e20 = (*d_53);
    return _e20;
}

fn opAMinF3_(d_54: ptr<function, vec3<f32>>, a_218: vec3<f32>, b_68: vec3<f32>) -> vec3<f32> {
    var a_219: vec3<f32>;
    var b_69: vec3<f32>;

    a_219 = a_218;
    b_69 = b_68;
    let _e17 = a_219;
    let _e18 = b_69;
    (*d_54) = min(_e17, _e18);
    let _e20 = (*d_54);
    return _e20;
}

fn opAMinF4_(d_55: ptr<function, vec4<f32>>, a_220: vec4<f32>, b_70: vec4<f32>) -> vec4<f32> {
    var a_221: vec4<f32>;
    var b_71: vec4<f32>;

    a_221 = a_220;
    b_71 = b_70;
    let _e17 = a_221;
    let _e18 = b_71;
    (*d_55) = min(_e17, _e18);
    let _e20 = (*d_55);
    return _e20;
}

fn opAMulF2_(d_56: ptr<function, vec2<f32>>, a_222: vec2<f32>, b_72: vec2<f32>) -> vec2<f32> {
    var a_223: vec2<f32>;
    var b_73: vec2<f32>;

    a_223 = a_222;
    b_73 = b_72;
    let _e15 = a_223;
    let _e16 = b_73;
    (*d_56) = (_e15 * _e16);
    let _e18 = (*d_56);
    return _e18;
}

fn opAMulF3_(d_57: ptr<function, vec3<f32>>, a_224: vec3<f32>, b_74: vec3<f32>) -> vec3<f32> {
    var a_225: vec3<f32>;
    var b_75: vec3<f32>;

    a_225 = a_224;
    b_75 = b_74;
    let _e15 = a_225;
    let _e16 = b_75;
    (*d_57) = (_e15 * _e16);
    let _e18 = (*d_57);
    return _e18;
}

fn opAMulF4_(d_58: ptr<function, vec4<f32>>, a_226: vec4<f32>, b_76: vec4<f32>) -> vec4<f32> {
    var a_227: vec4<f32>;
    var b_77: vec4<f32>;

    a_227 = a_226;
    b_77 = b_76;
    let _e15 = a_227;
    let _e16 = b_77;
    (*d_58) = (_e15 * _e16);
    let _e18 = (*d_58);
    return _e18;
}

fn opAMulOneF2_(d_59: ptr<function, vec2<f32>>, a_228: vec2<f32>, b_78: f32) -> vec2<f32> {
    var a_229: vec2<f32>;
    var b_79: f32;

    a_229 = a_228;
    b_79 = b_78;
    let _e15 = a_229;
    let _e16 = b_79;
    let _e18 = b_79;
    let _e20 = AF2_x(f32(_e18));
    (*d_59) = (_e15 * _e20);
    let _e22 = (*d_59);
    return _e22;
}

fn opAMulOneF3_(d_60: ptr<function, vec3<f32>>, a_230: vec3<f32>, b_80: f32) -> vec3<f32> {
    var a_231: vec3<f32>;
    var b_81: f32;

    a_231 = a_230;
    b_81 = b_80;
    let _e15 = a_231;
    let _e16 = b_81;
    let _e18 = b_81;
    let _e20 = AF3_x(f32(_e18));
    (*d_60) = (_e15 * _e20);
    let _e22 = (*d_60);
    return _e22;
}

fn opAMulOneF4_(d_61: ptr<function, vec4<f32>>, a_232: vec4<f32>, b_82: f32) -> vec4<f32> {
    var a_233: vec4<f32>;
    var b_83: f32;

    a_233 = a_232;
    b_83 = b_82;
    let _e15 = a_233;
    let _e16 = b_83;
    let _e18 = b_83;
    let _e20 = AF4_x(f32(_e18));
    (*d_61) = (_e15 * _e20);
    let _e22 = (*d_61);
    return _e22;
}

fn opANegF2_(d_62: ptr<function, vec2<f32>>, a_234: vec2<f32>) -> vec2<f32> {
    var a_235: vec2<f32>;

    a_235 = a_234;
    let _e13 = a_235;
    (*d_62) = -(_e13);
    let _e15 = (*d_62);
    return _e15;
}

fn opANegF3_(d_63: ptr<function, vec3<f32>>, a_236: vec3<f32>) -> vec3<f32> {
    var a_237: vec3<f32>;

    a_237 = a_236;
    let _e13 = a_237;
    (*d_63) = -(_e13);
    let _e15 = (*d_63);
    return _e15;
}

fn opANegF4_(d_64: ptr<function, vec4<f32>>, a_238: vec4<f32>) -> vec4<f32> {
    var a_239: vec4<f32>;

    a_239 = a_238;
    let _e13 = a_239;
    (*d_64) = -(_e13);
    let _e15 = (*d_64);
    return _e15;
}

fn opARcpF2_(d_65: ptr<function, vec2<f32>>, a_240: vec2<f32>) -> vec2<f32> {
    var a_241: vec2<f32>;

    a_241 = a_240;
    let _e14 = a_241;
    let _e15 = ARcpF2_(_e14);
    (*d_65) = _e15;
    let _e16 = (*d_65);
    return _e16;
}

fn opARcpF3_(d_66: ptr<function, vec3<f32>>, a_242: vec3<f32>) -> vec3<f32> {
    var a_243: vec3<f32>;

    a_243 = a_242;
    let _e14 = a_243;
    let _e15 = ARcpF3_(_e14);
    (*d_66) = _e15;
    let _e16 = (*d_66);
    return _e16;
}

fn opARcpF4_(d_67: ptr<function, vec4<f32>>, a_244: vec4<f32>) -> vec4<f32> {
    var a_245: vec4<f32>;

    a_245 = a_244;
    let _e14 = a_245;
    let _e15 = ARcpF4_(_e14);
    (*d_67) = _e15;
    let _e16 = (*d_67);
    return _e16;
}

fn FsrEasuCon(con0_: ptr<function, vec4<u32>>, con1_: ptr<function, vec4<u32>>, con2_: ptr<function, vec4<u32>>, con3_: ptr<function, vec4<u32>>, inputViewportInPixelsX: f32, inputViewportInPixelsY: f32, inputSizeInPixelsX: f32, inputSizeInPixelsY: f32, outputSizeInPixelsX: f32, outputSizeInPixelsY: f32) {
    var inputViewportInPixelsX_1: f32;
    var inputViewportInPixelsY_1: f32;
    var inputSizeInPixelsX_1: f32;
    var inputSizeInPixelsY_1: f32;
    var outputSizeInPixelsX_1: f32;
    var outputSizeInPixelsY_1: f32;

    inputViewportInPixelsX_1 = inputViewportInPixelsX;
    inputViewportInPixelsY_1 = inputViewportInPixelsY;
    inputSizeInPixelsX_1 = inputSizeInPixelsX;
    inputSizeInPixelsY_1 = inputSizeInPixelsY;
    outputSizeInPixelsX_1 = outputSizeInPixelsX;
    outputSizeInPixelsY_1 = outputSizeInPixelsY;
    let _e31 = inputViewportInPixelsX_1;
    let _e33 = outputSizeInPixelsX_1;
    let _e34 = ARcpF1_(_e33);
    let _e37 = inputViewportInPixelsX_1;
    let _e39 = outputSizeInPixelsX_1;
    let _e40 = ARcpF1_(_e39);
    (*con0_)[0i] = bitcast<u32>(f32((_e37 * _e40)));
    let _e46 = inputViewportInPixelsY_1;
    let _e48 = outputSizeInPixelsY_1;
    let _e49 = ARcpF1_(_e48);
    let _e52 = inputViewportInPixelsY_1;
    let _e54 = outputSizeInPixelsY_1;
    let _e55 = ARcpF1_(_e54);
    (*con0_)[1i] = bitcast<u32>(f32((_e52 * _e55)));
    let _e65 = AF1_x(0.5f);
    let _e66 = inputViewportInPixelsX_1;
    let _e69 = outputSizeInPixelsX_1;
    let _e70 = ARcpF1_(_e69);
    let _e76 = AF1_x(0.5f);
    let _e83 = AF1_x(0.5f);
    let _e84 = inputViewportInPixelsX_1;
    let _e87 = outputSizeInPixelsX_1;
    let _e88 = ARcpF1_(_e87);
    let _e94 = AF1_x(0.5f);
    (*con0_)[2i] = bitcast<u32>(f32((((_e83 * _e84) * _e88) - _e94)));
    let _e104 = AF1_x(0.5f);
    let _e105 = inputViewportInPixelsY_1;
    let _e108 = outputSizeInPixelsY_1;
    let _e109 = ARcpF1_(_e108);
    let _e115 = AF1_x(0.5f);
    let _e122 = AF1_x(0.5f);
    let _e123 = inputViewportInPixelsY_1;
    let _e126 = outputSizeInPixelsY_1;
    let _e127 = ARcpF1_(_e126);
    let _e133 = AF1_x(0.5f);
    (*con0_)[3i] = bitcast<u32>(f32((((_e122 * _e123) * _e127) - _e133)));
    let _e140 = inputSizeInPixelsX_1;
    let _e141 = ARcpF1_(_e140);
    let _e144 = inputSizeInPixelsX_1;
    let _e145 = ARcpF1_(_e144);
    (*con1_)[0i] = bitcast<u32>(f32(_e145));
    let _e151 = inputSizeInPixelsY_1;
    let _e152 = ARcpF1_(_e151);
    let _e155 = inputSizeInPixelsY_1;
    let _e156 = ARcpF1_(_e155);
    (*con1_)[1i] = bitcast<u32>(f32(_e156));
    let _e165 = AF1_x(1f);
    let _e167 = inputSizeInPixelsX_1;
    let _e168 = ARcpF1_(_e167);
    let _e175 = AF1_x(1f);
    let _e177 = inputSizeInPixelsX_1;
    let _e178 = ARcpF1_(_e177);
    (*con1_)[2i] = bitcast<u32>(f32((_e175 * _e178)));
    let _e190 = AF1_x(-1f);
    let _e192 = inputSizeInPixelsY_1;
    let _e193 = ARcpF1_(_e192);
    let _e202 = AF1_x(-1f);
    let _e204 = inputSizeInPixelsY_1;
    let _e205 = ARcpF1_(_e204);
    (*con1_)[3i] = bitcast<u32>(f32((_e202 * _e205)));
    let _e217 = AF1_x(-1f);
    let _e219 = inputSizeInPixelsX_1;
    let _e220 = ARcpF1_(_e219);
    let _e229 = AF1_x(-1f);
    let _e231 = inputSizeInPixelsX_1;
    let _e232 = ARcpF1_(_e231);
    (*con2_)[0i] = bitcast<u32>(f32((_e229 * _e232)));
    let _e242 = AF1_x(2f);
    let _e244 = inputSizeInPixelsY_1;
    let _e245 = ARcpF1_(_e244);
    let _e252 = AF1_x(2f);
    let _e254 = inputSizeInPixelsY_1;
    let _e255 = ARcpF1_(_e254);
    (*con2_)[1i] = bitcast<u32>(f32((_e252 * _e255)));
    let _e265 = AF1_x(1f);
    let _e267 = inputSizeInPixelsX_1;
    let _e268 = ARcpF1_(_e267);
    let _e275 = AF1_x(1f);
    let _e277 = inputSizeInPixelsX_1;
    let _e278 = ARcpF1_(_e277);
    (*con2_)[2i] = bitcast<u32>(f32((_e275 * _e278)));
    let _e288 = AF1_x(2f);
    let _e290 = inputSizeInPixelsY_1;
    let _e291 = ARcpF1_(_e290);
    let _e298 = AF1_x(2f);
    let _e300 = inputSizeInPixelsY_1;
    let _e301 = ARcpF1_(_e300);
    (*con2_)[3i] = bitcast<u32>(f32((_e298 * _e301)));
    let _e311 = AF1_x(0f);
    let _e313 = inputSizeInPixelsX_1;
    let _e314 = ARcpF1_(_e313);
    let _e321 = AF1_x(0f);
    let _e323 = inputSizeInPixelsX_1;
    let _e324 = ARcpF1_(_e323);
    (*con3_)[0i] = bitcast<u32>(f32((_e321 * _e324)));
    let _e334 = AF1_x(4f);
    let _e336 = inputSizeInPixelsY_1;
    let _e337 = ARcpF1_(_e336);
    let _e344 = AF1_x(4f);
    let _e346 = inputSizeInPixelsY_1;
    let _e347 = ARcpF1_(_e346);
    (*con3_)[1i] = bitcast<u32>(f32((_e344 * _e347)));
    (*con3_)[3i] = 0u;
    (*con3_)[2i] = 0u;
    return;
}

fn FsrEasuConOffset(con0_1: ptr<function, vec4<u32>>, con1_1: ptr<function, vec4<u32>>, con2_1: ptr<function, vec4<u32>>, con3_1: ptr<function, vec4<u32>>, inputViewportInPixelsX_2: f32, inputViewportInPixelsY_2: f32, inputSizeInPixelsX_2: f32, inputSizeInPixelsY_2: f32, outputSizeInPixelsX_2: f32, outputSizeInPixelsY_2: f32, inputOffsetInPixelsX: f32, inputOffsetInPixelsY: f32) {
    var inputViewportInPixelsX_3: f32;
    var inputViewportInPixelsY_3: f32;
    var inputSizeInPixelsX_3: f32;
    var inputSizeInPixelsY_3: f32;
    var outputSizeInPixelsX_3: f32;
    var outputSizeInPixelsY_3: f32;
    var inputOffsetInPixelsX_1: f32;
    var inputOffsetInPixelsY_1: f32;

    inputViewportInPixelsX_3 = inputViewportInPixelsX_2;
    inputViewportInPixelsY_3 = inputViewportInPixelsY_2;
    inputSizeInPixelsX_3 = inputSizeInPixelsX_2;
    inputSizeInPixelsY_3 = inputSizeInPixelsY_2;
    outputSizeInPixelsX_3 = outputSizeInPixelsX_2;
    outputSizeInPixelsY_3 = outputSizeInPixelsY_2;
    inputOffsetInPixelsX_1 = inputOffsetInPixelsX;
    inputOffsetInPixelsY_1 = inputOffsetInPixelsY;
    let _e47 = inputViewportInPixelsX_3;
    let _e48 = inputViewportInPixelsY_3;
    let _e49 = inputSizeInPixelsX_3;
    let _e50 = inputSizeInPixelsY_3;
    let _e51 = outputSizeInPixelsX_3;
    let _e52 = outputSizeInPixelsY_3;
    FsrEasuCon(con0_1, con1_1, con2_1, con3_1, _e47, _e48, _e49, _e50, _e51, _e52);
    let _e59 = AF1_x(0.5f);
    let _e60 = inputViewportInPixelsX_3;
    let _e63 = outputSizeInPixelsX_3;
    let _e64 = ARcpF1_(_e63);
    let _e70 = AF1_x(0.5f);
    let _e72 = inputOffsetInPixelsX_1;
    let _e79 = AF1_x(0.5f);
    let _e80 = inputViewportInPixelsX_3;
    let _e83 = outputSizeInPixelsX_3;
    let _e84 = ARcpF1_(_e83);
    let _e90 = AF1_x(0.5f);
    let _e92 = inputOffsetInPixelsX_1;
    (*con0_1)[2i] = bitcast<u32>(f32(((((_e79 * _e80) * _e84) - _e90) + _e92)));
    let _e102 = AF1_x(0.5f);
    let _e103 = inputViewportInPixelsY_3;
    let _e106 = outputSizeInPixelsY_3;
    let _e107 = ARcpF1_(_e106);
    let _e113 = AF1_x(0.5f);
    let _e115 = inputOffsetInPixelsY_1;
    let _e122 = AF1_x(0.5f);
    let _e123 = inputViewportInPixelsY_3;
    let _e126 = outputSizeInPixelsY_3;
    let _e127 = ARcpF1_(_e126);
    let _e133 = AF1_x(0.5f);
    let _e135 = inputOffsetInPixelsY_1;
    (*con0_1)[3i] = bitcast<u32>(f32(((((_e122 * _e123) * _e127) - _e133) + _e135)));
    return;
}

fn FsrEasuTapF(aC: ptr<function, vec3<f32>>, aW: ptr<function, f32>, off_2: vec2<f32>, dir: vec2<f32>, len: vec2<f32>, lob: f32, clp: f32, c_72: vec3<f32>) {
    var off_3: vec2<f32>;
    var dir_1: vec2<f32>;
    var len_1: vec2<f32>;
    var lob_1: f32;
    var clp_1: f32;
    var c_73: vec3<f32>;
    var v: vec2<f32>;
    var d2_: f32;
    var wB: f32;
    var wA: f32;
    var w: f32;

    off_3 = off_2;
    dir_1 = dir;
    len_1 = len;
    lob_1 = lob;
    clp_1 = clp;
    c_73 = c_72;
    let _e29 = off_3;
    let _e31 = dir_1;
    let _e34 = off_3;
    let _e36 = dir_1;
    v.x = ((_e29.x * _e31.x) + (_e34.y * _e36.y));
    let _e41 = off_3;
    let _e43 = dir_1;
    let _e47 = off_3;
    let _e49 = dir_1;
    v.y = ((_e41.x * -(_e43.y)) + (_e47.y * _e49.x));
    let _e53 = v;
    let _e54 = len_1;
    v = (_e53 * _e54);
    let _e56 = v;
    let _e58 = v;
    let _e61 = v;
    let _e63 = v;
    d2_ = ((_e56.x * _e58.x) + (_e61.y * _e63.y));
    let _e70 = d2_;
    let _e71 = clp_1;
    d2_ = min(_e70, _e71);
    let _e81 = AF1_x(0.4f);
    let _e82 = d2_;
    let _e90 = AF1_x(-1f);
    wB = ((_e81 * _e82) + _e90);
    let _e93 = lob_1;
    let _e94 = d2_;
    let _e102 = AF1_x(-1f);
    wA = ((_e93 * _e94) + _e102);
    let _e105 = wB;
    let _e106 = wB;
    wB = (_e105 * _e106);
    let _e108 = wA;
    let _e109 = wA;
    wA = (_e108 * _e109);
    let _e119 = AF1_x(1.5625f);
    let _e120 = wB;
    let _e136 = AF1_x(-0.5625f);
    wB = ((_e119 * _e120) + _e136);
    let _e138 = wB;
    let _e139 = wA;
    w = (_e138 * _e139);
    let _e142 = (*aC);
    let _e143 = c_73;
    let _e144 = w;
    (*aC) = (_e142 + (_e143 * _e144));
    let _e147 = (*aW);
    let _e148 = w;
    (*aW) = (_e147 + _e148);
    return;
}

fn FsrEasuSetF(dir_2: ptr<function, vec2<f32>>, len_2: ptr<function, f32>, pp: vec2<f32>, biS: bool, biT: bool, biU: bool, biV: bool, lA: f32, lB: f32, lC: f32, lD: f32, lE: f32) {
    var pp_1: vec2<f32>;
    var biS_1: bool;
    var biT_1: bool;
    var biU_1: bool;
    var biV_1: bool;
    var lA_1: f32;
    var lB_1: f32;
    var lC_1: f32;
    var lD_1: f32;
    var lE_1: f32;
    var w_1: f32;
    var dc: f32;
    var cb: f32;
    var lenX: f32;
    var dirX: f32;
    var ec: f32;
    var ca: f32;
    var lenY: f32;
    var dirY: f32;

    pp_1 = pp;
    biS_1 = biS;
    biT_1 = biT;
    biU_1 = biU;
    biV_1 = biV;
    lA_1 = lA;
    lB_1 = lB;
    lC_1 = lC;
    lD_1 = lD;
    lE_1 = lE;
    let _e39 = AF1_x(0f);
    w_1 = _e39;
    let _e41 = biS_1;
    if _e41 {
        let _e46 = AF1_x(1f);
        let _e47 = pp_1;
        let _e54 = AF1_x(1f);
        let _e55 = pp_1;
        w_1 = ((_e46 - _e47.x) * (_e54 - _e55.y));
    }
    let _e59 = biT_1;
    if _e59 {
        let _e60 = pp_1;
        let _e66 = AF1_x(1f);
        let _e67 = pp_1;
        w_1 = (_e60.x * (_e66 - _e67.y));
    }
    let _e71 = biU_1;
    if _e71 {
        let _e76 = AF1_x(1f);
        let _e77 = pp_1;
        let _e80 = pp_1;
        w_1 = ((_e76 - _e77.x) * _e80.y);
    }
    let _e83 = biV_1;
    if _e83 {
        let _e84 = pp_1;
        let _e86 = pp_1;
        w_1 = (_e84.x * _e86.y);
    }
    let _e89 = lD_1;
    let _e90 = lC_1;
    dc = (_e89 - _e90);
    let _e93 = lC_1;
    let _e94 = lB_1;
    cb = (_e93 - _e94);
    let _e98 = dc;
    let _e101 = cb;
    let _e104 = dc;
    let _e107 = cb;
    lenX = max(abs(_e104), abs(_e107));
    let _e112 = lenX;
    let _e113 = APrxLoRcpF1_(_e112);
    lenX = _e113;
    let _e114 = lD_1;
    let _e115 = lB_1;
    dirX = (_e114 - _e115);
    let _e119 = (*dir_2);
    let _e121 = dirX;
    let _e122 = w_1;
    (*dir_2).x = (_e119.x + (_e121 * _e122));
    let _e126 = dirX;
    let _e128 = lenX;
    let _e131 = dirX;
    let _e133 = lenX;
    let _e135 = ASatF1_((abs(_e131) * _e133));
    lenX = _e135;
    let _e136 = lenX;
    let _e137 = lenX;
    lenX = (_e136 * _e137);
    let _e139 = (*len_2);
    let _e140 = lenX;
    let _e141 = w_1;
    (*len_2) = (_e139 + (_e140 * _e141));
    let _e144 = lE_1;
    let _e145 = lC_1;
    ec = (_e144 - _e145);
    let _e148 = lC_1;
    let _e149 = lA_1;
    ca = (_e148 - _e149);
    let _e153 = ec;
    let _e156 = ca;
    let _e159 = ec;
    let _e162 = ca;
    lenY = max(abs(_e159), abs(_e162));
    let _e167 = lenY;
    let _e168 = APrxLoRcpF1_(_e167);
    lenY = _e168;
    let _e169 = lE_1;
    let _e170 = lA_1;
    dirY = (_e169 - _e170);
    let _e174 = (*dir_2);
    let _e176 = dirY;
    let _e177 = w_1;
    (*dir_2).y = (_e174.y + (_e176 * _e177));
    let _e181 = dirY;
    let _e183 = lenY;
    let _e186 = dirY;
    let _e188 = lenY;
    let _e190 = ASatF1_((abs(_e186) * _e188));
    lenY = _e190;
    let _e191 = lenY;
    let _e192 = lenY;
    lenY = (_e191 * _e192);
    let _e194 = (*len_2);
    let _e195 = lenY;
    let _e196 = w_1;
    (*len_2) = (_e194 + (_e195 * _e196));
    return;
}

fn FsrEasuF(pix: ptr<function, vec3<f32>>, ip: vec2<u32>, con0_2: vec4<u32>, con1_2: vec4<u32>, con2_2: vec4<u32>, con3_2: vec4<u32>) {
    var ip_1: vec2<u32>;
    var con0_3: vec4<u32>;
    var con1_3: vec4<u32>;
    var con2_3: vec4<u32>;
    var con3_3: vec4<u32>;
    var pp_2: vec2<f32>;
    var fp: vec2<f32>;
    var p0_: vec2<f32>;
    var p1_: vec2<f32>;
    var p2_: vec2<f32>;
    var p3_: vec2<f32>;
    var bczzR: vec4<f32>;
    var bczzG: vec4<f32>;
    var bczzB: vec4<f32>;
    var ijfeR: vec4<f32>;
    var ijfeG: vec4<f32>;
    var ijfeB: vec4<f32>;
    var klhgR: vec4<f32>;
    var klhgG: vec4<f32>;
    var klhgB: vec4<f32>;
    var zzonR: vec4<f32>;
    var zzonG: vec4<f32>;
    var zzonB: vec4<f32>;
    var bczzL: vec4<f32>;
    var ijfeL: vec4<f32>;
    var klhgL: vec4<f32>;
    var zzonL: vec4<f32>;
    var bL: f32;
    var cL: f32;
    var iL: f32;
    var jL: f32;
    var fL: f32;
    var eL: f32;
    var kL: f32;
    var lL: f32;
    var hL: f32;
    var gL: f32;
    var oL: f32;
    var nL: f32;
    var dir_3: vec2<f32>;
    var len_3: f32;
    var dir2_: vec2<f32>;
    var dirR: f32;
    var zro: bool;
    var local_8: f32;
    var local_9: f32;
    var stretch: f32;
    var len2_: vec2<f32>;
    var lob_2: f32;
    var clp_2: f32;
    var min4_: vec3<f32>;
    var max4_: vec3<f32>;
    var aC_1: vec3<f32>;
    var aW_1: f32;

    ip_1 = ip;
    con0_3 = con0_2;
    con1_3 = con1_2;
    con2_3 = con2_2;
    con3_3 = con3_2;
    let _e24 = ip_1;
    let _e26 = con0_3;
    let _e29 = con0_3;
    let _e34 = con0_3;
    let _e37 = con0_3;
    pp_2 = ((vec2<f32>(_e24) * bitcast<vec2<f32>>(vec2<u32>(_e29.xy))) + bitcast<vec2<f32>>(vec2<u32>(_e37.zw)));
    let _e44 = pp_2;
    fp = floor(_e44);
    let _e47 = pp_2;
    let _e48 = fp;
    pp_2 = (_e47 - _e48);
    let _e50 = fp;
    let _e51 = con1_3;
    let _e54 = con1_3;
    let _e59 = con1_3;
    let _e62 = con1_3;
    p0_ = ((_e50 * bitcast<vec2<f32>>(vec2<u32>(_e54.xy))) + bitcast<vec2<f32>>(vec2<u32>(_e62.zw)));
    let _e68 = p0_;
    let _e69 = con2_3;
    let _e72 = con2_3;
    p1_ = (_e68 + bitcast<vec2<f32>>(vec2<u32>(_e72.xy)));
    let _e78 = p0_;
    let _e79 = con2_3;
    let _e82 = con2_3;
    p2_ = (_e78 + bitcast<vec2<f32>>(vec2<u32>(_e82.zw)));
    let _e88 = p0_;
    let _e89 = con3_3;
    let _e92 = con3_3;
    p3_ = (_e88 + bitcast<vec2<f32>>(vec2<u32>(_e92.xy)));
    let _e99 = p0_;
    let _e100 = FsrEasuRF(_e99);
    bczzR = _e100;
    let _e103 = p0_;
    let _e104 = FsrEasuGF(_e103);
    bczzG = _e104;
    let _e107 = p0_;
    let _e108 = FsrEasuBF(_e107);
    bczzB = _e108;
    let _e111 = p1_;
    let _e112 = FsrEasuRF(_e111);
    ijfeR = _e112;
    let _e115 = p1_;
    let _e116 = FsrEasuGF(_e115);
    ijfeG = _e116;
    let _e119 = p1_;
    let _e120 = FsrEasuBF(_e119);
    ijfeB = _e120;
    let _e123 = p2_;
    let _e124 = FsrEasuRF(_e123);
    klhgR = _e124;
    let _e127 = p2_;
    let _e128 = FsrEasuGF(_e127);
    klhgG = _e128;
    let _e131 = p2_;
    let _e132 = FsrEasuBF(_e131);
    klhgB = _e132;
    let _e135 = p3_;
    let _e136 = FsrEasuRF(_e135);
    zzonR = _e136;
    let _e139 = p3_;
    let _e140 = FsrEasuGF(_e139);
    zzonG = _e140;
    let _e143 = p3_;
    let _e144 = FsrEasuBF(_e143);
    zzonB = _e144;
    let _e146 = bczzB;
    let _e151 = AF4_x(0.5f);
    let _e153 = bczzR;
    let _e158 = AF4_x(0.5f);
    let _e160 = bczzG;
    bczzL = ((_e146 * _e151) + ((_e153 * _e158) + _e160));
    let _e164 = ijfeB;
    let _e169 = AF4_x(0.5f);
    let _e171 = ijfeR;
    let _e176 = AF4_x(0.5f);
    let _e178 = ijfeG;
    ijfeL = ((_e164 * _e169) + ((_e171 * _e176) + _e178));
    let _e182 = klhgB;
    let _e187 = AF4_x(0.5f);
    let _e189 = klhgR;
    let _e194 = AF4_x(0.5f);
    let _e196 = klhgG;
    klhgL = ((_e182 * _e187) + ((_e189 * _e194) + _e196));
    let _e200 = zzonB;
    let _e205 = AF4_x(0.5f);
    let _e207 = zzonR;
    let _e212 = AF4_x(0.5f);
    let _e214 = zzonG;
    zzonL = ((_e200 * _e205) + ((_e207 * _e212) + _e214));
    let _e218 = bczzL;
    bL = _e218.x;
    let _e221 = bczzL;
    cL = _e221.y;
    let _e224 = ijfeL;
    iL = _e224.x;
    let _e227 = ijfeL;
    jL = _e227.y;
    let _e230 = ijfeL;
    fL = _e230.z;
    let _e233 = ijfeL;
    eL = _e233.w;
    let _e236 = klhgL;
    kL = _e236.x;
    let _e239 = klhgL;
    lL = _e239.y;
    let _e242 = klhgL;
    hL = _e242.z;
    let _e245 = klhgL;
    gL = _e245.w;
    let _e248 = zzonL;
    oL = _e248.z;
    let _e251 = zzonL;
    nL = _e251.w;
    let _e258 = AF2_x(0f);
    dir_3 = _e258;
    let _e264 = AF1_x(0f);
    len_3 = _e264;
    let _e280 = pp_2;
    let _e285 = bL;
    let _e286 = eL;
    let _e287 = fL;
    let _e288 = gL;
    let _e289 = jL;
    FsrEasuSetF((&dir_3), (&len_3), _e280, true, false, false, false, _e285, _e286, _e287, _e288, _e289);
    let _e304 = pp_2;
    let _e309 = cL;
    let _e310 = fL;
    let _e311 = gL;
    let _e312 = hL;
    let _e313 = kL;
    FsrEasuSetF((&dir_3), (&len_3), _e304, false, true, false, false, _e309, _e310, _e311, _e312, _e313);
    let _e328 = pp_2;
    let _e333 = fL;
    let _e334 = iL;
    let _e335 = jL;
    let _e336 = kL;
    let _e337 = nL;
    FsrEasuSetF((&dir_3), (&len_3), _e328, false, false, true, false, _e333, _e334, _e335, _e336, _e337);
    let _e352 = pp_2;
    let _e357 = gL;
    let _e358 = jL;
    let _e359 = kL;
    let _e360 = lL;
    let _e361 = oL;
    FsrEasuSetF((&dir_3), (&len_3), _e352, false, false, false, true, _e357, _e358, _e359, _e360, _e361);
    let _e362 = dir_3;
    let _e363 = dir_3;
    dir2_ = (_e362 * _e363);
    let _e366 = dir2_;
    let _e368 = dir2_;
    dirR = (_e366.x + _e368.y);
    let _e372 = dirR;
    let _e381 = AF1_x(0.000030517578f);
    zro = (_e372 < _e381);
    let _e385 = dirR;
    let _e386 = APrxLoRsqF1_(_e385);
    dirR = _e386;
    let _e387 = zro;
    if _e387 {
        let _e392 = AF1_x(1f);
        local_8 = _e392;
    } else {
        let _e393 = dirR;
        local_8 = _e393;
    }
    let _e395 = local_8;
    dirR = _e395;
    let _e397 = zro;
    if _e397 {
        let _e402 = AF1_x(1f);
        local_9 = _e402;
    } else {
        let _e403 = dir_3;
        local_9 = _e403.x;
    }
    let _e406 = local_9;
    dir_3.x = _e406;
    let _e407 = dir_3;
    let _e408 = dirR;
    let _e410 = dirR;
    let _e412 = AF2_x(f32(_e410));
    dir_3 = (_e407 * _e412);
    let _e414 = len_3;
    let _e419 = AF1_x(0.5f);
    len_3 = (_e414 * _e419);
    let _e421 = len_3;
    let _e422 = len_3;
    len_3 = (_e421 * _e422);
    let _e424 = dir_3;
    let _e426 = dir_3;
    let _e429 = dir_3;
    let _e431 = dir_3;
    let _e435 = dir_3;
    let _e437 = dir_3;
    let _e440 = dir_3;
    let _e442 = dir_3;
    let _e445 = dir_3;
    let _e447 = dir_3;
    let _e450 = dir_3;
    let _e452 = dir_3;
    let _e456 = dir_3;
    let _e458 = dir_3;
    let _e461 = dir_3;
    let _e463 = dir_3;
    let _e466 = dir_3;
    let _e468 = dir_3;
    let _e471 = dir_3;
    let _e473 = dir_3;
    let _e477 = APrxLoRcpF1_(max(abs(_e468.x), abs(_e473.y)));
    stretch = (((_e424.x * _e426.x) + (_e429.y * _e431.y)) * _e477);
    let _e484 = AF1_x(1f);
    let _e485 = stretch;
    let _e490 = AF1_x(1f);
    let _e492 = len_3;
    let _e499 = AF1_x(1f);
    let _e506 = AF1_x(-0.5f);
    let _e507 = len_3;
    len2_ = vec2<f32>((_e484 + ((_e485 - _e490) * _e492)), (_e499 + (_e506 * _e507)));
    let _e516 = AF1_x(0.5f);
    let _e533 = AF1_x(-0.29f);
    let _e534 = len_3;
    lob_2 = (_e516 + (_e533 * _e534));
    let _e539 = lob_2;
    let _e540 = APrxLoRcpF1_(_e539);
    clp_2 = _e540;
    let _e542 = ijfeR;
    let _e544 = ijfeG;
    let _e546 = ijfeB;
    let _e549 = klhgR;
    let _e551 = klhgG;
    let _e553 = klhgB;
    let _e556 = ijfeR;
    let _e558 = ijfeG;
    let _e560 = ijfeB;
    let _e563 = ijfeR;
    let _e565 = ijfeG;
    let _e567 = ijfeB;
    let _e570 = klhgR;
    let _e572 = klhgG;
    let _e574 = klhgB;
    let _e577 = ijfeR;
    let _e579 = ijfeG;
    let _e581 = ijfeB;
    let _e584 = AMin3F3_(vec3<f32>(_e563.z, _e565.z, _e567.z), vec3<f32>(_e570.w, _e572.w, _e574.w), vec3<f32>(_e577.y, _e579.y, _e581.y));
    let _e585 = klhgR;
    let _e587 = klhgG;
    let _e589 = klhgB;
    let _e592 = ijfeR;
    let _e594 = ijfeG;
    let _e596 = ijfeB;
    let _e599 = klhgR;
    let _e601 = klhgG;
    let _e603 = klhgB;
    let _e606 = ijfeR;
    let _e608 = ijfeG;
    let _e610 = ijfeB;
    let _e613 = ijfeR;
    let _e615 = ijfeG;
    let _e617 = ijfeB;
    let _e620 = klhgR;
    let _e622 = klhgG;
    let _e624 = klhgB;
    let _e627 = ijfeR;
    let _e629 = ijfeG;
    let _e631 = ijfeB;
    let _e634 = AMin3F3_(vec3<f32>(_e613.z, _e615.z, _e617.z), vec3<f32>(_e620.w, _e622.w, _e624.w), vec3<f32>(_e627.y, _e629.y, _e631.y));
    let _e635 = klhgR;
    let _e637 = klhgG;
    let _e639 = klhgB;
    min4_ = min(_e634, vec3<f32>(_e635.x, _e637.x, _e639.x));
    let _e644 = ijfeR;
    let _e646 = ijfeG;
    let _e648 = ijfeB;
    let _e651 = klhgR;
    let _e653 = klhgG;
    let _e655 = klhgB;
    let _e658 = ijfeR;
    let _e660 = ijfeG;
    let _e662 = ijfeB;
    let _e665 = ijfeR;
    let _e667 = ijfeG;
    let _e669 = ijfeB;
    let _e672 = klhgR;
    let _e674 = klhgG;
    let _e676 = klhgB;
    let _e679 = ijfeR;
    let _e681 = ijfeG;
    let _e683 = ijfeB;
    let _e686 = AMax3F3_(vec3<f32>(_e665.z, _e667.z, _e669.z), vec3<f32>(_e672.w, _e674.w, _e676.w), vec3<f32>(_e679.y, _e681.y, _e683.y));
    let _e687 = klhgR;
    let _e689 = klhgG;
    let _e691 = klhgB;
    let _e694 = ijfeR;
    let _e696 = ijfeG;
    let _e698 = ijfeB;
    let _e701 = klhgR;
    let _e703 = klhgG;
    let _e705 = klhgB;
    let _e708 = ijfeR;
    let _e710 = ijfeG;
    let _e712 = ijfeB;
    let _e715 = ijfeR;
    let _e717 = ijfeG;
    let _e719 = ijfeB;
    let _e722 = klhgR;
    let _e724 = klhgG;
    let _e726 = klhgB;
    let _e729 = ijfeR;
    let _e731 = ijfeG;
    let _e733 = ijfeB;
    let _e736 = AMax3F3_(vec3<f32>(_e715.z, _e717.z, _e719.z), vec3<f32>(_e722.w, _e724.w, _e726.w), vec3<f32>(_e729.y, _e731.y, _e733.y));
    let _e737 = klhgR;
    let _e739 = klhgG;
    let _e741 = klhgB;
    max4_ = max(_e736, vec3<f32>(_e737.x, _e739.x, _e741.x));
    let _e750 = AF3_x(0f);
    aC_1 = _e750;
    let _e756 = AF1_x(0f);
    aW_1 = _e756;
    let _e764 = pp_2;
    let _e770 = bczzR;
    let _e772 = bczzG;
    let _e774 = bczzB;
    let _e783 = pp_2;
    let _e785 = dir_3;
    let _e786 = len2_;
    let _e787 = lob_2;
    let _e788 = clp_2;
    let _e789 = bczzR;
    let _e791 = bczzG;
    let _e793 = bczzB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(0f, -1f) - _e783), _e785, _e786, _e787, _e788, vec3<f32>(_e789.x, _e791.x, _e793.x));
    let _e802 = pp_2;
    let _e808 = bczzR;
    let _e810 = bczzG;
    let _e812 = bczzB;
    let _e821 = pp_2;
    let _e823 = dir_3;
    let _e824 = len2_;
    let _e825 = lob_2;
    let _e826 = clp_2;
    let _e827 = bczzR;
    let _e829 = bczzG;
    let _e831 = bczzB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(1f, -1f) - _e821), _e823, _e824, _e825, _e826, vec3<f32>(_e827.y, _e829.y, _e831.y));
    let _e840 = pp_2;
    let _e846 = ijfeR;
    let _e848 = ijfeG;
    let _e850 = ijfeB;
    let _e859 = pp_2;
    let _e861 = dir_3;
    let _e862 = len2_;
    let _e863 = lob_2;
    let _e864 = clp_2;
    let _e865 = ijfeR;
    let _e867 = ijfeG;
    let _e869 = ijfeB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(-1f, 1f) - _e859), _e861, _e862, _e863, _e864, vec3<f32>(_e865.x, _e867.x, _e869.x));
    let _e877 = pp_2;
    let _e883 = ijfeR;
    let _e885 = ijfeG;
    let _e887 = ijfeB;
    let _e895 = pp_2;
    let _e897 = dir_3;
    let _e898 = len2_;
    let _e899 = lob_2;
    let _e900 = clp_2;
    let _e901 = ijfeR;
    let _e903 = ijfeG;
    let _e905 = ijfeB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(0f, 1f) - _e895), _e897, _e898, _e899, _e900, vec3<f32>(_e901.y, _e903.y, _e905.y));
    let _e913 = pp_2;
    let _e919 = ijfeR;
    let _e921 = ijfeG;
    let _e923 = ijfeB;
    let _e931 = pp_2;
    let _e933 = dir_3;
    let _e934 = len2_;
    let _e935 = lob_2;
    let _e936 = clp_2;
    let _e937 = ijfeR;
    let _e939 = ijfeG;
    let _e941 = ijfeB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(0f, 0f) - _e931), _e933, _e934, _e935, _e936, vec3<f32>(_e937.z, _e939.z, _e941.z));
    let _e950 = pp_2;
    let _e956 = ijfeR;
    let _e958 = ijfeG;
    let _e960 = ijfeB;
    let _e969 = pp_2;
    let _e971 = dir_3;
    let _e972 = len2_;
    let _e973 = lob_2;
    let _e974 = clp_2;
    let _e975 = ijfeR;
    let _e977 = ijfeG;
    let _e979 = ijfeB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(-1f, 0f) - _e969), _e971, _e972, _e973, _e974, vec3<f32>(_e975.w, _e977.w, _e979.w));
    let _e987 = pp_2;
    let _e993 = klhgR;
    let _e995 = klhgG;
    let _e997 = klhgB;
    let _e1005 = pp_2;
    let _e1007 = dir_3;
    let _e1008 = len2_;
    let _e1009 = lob_2;
    let _e1010 = clp_2;
    let _e1011 = klhgR;
    let _e1013 = klhgG;
    let _e1015 = klhgB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(1f, 1f) - _e1005), _e1007, _e1008, _e1009, _e1010, vec3<f32>(_e1011.x, _e1013.x, _e1015.x));
    let _e1023 = pp_2;
    let _e1029 = klhgR;
    let _e1031 = klhgG;
    let _e1033 = klhgB;
    let _e1041 = pp_2;
    let _e1043 = dir_3;
    let _e1044 = len2_;
    let _e1045 = lob_2;
    let _e1046 = clp_2;
    let _e1047 = klhgR;
    let _e1049 = klhgG;
    let _e1051 = klhgB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(2f, 1f) - _e1041), _e1043, _e1044, _e1045, _e1046, vec3<f32>(_e1047.y, _e1049.y, _e1051.y));
    let _e1059 = pp_2;
    let _e1065 = klhgR;
    let _e1067 = klhgG;
    let _e1069 = klhgB;
    let _e1077 = pp_2;
    let _e1079 = dir_3;
    let _e1080 = len2_;
    let _e1081 = lob_2;
    let _e1082 = clp_2;
    let _e1083 = klhgR;
    let _e1085 = klhgG;
    let _e1087 = klhgB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(2f, 0f) - _e1077), _e1079, _e1080, _e1081, _e1082, vec3<f32>(_e1083.z, _e1085.z, _e1087.z));
    let _e1095 = pp_2;
    let _e1101 = klhgR;
    let _e1103 = klhgG;
    let _e1105 = klhgB;
    let _e1113 = pp_2;
    let _e1115 = dir_3;
    let _e1116 = len2_;
    let _e1117 = lob_2;
    let _e1118 = clp_2;
    let _e1119 = klhgR;
    let _e1121 = klhgG;
    let _e1123 = klhgB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(1f, 0f) - _e1113), _e1115, _e1116, _e1117, _e1118, vec3<f32>(_e1119.w, _e1121.w, _e1123.w));
    let _e1131 = pp_2;
    let _e1137 = zzonR;
    let _e1139 = zzonG;
    let _e1141 = zzonB;
    let _e1149 = pp_2;
    let _e1151 = dir_3;
    let _e1152 = len2_;
    let _e1153 = lob_2;
    let _e1154 = clp_2;
    let _e1155 = zzonR;
    let _e1157 = zzonG;
    let _e1159 = zzonB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(1f, 2f) - _e1149), _e1151, _e1152, _e1153, _e1154, vec3<f32>(_e1155.z, _e1157.z, _e1159.z));
    let _e1167 = pp_2;
    let _e1173 = zzonR;
    let _e1175 = zzonG;
    let _e1177 = zzonB;
    let _e1185 = pp_2;
    let _e1187 = dir_3;
    let _e1188 = len2_;
    let _e1189 = lob_2;
    let _e1190 = clp_2;
    let _e1191 = zzonR;
    let _e1193 = zzonG;
    let _e1195 = zzonB;
    FsrEasuTapF((&aC_1), (&aW_1), (vec2<f32>(0f, 2f) - _e1185), _e1187, _e1188, _e1189, _e1190, vec3<f32>(_e1191.w, _e1193.w, _e1195.w));
    let _e1200 = aC_1;
    let _e1202 = aW_1;
    let _e1203 = ARcpF1_(_e1202);
    let _e1206 = aW_1;
    let _e1207 = ARcpF1_(_e1206);
    let _e1209 = AF3_x(f32(_e1207));
    let _e1211 = min4_;
    let _e1212 = aC_1;
    let _e1214 = aW_1;
    let _e1215 = ARcpF1_(_e1214);
    let _e1218 = aW_1;
    let _e1219 = ARcpF1_(_e1218);
    let _e1221 = AF3_x(f32(_e1219));
    let _e1224 = max4_;
    let _e1226 = aC_1;
    let _e1228 = aW_1;
    let _e1229 = ARcpF1_(_e1228);
    let _e1232 = aW_1;
    let _e1233 = ARcpF1_(_e1232);
    let _e1235 = AF3_x(f32(_e1233));
    let _e1237 = min4_;
    let _e1238 = aC_1;
    let _e1240 = aW_1;
    let _e1241 = ARcpF1_(_e1240);
    let _e1244 = aW_1;
    let _e1245 = ARcpF1_(_e1244);
    let _e1247 = AF3_x(f32(_e1245));
    (*pix) = min(_e1224, max(_e1237, (_e1238 * _e1247)));
    return;
}

fn FsrRcasCon(con: ptr<function, vec4<u32>>, sharpness: f32) {
    var sharpness_1: f32;
    var hSharp: vec2<f32>;

    sharpness_1 = sharpness;
    let _e16 = sharpness_1;
    let _e19 = sharpness_1;
    sharpness_1 = exp2(f32(-(_e19)));
    let _e23 = sharpness_1;
    let _e24 = sharpness_1;
    hSharp = vec2<f32>(_e23, _e24);
    let _e29 = sharpness_1;
    let _e31 = sharpness_1;
    (*con)[0i] = bitcast<u32>(f32(_e31));
    let _e37 = hSharp;
    (*con)[1i] = pack2x16float(_e37);
    (*con)[2i] = 0u;
    (*con)[3i] = 0u;
    return;
}

fn FsrLfgaF(c_74: ptr<function, vec3<f32>>, t: vec3<f32>, a_246: f32) {
    var t_1: vec3<f32>;
    var a_247: f32;

    t_1 = t;
    a_247 = a_246;
    let _e18 = (*c_74);
    let _e19 = t_1;
    let _e20 = a_247;
    let _e22 = a_247;
    let _e24 = AF3_x(f32(_e22));
    let _e30 = AF3_x(1f);
    let _e31 = (*c_74);
    let _e38 = AF3_x(1f);
    let _e39 = (*c_74);
    let _e41 = (*c_74);
    (*c_74) = (_e18 + ((_e19 * _e24) * min((_e38 - _e39), _e41)));
    return;
}

fn FsrSrtmF(c_75: ptr<function, vec3<f32>>) {
    let _e14 = (*c_75);
    let _e15 = (*c_75);
    let _e17 = (*c_75);
    let _e19 = (*c_75);
    let _e21 = (*c_75);
    let _e23 = (*c_75);
    let _e25 = (*c_75);
    let _e27 = AMax3F1_(_e21.x, _e23.y, _e25.z);
    let _e32 = AF1_x(1f);
    let _e34 = (*c_75);
    let _e36 = (*c_75);
    let _e38 = (*c_75);
    let _e40 = (*c_75);
    let _e42 = (*c_75);
    let _e44 = (*c_75);
    let _e46 = AMax3F1_(_e40.x, _e42.y, _e44.z);
    let _e51 = AF1_x(1f);
    let _e53 = ARcpF1_((_e46 + _e51));
    let _e55 = (*c_75);
    let _e57 = (*c_75);
    let _e59 = (*c_75);
    let _e61 = (*c_75);
    let _e63 = (*c_75);
    let _e65 = (*c_75);
    let _e67 = AMax3F1_(_e61.x, _e63.y, _e65.z);
    let _e72 = AF1_x(1f);
    let _e74 = (*c_75);
    let _e76 = (*c_75);
    let _e78 = (*c_75);
    let _e80 = (*c_75);
    let _e82 = (*c_75);
    let _e84 = (*c_75);
    let _e86 = AMax3F1_(_e80.x, _e82.y, _e84.z);
    let _e91 = AF1_x(1f);
    let _e93 = ARcpF1_((_e86 + _e91));
    let _e95 = AF3_x(f32(_e93));
    (*c_75) = (_e14 * _e95);
    return;
}

fn FsrSrtmInvF(c_76: ptr<function, vec3<f32>>) {
    let _e14 = (*c_76);
    let _e23 = AF1_x(0.000030517578f);
    let _e28 = AF1_x(1f);
    let _e29 = (*c_76);
    let _e31 = (*c_76);
    let _e33 = (*c_76);
    let _e35 = (*c_76);
    let _e37 = (*c_76);
    let _e39 = (*c_76);
    let _e41 = AMax3F1_(_e35.x, _e37.y, _e39.z);
    let _e51 = AF1_x(0.000030517578f);
    let _e56 = AF1_x(1f);
    let _e57 = (*c_76);
    let _e59 = (*c_76);
    let _e61 = (*c_76);
    let _e63 = (*c_76);
    let _e65 = (*c_76);
    let _e67 = (*c_76);
    let _e69 = AMax3F1_(_e63.x, _e65.y, _e67.z);
    let _e80 = AF1_x(0.000030517578f);
    let _e85 = AF1_x(1f);
    let _e86 = (*c_76);
    let _e88 = (*c_76);
    let _e90 = (*c_76);
    let _e92 = (*c_76);
    let _e94 = (*c_76);
    let _e96 = (*c_76);
    let _e98 = AMax3F1_(_e92.x, _e94.y, _e96.z);
    let _e108 = AF1_x(0.000030517578f);
    let _e113 = AF1_x(1f);
    let _e114 = (*c_76);
    let _e116 = (*c_76);
    let _e118 = (*c_76);
    let _e120 = (*c_76);
    let _e122 = (*c_76);
    let _e124 = (*c_76);
    let _e126 = AMax3F1_(_e120.x, _e122.y, _e124.z);
    let _e129 = ARcpF1_(max(_e108, (_e113 - _e126)));
    let _e139 = AF1_x(0.000030517578f);
    let _e144 = AF1_x(1f);
    let _e145 = (*c_76);
    let _e147 = (*c_76);
    let _e149 = (*c_76);
    let _e151 = (*c_76);
    let _e153 = (*c_76);
    let _e155 = (*c_76);
    let _e157 = AMax3F1_(_e151.x, _e153.y, _e155.z);
    let _e167 = AF1_x(0.000030517578f);
    let _e172 = AF1_x(1f);
    let _e173 = (*c_76);
    let _e175 = (*c_76);
    let _e177 = (*c_76);
    let _e179 = (*c_76);
    let _e181 = (*c_76);
    let _e183 = (*c_76);
    let _e185 = AMax3F1_(_e179.x, _e181.y, _e183.z);
    let _e196 = AF1_x(0.000030517578f);
    let _e201 = AF1_x(1f);
    let _e202 = (*c_76);
    let _e204 = (*c_76);
    let _e206 = (*c_76);
    let _e208 = (*c_76);
    let _e210 = (*c_76);
    let _e212 = (*c_76);
    let _e214 = AMax3F1_(_e208.x, _e210.y, _e212.z);
    let _e224 = AF1_x(0.000030517578f);
    let _e229 = AF1_x(1f);
    let _e230 = (*c_76);
    let _e232 = (*c_76);
    let _e234 = (*c_76);
    let _e236 = (*c_76);
    let _e238 = (*c_76);
    let _e240 = (*c_76);
    let _e242 = AMax3F1_(_e236.x, _e238.y, _e240.z);
    let _e245 = ARcpF1_(max(_e224, (_e229 - _e242)));
    let _e247 = AF3_x(f32(_e245));
    (*c_76) = (_e14 * _e247);
    return;
}

fn FsrTepdDitF(p_12: vec2<u32>, f: u32) -> f32 {
    var p_13: vec2<u32>;
    var f_1: u32;
    var x_276: f32;
    var y_129: f32;
    var a_248: f32;
    var b_84: f32;

    p_13 = p_12;
    f_1 = f;
    let _e17 = p_13;
    let _e19 = f_1;
    let _e22 = p_13;
    let _e24 = f_1;
    let _e27 = AF1_x(f32((_e22.x + _e24)));
    x_276 = _e27;
    let _e29 = p_13;
    let _e32 = p_13;
    let _e35 = AF1_x(f32(_e32.y));
    y_129 = _e35;
    let _e53 = AF1_x(1.618034f);
    a_248 = _e53;
    let _e63 = AF1_x(0.2710027f);
    b_84 = _e63;
    let _e65 = x_276;
    let _e66 = a_248;
    let _e68 = y_129;
    let _e69 = b_84;
    x_276 = ((_e65 * _e66) + (_e68 * _e69));
    let _e73 = x_276;
    let _e74 = AFractF1_(_e73);
    return _e74;
}

fn FsrTepdC8F(c_77: ptr<function, vec3<f32>>, dit: f32) {
    var dit_1: f32;
    var n_8: vec3<f32>;
    var a_249: vec3<f32>;
    var b_85: vec3<f32>;
    var r_4: vec3<f32>;

    dit_1 = dit;
    let _e17 = (*c_77);
    n_8 = sqrt(_e17);
    let _e20 = n_8;
    let _e25 = AF3_x(255f);
    let _e27 = n_8;
    let _e32 = AF3_x(255f);
    let _e43 = AF3_x(0.003921569f);
    n_8 = (floor((_e27 * _e32)) * _e43);
    let _e45 = n_8;
    let _e46 = n_8;
    a_249 = (_e45 * _e46);
    let _e49 = n_8;
    let _e58 = AF3_x(0.003921569f);
    b_85 = (_e49 + _e58);
    let _e61 = b_85;
    let _e62 = b_85;
    b_85 = (_e61 * _e62);
    let _e64 = (*c_77);
    let _e65 = b_85;
    let _e67 = a_249;
    let _e68 = b_85;
    let _e70 = a_249;
    let _e71 = b_85;
    let _e73 = APrxMedRcpF3_((_e70 - _e71));
    r_4 = ((_e64 - _e65) * _e73);
    let _e76 = n_8;
    let _e77 = dit_1;
    let _e79 = dit_1;
    let _e81 = AF3_x(f32(_e79));
    let _e82 = r_4;
    let _e84 = dit_1;
    let _e86 = dit_1;
    let _e88 = AF3_x(f32(_e86));
    let _e89 = r_4;
    let _e91 = AGtZeroF3_((_e88 - _e89));
    let _e100 = AF3_x(0.003921569f);
    let _e103 = n_8;
    let _e104 = dit_1;
    let _e106 = dit_1;
    let _e108 = AF3_x(f32(_e106));
    let _e109 = r_4;
    let _e111 = dit_1;
    let _e113 = dit_1;
    let _e115 = AF3_x(f32(_e113));
    let _e116 = r_4;
    let _e118 = AGtZeroF3_((_e115 - _e116));
    let _e127 = AF3_x(0.003921569f);
    let _e130 = ASatF3_((_e103 + (_e118 * _e127)));
    (*c_77) = _e130;
    return;
}

fn FsrTepdC10F(c_78: ptr<function, vec3<f32>>, dit_2: f32) {
    var dit_3: f32;
    var n_9: vec3<f32>;
    var a_250: vec3<f32>;
    var b_86: vec3<f32>;
    var r_5: vec3<f32>;

    dit_3 = dit_2;
    let _e17 = (*c_78);
    n_9 = sqrt(_e17);
    let _e20 = n_9;
    let _e25 = AF3_x(1023f);
    let _e27 = n_9;
    let _e32 = AF3_x(1023f);
    let _e43 = AF3_x(0.0009775171f);
    n_9 = (floor((_e27 * _e32)) * _e43);
    let _e45 = n_9;
    let _e46 = n_9;
    a_250 = (_e45 * _e46);
    let _e49 = n_9;
    let _e58 = AF3_x(0.0009775171f);
    b_86 = (_e49 + _e58);
    let _e61 = b_86;
    let _e62 = b_86;
    b_86 = (_e61 * _e62);
    let _e64 = (*c_78);
    let _e65 = b_86;
    let _e67 = a_250;
    let _e68 = b_86;
    let _e70 = a_250;
    let _e71 = b_86;
    let _e73 = APrxMedRcpF3_((_e70 - _e71));
    r_5 = ((_e64 - _e65) * _e73);
    let _e76 = n_9;
    let _e77 = dit_3;
    let _e79 = dit_3;
    let _e81 = AF3_x(f32(_e79));
    let _e82 = r_5;
    let _e84 = dit_3;
    let _e86 = dit_3;
    let _e88 = AF3_x(f32(_e86));
    let _e89 = r_5;
    let _e91 = AGtZeroF3_((_e88 - _e89));
    let _e100 = AF3_x(0.0009775171f);
    let _e103 = n_9;
    let _e104 = dit_3;
    let _e106 = dit_3;
    let _e108 = AF3_x(f32(_e106));
    let _e109 = r_5;
    let _e111 = dit_3;
    let _e113 = dit_3;
    let _e115 = AF3_x(f32(_e113));
    let _e116 = r_5;
    let _e118 = AGtZeroF3_((_e115 - _e116));
    let _e127 = AF3_x(0.0009775171f);
    let _e130 = ASatF3_((_e103 + (_e118 * _e127)));
    (*c_78) = _e130;
    return;
}

// fn CurrFilter(pos: vec2<u32>) -> vec3<f32> {
//     var pos_1: vec2<u32>;
//     var c_79: vec3<f32>;

//     pos_1 = pos;
//     let _e23 = pos_1;
//     let _e24 = global.Const0_;
//     let _e25 = global.Const1_;
//     let _e26 = global.Const2_;
//     let _e27 = global.Const3_;
//     FsrEasuF((&c_79), _e23, _e24, _e25, _e26, _e27);
//     let _e28 = global.Sample;
//     if _e28.x == 1u {
//         let _e33 = c_79;
//         let _e34 = c_79;
//         c_79 = (_e33 * _e34);
//     }
//     let _e36 = pos_1;
//     let _e38 = c_79;
//     let _e45 = pos_1;
//     let _e47 = c_79;
//     // textureStore(OutputTexture, vec2<i32>(_e45), vec4<f32>(_e47.x, _e47.y, _e47.z, 1f));
//     return _e47;
// }

// fn main_1() {
//     var gxy: vec2<u32>;

//     let _e15 = gl_LocalInvocationID_1;
//     let _e17 = gl_LocalInvocationID_1;
//     let _e19 = ARmp8x8_(_e17.x);
//     let _e20 = gl_WorkGroupID_1;
//     let _e24 = gl_WorkGroupID_1;
//     gxy = (_e19 + vec2<u32>((_e20.x << 4u), (_e24.y << 4u)));
//     let _e32 = gxy;
//     CurrFilter(_e32);
//     let _e34 = gxy;
//     gxy.x = (_e34.x + 8u);
//     let _e39 = gxy;
//     CurrFilter(_e39);
//     let _e41 = gxy;
//     gxy.y = (_e41.y + 8u);
//     let _e46 = gxy;
//     CurrFilter(_e46);
//     let _e48 = gxy;
//     gxy.x = (_e48.x - 8u);
//     let _e53 = gxy;
//     CurrFilter(_e53);
//     return;
// }

// @compute @workgroup_size(64, 1, 1) 
// fn main(@builtin(local_invocation_id) gl_LocalInvocationID: vec3<u32>, @builtin(workgroup_id) gl_WorkGroupID: vec3<u32>) {
//     gl_LocalInvocationID_1 = gl_LocalInvocationID;
//     gl_WorkGroupID_1 = gl_WorkGroupID;
//     main_1();
//     return;
// }
