use byte_slice_cast::AsMutByteSlice;
use phf_shared::{FmtConst, PhfBorrow, PhfHash};

pub(crate) type Slice = [usize; ((32 * 8) / usize::BITS) as usize];

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct Lower(pub(crate) Slice);

#[inline]
pub(crate) fn full_to_lower(s: &[u8]) -> Lower {
    // The caller ensures that s.len() <= 32.
    let mut lower = Slice::default();
    lower.as_mut_byte_slice()[..s.len()].copy_from_slice(s);
    for c in &mut lower {
        *c |= 0x2020_2020_2020_2020_2020_2020_2020_2020_u128 as usize;
    }
    Lower(lower)
}

impl PhfHash for Lower {
    #[inline]
    fn phf_hash<H: core::hash::Hasher>(&self, state: &mut H) {
        for c in self.0 {
            #[cfg(target_pointer_width = "8")]
            (c as u8).phf_hash(state);
            #[cfg(target_pointer_width = "16")]
            (c as u16).phf_hash(state);
            #[cfg(target_pointer_width = "32")]
            (c as u32).phf_hash(state);
            #[cfg(target_pointer_width = "64")]
            (c as u64).phf_hash(state);
            #[cfg(target_pointer_width = "128")]
            (c as u128).phf_hash(state);

            #[cfg(not(any(
                target_pointer_width = "8",
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            )))]
            compile_error!(
                "Unsupported target_pointer_width. \
                Please file a bug report to <https://github.com/Kijewski/tzdb/>."
            );
        }
    }
}

impl FmtConst for Lower {
    fn fmt_const(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lower({:#x?})", self.0)
    }
}

impl PhfBorrow<Self> for Lower {
    #[inline]
    fn borrow(&self) -> &Self {
        self
    }
}
