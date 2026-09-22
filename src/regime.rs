use crate::address::TranslationGranule;
use crate::descriptor::{DescriptorFormat, DescriptorLayout};

pub trait TranslationRegime<F, G>: Copy + 'static
where
    F: DescriptorFormat,
    G: TranslationGranule,
{
    type Layout: DescriptorLayout<G, Format = F>;
}

pub trait SemanticRegime<F>: Copy + 'static
where
    F: DescriptorFormat,
{
    type Leaf: Copy;
    type Table: Copy;
    type Codec;
}

pub type RegimeLayout<F, R, G> = <R as TranslationRegime<F, G>>::Layout;
pub type InterpretedLeafFields<F, R, G> =
    <RegimeLayout<F, R, G> as DescriptorLayout<G>>::LeafFields;
pub type InterpretedTableFields<F, R, G> =
    <RegimeLayout<F, R, G> as DescriptorLayout<G>>::TableFields;
