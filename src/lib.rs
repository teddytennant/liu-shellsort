//! Shellsort gaps from Bo Liu, arXiv 2609.29881.
//!
//! `h_t = floor(alpha * R^t)` with the exact rationals
//! `alpha = 420574650882923 / 7668945023518835` and
//! `R = 582942583375009 / 250000000000000`.
//! The executable sequence is `{1, 3, 8, 20, 47, 116, 300, 585}` union `{h_t : t >= 12}`.

use num_bigint::BigUint;

const ALPHA_NUM: u64 = 420574650882923;
const ALPHA_DEN: u64 = 7668945023518835;
const R_NUM: u64 = 582942583375009;
const R_DEN: u64 = 250000000000000;

/// Tuned prefix. It replaces backbone terms `h_9`, `h_10`, and `h_11` only.
const EXEC_PREFIX: [u64; 8] = [1, 3, 8, 20, 47, 116, 300, 585];

/// Closed-form terms join the executable sequence at this index.
const EXEC_GEOMETRIC_FROM: u32 = 12;

/// Geometric terms past this cap already exceed any `u64` length.
const GEOMETRIC_CAP: usize = 128;

/// Running value of `alpha * R^t`, kept as an unreduced rational.
struct Ratio {
    num: BigUint,
    den: BigUint,
    r_num: BigUint,
    r_den: BigUint,
    t: u32,
}

impl Ratio {
    fn new() -> Self {
        Self {
            num: BigUint::from(ALPHA_NUM),
            den: BigUint::from(ALPHA_DEN),
            r_num: BigUint::from(R_NUM),
            r_den: BigUint::from(R_DEN),
            t: 0,
        }
    }

    /// `floor(alpha * R^t)` by integer division. Advances only forward.
    fn floor_at(&mut self, t: u32) -> BigUint {
        while self.t < t {
            self.num *= &self.r_num;
            self.den *= &self.r_den;
            self.t += 1;
        }
        &self.num / &self.den
    }
}

fn as_u64(h: &BigUint) -> Option<u64> {
    u64::try_from(h).ok()
}

/// Backbone term `h_t = floor(alpha * R^t)`.
///
/// Panics if the term does not fit in `u64`.
pub fn h_t(t: u32) -> u64 {
    let h = Ratio::new().floor_at(t);
    as_u64(&h).expect("h_t does not fit in u64")
}

/// First `count` executable gaps, ascending.
///
/// Panics if a requested term does not fit in `u64`.
pub fn exec_gaps(count: usize) -> Vec<u64> {
    let mut out = Vec::with_capacity(count);
    out.extend(EXEC_PREFIX.into_iter().take(count));
    if out.len() == count {
        return out;
    }
    let mut ratio = Ratio::new();
    let mut t = EXEC_GEOMETRIC_FROM;
    while out.len() < count {
        let h = ratio.floor_at(t);
        let h = as_u64(&h).expect("executable gap does not fit in u64");
        out.push(h);
        t += 1;
    }
    out
}

/// Distinct `H_exec` gaps strictly below `n`, descending.
///
/// The last gap is 1 when `n > 1`. `max_terms` caps how many closed-form
/// terms `h_t` (`t >= 12`) are generated. Generation also stops once
/// `h_t >= n`, so a huge `n` does not need terms past the length.
pub fn gaps_below(n: usize, max_terms: usize) -> Vec<u64> {
    if n <= 1 {
        return Vec::new();
    }
    let n64 = u64::try_from(n).expect("n fits in u64 on this target");
    let mut gaps = Vec::new();
    for g in EXEC_PREFIX {
        if g < n64 {
            gaps.push(g);
        }
    }
    if max_terms > 0 {
        let mut ratio = Ratio::new();
        let mut t = EXEC_GEOMETRIC_FROM;
        for _ in 0..max_terms {
            let h = ratio.floor_at(t);
            match as_u64(&h) {
                Some(h) if h >= 1 && h < n64 => gaps.push(h),
                _ => break,
            }
            t = t.saturating_add(1);
        }
    }
    gaps.sort_unstable_by(|a, b| b.cmp(a));
    gaps.dedup();
    gaps
}

/// Shellsort `a` with every distinct executable gap strictly below `a.len()`.
///
/// Returns `(comparisons, moves)`. An `h`-pass insertion-sorts each residue
/// class modulo `h`. Every key comparison and every array write is counted.
/// Equal keys do not swap: the scan stops when `a[j - h] <= tmp`.
pub fn shellsort(a: &mut [u64]) -> (u64, u64) {
    let n = a.len();
    let gaps = gaps_below(n, GEOMETRIC_CAP);
    let mut comparisons = 0u64;
    let mut moves = 0u64;
    for &gap in &gaps {
        let h = gap as usize;
        debug_assert!(h > 0 && h < n);
        for i in h..n {
            let tmp = a[i];
            let mut j = i;
            while j >= h {
                comparisons += 1;
                if a[j - h] <= tmp {
                    break;
                }
                a[j] = a[j - h];
                moves += 1;
                j -= h;
            }
            a[j] = tmp;
            moves += 1;
        }
    }
    (comparisons, moves)
}
