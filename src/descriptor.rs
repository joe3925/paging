use crate::address::{Level, PhysAddr, TranslationGranule};
use crate::table::{TableAddr, TableTransition};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DescriptorKind {
    Block,
    Page,
    Table,
    Invalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NextTableDescriptor<G: TranslationGranule> {
    pub address: TableAddr<G>,
    pub level: Level,
    pub stride_count: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DescriptorError {
    InvalidLeafLevel {
        level: Level,
    },
    InvalidTableTransition {
        parent_level: Level,
        child_level: Level,
        stride_count: u8,
    },
    ReservedFieldSet {
        bit: u8,
    },
    InvalidNtBbmCombination {
        level: Level,
    },
    InvalidReservedBitState,
}

pub unsafe trait DescriptorFormat: Copy + Sized + 'static {
    type Raw: Copy + Eq;
    const DESCRIPTOR_BYTES: usize;
    const DESCRIPTOR_SHIFT: u8;
    const MAX_INPUT_ADDRESS_BITS: u8;
    const OUTPUT_ADDRESS_BITS: u8;
    const SUPPORTED_OUTPUT_ADDRESS_BITS: &'static [u8];
    const EXTENDED_LOWEST_ROOT_LEVEL: Level;
    const FINAL_LEVEL: Level;
    fn invalid() -> Self::Raw;
    unsafe fn read_descriptor(ptr: *const Self::Raw) -> Self::Raw;
    unsafe fn write_descriptor(ptr: *mut Self::Raw, raw: Self::Raw);
}

pub unsafe trait SupportsLiveDescriptorIo: DescriptorFormat {}

pub unsafe trait DescriptorLayout<G>: Copy + 'static
where
    G: TranslationGranule,
{
    type Format: DescriptorFormat;
    type LeafFields: Copy;
    type TableFields: Copy;
    type Error;
    const ADDRESS_FIELD_MASK: u128;
    fn supports_leaf_level(level: Level) -> bool;
    fn kind(raw: <Self::Format as DescriptorFormat>::Raw, level: Level) -> DescriptorKind;
    fn decode_leaf_fields(
        raw: <Self::Format as DescriptorFormat>::Raw,
        level: Level,
    ) -> Self::LeafFields;
    fn decode_table_fields(
        raw: <Self::Format as DescriptorFormat>::Raw,
        level: Level,
    ) -> Self::TableFields;
    fn leaf_descriptor(
        output: PhysAddr,
        level: Level,
        fields: Self::LeafFields,
    ) -> Result<<Self::Format as DescriptorFormat>::Raw, Self::Error>;
    fn table_descriptor(
        table: TableAddr<G>,
        transition: TableTransition<Self::Format, G>,
        fields: Self::TableFields,
    ) -> Result<<Self::Format as DescriptorFormat>::Raw, Self::Error>;
    fn output_address(raw: <Self::Format as DescriptorFormat>::Raw, level: Level) -> PhysAddr;
    fn table_address(raw: <Self::Format as DescriptorFormat>::Raw, level: Level) -> TableAddr<G>;
    fn next_table(
        raw: <Self::Format as DescriptorFormat>::Raw,
        level: Level,
    ) -> Option<NextTableDescriptor<G>> {
        level
            .is_before(Self::Format::FINAL_LEVEL)
            .then(|| NextTableDescriptor {
                address: Self::table_address(raw, level),
                level: level.next(),
                stride_count: 1,
            })
    }
    fn supports_table_transition(transition: TableTransition<Self::Format, G>) -> bool {
        transition.level_step() == 1 && transition.child().stride_count().raw() == 1
    }
}
