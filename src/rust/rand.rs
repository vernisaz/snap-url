/*
* Rust translation of PCG Random Number Generation for C++
*
* Copyright 2014 Melissa O'Neill <oneill@pcg-random.org>
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*
* For additional information about the PCG random number generation scheme,
* including its license and other licensing options, visit
*
*     http://www.pcg-random.org
*/

#[derive(Debug)]
pub struct PCG32 {
    state: u64,
    inc: u64,
}

#[repr(C)]
union f32_or_i32 {
    u: u32,
    f: f32,
}

const DEFAULT_STATE: u64 = 0x853c49e6748fea9b;
const DEFAULT_STREAM: u64 = 0xda3e39cb94b95bdb;
const MULT: u64 = 0x5851f42d4c957f2d;

impl PCG32 {
    pub fn new() -> Self {
        PCG32 {
            state: DEFAULT_STATE,
            inc: DEFAULT_STREAM,
        }
    }

    pub fn seed(&mut self, init_state: u64, init_seq: u64) {
        self.state = 0;
        self.inc = (init_seq << 1) | 1;
        let _ = self.next_u32();
        self.state = self.state.wrapping_add(init_state);
        let _ = self.next_u32();
    }

    pub fn next_u32(&mut self) -> u32 {
        let oldstate = self.state;
        self.state = oldstate.wrapping_mul(MULT).wrapping_add(self.inc);
        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59u32) as u32;
        (xorshifted >> rot) | (xorshifted << (((!rot).wrapping_add(1u32)) & 31))
    }

    #[allow(dead_code)]
    pub fn next_u32_bounded(&mut self, bound: u32) -> u32 {
        // To avoid bias, we need to make the range of the RNG a multiple of
        // bound, which we do by dropping output less than a threshold.
        // A naive scheme to calculate the threshold would be to do
        //
        //     uint32_t threshold = 0x100000000ull % bound;
        //
        // but 64-bit div/mod is slower than 32-bit div/mod (especially on
        // 32-bit platforms).  In essence, we do
        //
        //     uint32_t threshold = (0x100000000ull-bound) % bound;
        //
        // because this version will calculate the same modulus, but the LHS
        // value is less than 2^32.

        let threshold = (!bound + 1) % bound;

        // Uniformity guarantees that this loop will terminate.  In practice, it
        // should usually terminate quickly; on average (assuming all bounds are
        // equally likely), 82.25% of the time, we can expect it to require just
        // one iteration.  In the worst case, someone passes a bound of 2^31 + 1
        // (i.e., 2147483649), which invalidates almost 50% of the range.  In
        // practice, bounds are typically small and only a tiny amount of the range
        // is eliminated.
        loop {
            let r = self.next_u32();
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[allow(dead_code)]
    pub fn next_float(&mut self) -> f32 {
        let next_uint = self.next_u32();
        unsafe {
            let x = f32_or_i32 {
                u: (next_uint >> 9) | 0x3f800000,
            };
            x.f - 1.0
        }
    }

    pub fn next_double(&mut self) -> f64 {
        self.next_u32() as f64 * 2.328_306_436_538_696_3e-10
    }

    pub fn gen_range(&mut self, low: f64, high: f64) -> f64 {
        let r = self.next_double();
        if low < high {
            r * (high - low) + low
        } else {
            r * (low - high) + high
        }
    }

    #[allow(dead_code)]
    pub fn advance(&mut self, delta: i64) {
        let mut acc_mult: u64 = 1;
        let mut acc_plus: u64 = 0;
        let mut cur_mult = MULT;
        let mut cur_plus = self.inc;
        let mut delta = delta as u64;

        while delta > 0 {
            if delta & 1 != 0 {
                acc_mult = acc_mult.wrapping_mul(cur_mult);
                acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
            }
            cur_plus = cur_mult.wrapping_mul(cur_plus).wrapping_add(cur_plus);
            cur_mult = cur_mult.wrapping_mul(cur_mult);
            delta /= 2;
        }
        self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
    }

    #[allow(dead_code)]
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        if slice.len() <= 1 {
            return;
        }
        assert!(slice.len() <= u32::MAX as usize);

        for i in (1..slice.len()).rev() {
            let chosen = self.next_u32_bounded(i as u32 + 1) as usize;
            slice.swap(i, chosen);
        }
    }
}

impl Default for PCG32 {
    fn default() -> Self {
        Self::new()
    }
}
