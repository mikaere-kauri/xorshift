// Copyright 2016 Alexander Stocko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::num::Wrapping as w;
use rand::{Rand, Rng, SeedableRng};

#[derive(Copy, Clone)]
pub struct SplitMix64(u64);

impl Rng for SplitMix64 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut z = w(self.0) + w(0x9E3779B97F4A7C15_u64);
        self.0 = z.0;
        z = (z ^ (z >> 30)) * w(0xBF58476D1CE4E5B9_u64);
        z = (z ^ (z >> 27)) * w(0x94D049BB133111EB_u64);
        (z ^ (z >> 31)).0
    }
}

impl SeedableRng<u64> for SplitMix64 {
    fn reseed(&mut self, seed: u64) {
        self.0 = seed;
    }

    fn from_seed(seed: u64) -> SplitMix64 {
        SplitMix64(seed)
    }
}

impl Rand for SplitMix64 {
    fn rand<R: Rng>(other: &mut R) -> SplitMix64 {
        SeedableRng::from_seed(other.gen())
    }
}


#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use super::SplitMix64;
    #[test]
    fn test() {
        // Calculated from reference implementation
        // https://github.com/astocko/xorshift-cpp
        let seed: u64 = 1477776061723855037;
        let t_vals: Vec<u64> = vec![1985237415132408290,
                                    2979275885539914483,
                                    13511426838097143398,
                                    8488337342461049707,
                                    15141737807933549159,
                                    17093170987380407015,
                                    16389528042912955399,
                                    13177319091862933652,
                                    10841969400225389492,
                                    17094824097954834098,
                                    3336622647361835228,
                                    9678412372263018368,
                                    11111587619974030187,
                                    7882215801036322410,
                                    5709234165213761869,
                                    7799681907651786826,
                                    4616320717312661886,
                                    4251077652075509767,
                                    7836757050122171900,
                                    5054003328188417616,
                                    12919285918354108358,
                                    16477564761813870717,
                                    5124667218451240549,
                                    18099554314556827626,
                                    7603784838804469118,
                                    6358551455431362471,
                                    3037176434532249502,
                                    3217550417701719149,
                                    9958699920490216947,
                                    5965803675992506258,
                                    12000828378049868312,
                                    12720568162811471118,
                                    245696019213873792,
                                    8351371993958923852,
                                    14378754021282935786,
                                    5655432093647472106,
                                    5508031680350692005,
                                    8515198786865082103,
                                    6287793597487164412,
                                    14963046237722101617,
                                    3630795823534910476,
                                    8422285279403485710,
                                    10554287778700714153,
                                    10871906555720704584,
                                    8659066966120258468,
                                    9420238805069527062,
                                    10338115333623340156,
                                    13514802760105037173,
                                    14635952304031724449,
                                    15419692541594102413];

        let mut rng: SplitMix64 = SeedableRng::from_seed(seed);
        let vals = rng.gen_iter::<u64>().take(t_vals.len()).collect::<Vec<u64>>();
        assert!(::test::iter_eq(t_vals, vals));
    }
}
