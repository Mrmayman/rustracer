//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Copyright (c) 2018-2019 Michele Morrone
//  All rights reserved.
//
//  https://michelemorrone.eu - https://BrutPitt.com
//
//  me@michelemorrone.eu - brutpitt@gmail.com
//  twitter: @BrutPitt - github: BrutPitt
//  
//  https://github.com/BrutPitt/glslSmartDeNoise/
//
//  This software is distributed under the terms of the BSD 2-Clause license
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

const INV_SQRT_OF_2PI = 0.39894228040143267793994605993439;  // 1.0/SQRT_OF_2PI
const INV_PI = 0.31830988618379067153776752674503;

//  smartDeNoise - parameters
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
//  sampler2D tex     - sampler image / texture
//  vec2 uv           - actual fragment coord
//  let sigma  >  0 - sigma Standard Deviation
//  let kSigma >= 0 - sigma coefficient 
//      kSigma * sigma  -->  radius of the circular kernel
//  let threshold   - edge sharpening threshold 

fn smartDeNoise(uv: vec2<f32>, sigma: f32, kSigma: f32, threshold: f32) -> vec4<f32> {
    let radius = round(kSigma * sigma);
    let radQ = radius * radius;

    let invSigmaQx2 = .5 / (sigma * sigma);      // 1.0 / (sigma^2 * 2.0)
    let invSigmaQx2PI = INV_PI * invSigmaQx2;    // 1/(2 * PI * sigma^2)

    let invThresholdSqx2 = .5 / (threshold * threshold);     // 1.0 / (sigma^2 * 2.0)
    let invThresholdSqrt2PI = INV_SQRT_OF_2PI / threshold;   // 1.0 / (sqrt(2*PI) * sigma^2)

    let centrPx: vec4<f32> = textureSample(texture, texture_sampler, uv);

    var zBuff = 0.0;
    var aBuff = vec4(0.0);
    let size = vec2(data.width, data.height) / data.scale_factor;

    var d = vec2<f32>(0.0);
    for (d.x = -radius; d.x <= radius; d.x += 1.0) {
        let pt = sqrt(radQ - d.x * d.x);       // pt = yRadius: have circular trend
        for (d.y = -pt; d.y <= pt; d.y += 1.0) {
            let blurFactor = exp(-dot(d, d) * invSigmaQx2) * invSigmaQx2PI;

            let walkPx = textureSample(texture, texture_sampler, uv + d / size);
            let dC = walkPx - centrPx;
            let deltaFactor = exp(-dot(dC, dC) * invThresholdSqx2) * invThresholdSqrt2PI * blurFactor;

            zBuff += deltaFactor;
            aBuff += deltaFactor * walkPx;
        }
    }
    return aBuff / zBuff;
}