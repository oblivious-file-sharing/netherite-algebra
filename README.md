# Netherite Algebra Library

This is the algebra library for an on-going project called Netherite.

## Components

The library implements the following classical BN254 curve based on the parameters in [\[herumi/ate-pairing\]](https://github.com/herumi/ate-pairing) in the [arkworks](https://github.com/arkworks-rs/) ecosystem.
 > `p = 36u^4 + 36u^3 + 24u^2 + 6u + 1`, 
 > `u = -(2^62 + 2^55 + 1)`
 
The paperwork to compute auxiliary parameters is done in Sage scripts here: [src/curve_bn254/sage_scripts](src/curves_bn254/sage_scripts) .

## Authors

- Tianchen Liu [@tcliu](https://github.com/tc-liu), UC Berkeley
- Weikeng Chen [@weikengchen](https://github.com/weikengchen/), UC Berkeley