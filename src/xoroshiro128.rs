// Copyright 2016 Alexander Stocko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::num::Wrapping as w;
use rand::{Rand, Rng, SeedableRng};
use RngJump;

const STATE_SIZE: usize = 2;

#[derive(Copy, Clone)]
pub struct Xoroshiro128([u64; 2]);

static EMPTY: Xoroshiro128 = Xoroshiro128([0, 0]);
static JUMP: [u64; 2] = [0xbeac0467eba5facb, 0xd86b048b86aa9922];

#[inline]
fn rotl(x: u64, k: i32) -> u64 {
    (x << k) | (x >> (64 - k))
}

impl Rng for Xoroshiro128 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let s0 = w(self.0[0]);
        let mut s1 = w(self.0[1]);
        let result = s0 + s1;

        s1 ^= s0;
        self.0[0] = (w(rotl(s0.0, 55)) ^ s1 ^ (s1 << 14)).0;
        self.0[1] = rotl(s1.0, 36);

        result.0
    }
}

impl<'a> SeedableRng<&'a [u64]> for Xoroshiro128 {
    fn reseed(&mut self, seed: &'a [u64]) {
        if seed.len() < 2 {
            panic!("Xoroshiro128 seed needs at least two u64s for seeding.");
        }
        self.0[0] = seed[0];
        self.0[1] = seed[1];
    }

    fn from_seed(seed: &'a [u64]) -> Xoroshiro128 {
        let mut rng = EMPTY;
        rng.reseed(seed);
        rng
    }
}

impl Rand for Xoroshiro128 {
    fn rand<R: Rng>(other: &mut R) -> Xoroshiro128 {
        let mut key: [u64; STATE_SIZE] = [0; STATE_SIZE];
        for word in key.iter_mut() {
            *word = other.gen();
        }
        SeedableRng::from_seed(&key[..])
    }
}

impl RngJump for Xoroshiro128 {
    fn jump(&mut self, count: usize) {
        for _ in 0..count {
            let mut s0: u64 = 0;
            let mut s1: u64 = 0;

            for i in 0..JUMP.len() {
                for b in 0..64 {
                    if (JUMP[i] & 1 << b) != 0 {
                        s0 ^= self.0[0];
                        s1 ^= self.0[1];
                    }
                    self.next_u64();
                }
            }
            self.0[0] = s0;
            self.0[1] = s1;
        }
    }
}


#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use super::Xoroshiro128;
    #[test]
    fn test() {
        // Calculated from reference implementation
        // https://github.com/astocko/xorshift-cpp
        let seed: u64 = 1477776328140003287;
        let t_vals: Vec<u64> = vec![2955552656280006574,
                                    16972449677822927371,
                                    7745721154813139207,
                                    12997958984192882321,
                                    4860378213520716854,
                                    8726511682199311786,
                                    4967513430844037468,
                                    8198976591537859742,
                                    9550424487982531115,
                                    4998682132896022152,
                                    13530700387126949659,
                                    4863306358944123927,
                                    6496460551288602950,
                                    6300357993177847246,
                                    12981686428016233582,
                                    12822865705859271257,
                                    2796743621789288691,
                                    8661416515684566800,
                                    11445987918223307471,
                                    1790853738844129809,
                                    2512856687931852193,
                                    16961358987206987195,
                                    16831923336886883616,
                                    1799620397890053848,
                                    4161295844397818624,
                                    11706748128305355888,
                                    12617353356118917788,
                                    8547805800213650247,
                                    10603793685490426181,
                                    2685147166973982615,
                                    11631827950742619990,
                                    17869005055181116877,
                                    2020111105125139909,
                                    16554904763398876336,
                                    9181122027598760409,
                                    9525691846569931390,
                                    12672329911556000760,
                                    1151541992527799435,
                                    4599060499520055258,
                                    221771256380528480,
                                    1278551507256768851,
                                    6765526366205621730,
                                    17926663798966796569,
                                    2326731362433357863,
                                    3573739488452626027,
                                    12112678412767368200,
                                    11945823449132469584,
                                    18281508020577789940,
                                    17522627411608091340,
                                    6715575954761285513];

        let states = [seed, seed];
        let mut rng: Xoroshiro128 = SeedableRng::from_seed(&states[..]);
        let vals = rng.gen_iter::<u64>().take(t_vals.len()).collect::<Vec<u64>>();
        assert!(::test::iter_eq(t_vals, vals));
    }
}
