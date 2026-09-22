use crate::address::{Level, TranslationGranule};
use crate::descriptor::DescriptorFormat;
use crate::regime::{
    InterpretedLeafFields, InterpretedTableFields, SemanticRegime, TranslationRegime,
};

pub type SemanticLeafAttrs<F, R> = <R as SemanticRegime<F>>::Leaf;
pub type SemanticTableAttrs<F, R> = <R as SemanticRegime<F>>::Table;

pub trait AttributeCodec<F, R, G, Cfg>
where
    F: DescriptorFormat,
    R: TranslationRegime<F, G> + SemanticRegime<F>,
    G: TranslationGranule,
{
    type Error;
    fn encode_leaf(
        config: &Cfg,
        level: Level,
        attrs: R::Leaf,
    ) -> Result<InterpretedLeafFields<F, R, G>, Self::Error>;
    fn encode_table(
        config: &Cfg,
        level: Level,
        attrs: R::Table,
    ) -> Result<InterpretedTableFields<F, R, G>, Self::Error>;
    fn decode_leaf(
        config: &Cfg,
        level: Level,
        raw: InterpretedLeafFields<F, R, G>,
    ) -> Result<R::Leaf, Self::Error>;
    fn decode_table(
        config: &Cfg,
        level: Level,
        raw: InterpretedTableFields<F, R, G>,
    ) -> Result<R::Table, Self::Error>;
}
