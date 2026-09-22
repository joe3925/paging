#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Level(i8);

impl Level {
    pub const NEG2: Self = Self(-2);
    pub const NEG1: Self = Self(-1);
    pub const L0: Self = Self(0);
    pub const L1: Self = Self(1);
    pub const L2: Self = Self(2);
    pub const L3: Self = Self(3);

    pub const fn new(value: i8) -> Self {
        Self(value)
    }
    pub const fn as_i8(self) -> i8 {
        self.0
    }
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
    pub const fn previous(self) -> Self {
        Self(self.0 - 1)
    }
    pub const fn is_l0(self) -> bool {
        self.0 == 0
    }
    pub const fn is_l1(self) -> bool {
        self.0 == 1
    }
    pub const fn is_l2(self) -> bool {
        self.0 == 2
    }
    pub const fn is_l3(self) -> bool {
        self.0 == 3
    }
    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }
    pub const fn is_before(self, other: Self) -> bool {
        self.0 < other.0
    }
    pub const fn is_after(self, other: Self) -> bool {
        self.0 > other.0
    }
    pub const fn distance_from(self, root: Self) -> Option<u8> {
        if self.is_before(root) {
            None
        } else {
            Some((self.0 as i16 - root.0 as i16) as u8)
        }
    }
    pub const fn is_between_inclusive(self, start: Self, end: Self) -> bool {
        self.0 >= start.0 && self.0 <= end.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GranuleError {
    AddressNotAligned,
}

pub unsafe trait TranslationGranule: Copy + 'static {
    const SHIFT: u8;
    const SIZE: u64 = 1u64 << Self::SHIFT;
    const MASK: u64 = Self::SIZE - 1;

    fn shift() -> u8 {
        Self::SHIFT
    }
    fn size() -> u64 {
        Self::SIZE
    }
    fn mask() -> u64 {
        Self::MASK
    }
    fn is_page_aligned(value: u64) -> bool {
        value & Self::MASK == 0
    }
    fn align_down(value: u64) -> u64 {
        value & !Self::MASK
    }
    fn align_up(value: u64) -> Option<u64> {
        value
            .checked_add(Self::MASK)
            .map(|value| value & !Self::MASK)
    }
    fn validate_page_alignment(value: u64) -> Result<(), GranuleError> {
        if Self::is_page_aligned(value) {
            Ok(())
        } else {
            Err(GranuleError::AddressNotAligned)
        }
    }
}
