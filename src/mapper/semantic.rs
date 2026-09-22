use crate::address::{Level, TranslationGranule};
use crate::attrs::{AttributeCodec, SemanticLeafAttrs, SemanticTableAttrs};
use crate::descriptor::{DescriptorError, DescriptorFormat, DescriptorLayout};
use crate::regime::{
    InterpretedLeafFields, InterpretedTableFields, RegimeLayout, SemanticRegime, TranslationRegime,
};
use crate::table::{TableAccessMut, TableFrameProvider};
use crate::translation::walk::{WalkInputAddr, WalkOutputAddr};
use crate::translation::{WalkLeaf, WalkTable};

use super::{MapLeafOutcome, Mapper, MapperError, MapperMode, Mapping};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticMapperError<AttributeError, AccessError, FrameError> {
    Attribute(AttributeError),
    Mapper(MapperError<AccessError, FrameError>),
}

impl<F, R, G, A, P, M> Mapper<F, R, G, A, P, M>
where
    F: DescriptorFormat,
    R: TranslationRegime<F, G> + SemanticRegime<F>,
    G: TranslationGranule,
    A: TableAccessMut<F, G>,
    P: TableFrameProvider<G>,
    M: MapperMode<F, R, G>,
    InterpretedLeafFields<F, R, G>: Copy,
    RegimeLayout<F, R, G>: DescriptorLayout<G, Format = F, Error = DescriptorError>,
{
    pub fn map_semantic_leaf<Cfg>(
        &mut self,
        config: &Cfg,
        input: WalkInputAddr,
        output: WalkOutputAddr,
        level: Level,
        leaf_attrs: SemanticLeafAttrs<F, R>,
        table_attrs: SemanticTableAttrs<F, R>,
    ) -> Result<
        MapLeafOutcome,
        SemanticMapperError<<R::Codec as AttributeCodec<F, R, G, Cfg>>::Error, A::Error, P::Error>,
    >
    where
        R::Codec: AttributeCodec<F, R, G, Cfg>,
    {
        let leaf =
            <R::Codec as AttributeCodec<F, R, G, Cfg>>::encode_leaf(config, level, leaf_attrs)
                .map_err(SemanticMapperError::Attribute)?;
        let table =
            <R::Codec as AttributeCodec<F, R, G, Cfg>>::encode_table(config, level, table_attrs)
                .map_err(SemanticMapperError::Attribute)?;
        self.map_leaf(input, output, level, leaf, table)
            .map_err(SemanticMapperError::Mapper)
    }
}

impl<F, R, G> Mapping<F, R, G>
where
    F: DescriptorFormat,
    R: TranslationRegime<F, G> + SemanticRegime<F>,
    G: TranslationGranule,
{
    pub fn semantic_attrs<Cfg>(
        &self,
        config: &Cfg,
    ) -> Result<SemanticLeafAttrs<F, R>, <R::Codec as AttributeCodec<F, R, G, Cfg>>::Error>
    where
        R::Codec: AttributeCodec<F, R, G, Cfg>,
    {
        <R::Codec as AttributeCodec<F, R, G, Cfg>>::decode_leaf(
            config,
            self.level(),
            *self.fields(),
        )
    }
}

impl<F, R, G> WalkLeaf<F, R, G>
where
    F: DescriptorFormat,
    R: TranslationRegime<F, G> + SemanticRegime<F>,
    G: TranslationGranule,
{
    pub fn semantic_attrs<Cfg>(
        &self,
        config: &Cfg,
    ) -> Result<SemanticLeafAttrs<F, R>, <R::Codec as AttributeCodec<F, R, G, Cfg>>::Error>
    where
        R::Codec: AttributeCodec<F, R, G, Cfg>,
    {
        <R::Codec as AttributeCodec<F, R, G, Cfg>>::decode_leaf(
            config,
            self.info().level(),
            *self.fields(),
        )
    }
}

impl<F, R, G> WalkTable<F, R, G>
where
    F: DescriptorFormat,
    R: TranslationRegime<F, G> + SemanticRegime<F>,
    G: TranslationGranule,
{
    pub fn semantic_attrs<Cfg>(
        &self,
        config: &Cfg,
    ) -> Result<SemanticTableAttrs<F, R>, <R::Codec as AttributeCodec<F, R, G, Cfg>>::Error>
    where
        R::Codec: AttributeCodec<F, R, G, Cfg>,
    {
        <R::Codec as AttributeCodec<F, R, G, Cfg>>::decode_table(
            config,
            self.info().level(),
            *self.fields(),
        )
    }
}

pub fn decode_semantic_leaf<F, R, G, Cfg>(
    config: &Cfg,
    level: Level,
    raw: InterpretedLeafFields<F, R, G>,
) -> Result<SemanticLeafAttrs<F, R>, <R::Codec as AttributeCodec<F, R, G, Cfg>>::Error>
where
    F: DescriptorFormat,
    R: TranslationRegime<F, G> + SemanticRegime<F>,
    G: TranslationGranule,
    R::Codec: AttributeCodec<F, R, G, Cfg>,
{
    <R::Codec as AttributeCodec<F, R, G, Cfg>>::decode_leaf(config, level, raw)
}

pub fn decode_semantic_table<F, R, G, Cfg>(
    config: &Cfg,
    level: Level,
    raw: InterpretedTableFields<F, R, G>,
) -> Result<SemanticTableAttrs<F, R>, <R::Codec as AttributeCodec<F, R, G, Cfg>>::Error>
where
    F: DescriptorFormat,
    R: TranslationRegime<F, G> + SemanticRegime<F>,
    G: TranslationGranule,
    R::Codec: AttributeCodec<F, R, G, Cfg>,
{
    <R::Codec as AttributeCodec<F, R, G, Cfg>>::decode_table(config, level, raw)
}
