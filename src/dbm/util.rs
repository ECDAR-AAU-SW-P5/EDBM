use crate::util::constraints::{
    raw_constants::{LS_INFINITY, ZERO},
    ClockIndex, RawInequality,
};

use super::{Valid, DBM};

pub fn worst_value(
    dbm1: &DBM<Valid>,
    dbm2: &DBM<Valid>,
    i: ClockIndex,
    j: ClockIndex,
) -> RawInequality {
    debug_assert_eq!(dbm1.dim, dbm2.dim);
    let dim = dbm1.dim;
    let dbm_ij = dbm2[(i, j)].as_weak();

    for k in 0..dim {
        if k != i && k != j {
            if dbm2[(k, j)] != LS_INFINITY && dbm1[(i, k)] != LS_INFINITY {
                let v = dbm_ij - (dbm1[(i, k)].as_weak() + dbm2[(k, j)].as_weak());
                if v >= ZERO {
                    return LS_INFINITY;
                }
            }

            if dbm2[(i, k)] != LS_INFINITY && dbm1[(k, j)] != LS_INFINITY {
                let v = dbm_ij - (dbm1[(k, j)].as_weak() + dbm2[(i, k)].as_weak());
                if v >= ZERO {
                    return LS_INFINITY;
                }
            }
        }
    }

    dbm_ij - dbm1[(i, j)]
}
