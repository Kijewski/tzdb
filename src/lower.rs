use core::fmt;

use phf_shared::{FmtConst, PhfBorrow, PhfHash};

#[allow(unused)]
pub(crate) const FULL_TO_LOWER_MAX_LEN: usize = 31;

pub(crate) type Slice = [u8; 32];

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct Lower(pub(crate) Slice);

#[inline]
pub(crate) fn full_to_lower(s: &[u8]) -> Lower {
    // The caller ensures that s.len() <= FULL_TO_LOWER_MAX_LEN.
    let mut lower = [0u64; 4];
    {
        let bytes = unsafe { &mut *lower.as_mut_ptr().cast::<Slice>() };
        bytes[..s.len()].copy_from_slice(s);
    }
    for c in &mut lower {
        *c |= 0x2020_2020_2020_2020;
    }

    let mut bytes = unsafe { *lower.as_ptr().cast::<Slice>() };
    bytes[s.len()] = 0;
    Lower(bytes)
}

impl PhfHash for Lower {
    #[inline]
    fn phf_hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.phf_hash(state);
    }
}

impl FmtConst for Lower {
    fn fmt_const(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lower(*b\"")?;
        for c in self.0 {
            match c {
                0 => {
                    write!(f, "\\0")?;
                    continue;
                },
                b'\\' | b'"' => (),
                0x20..=0x7e => {
                    write!(f, "{}", c as char)?;
                    continue;
                },
                _ => (),
            }
            write!(f, "\\x{:02x}", c)?;
        }
        write!(f, "\")")
    }
}

impl PhfBorrow<Self> for Lower {
    #[inline]
    fn borrow(&self) -> &Self {
        self
    }
}
