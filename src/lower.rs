use core::fmt;

use byte_slice_cast::AsMutByteSlice;
use phf_shared::{FmtConst, PhfBorrow, PhfHash};

#[allow(unused)]
pub(crate) const FULL_TO_LOWER_MAX_LEN: usize = 31;

pub(crate) type Slice = [u64; 4];

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct Lower(pub(crate) Slice);

#[inline]
pub(crate) fn full_to_lower(s: &[u8]) -> Lower {
    // The caller ensures that s.len() <= FULL_TO_LOWER_MAX_LEN.
    let mut lower = Slice::default();
    lower.as_mut_byte_slice()[..s.len()].copy_from_slice(s);
    for c in &mut lower {
        *c |= 0x2020_2020_2020_2020;
    }
    lower.as_mut_byte_slice()[s.len()] = 0;
    Lower(lower)
}

impl PhfHash for Lower {
    #[inline]
    fn phf_hash<H: core::hash::Hasher>(&self, state: &mut H) {
        for c in self.0 {
            c.phf_hash(state);
        }
    }
}

impl FmtConst for Lower {
    fn fmt_const(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lower({:#x?})", self.0)
    }
}

impl PhfBorrow<Self> for Lower {
    #[inline]
    fn borrow(&self) -> &Self {
        self
    }
}
