use bevy_procedural_tilemaps::prelude::*;
use crate::map::assets::SpawnableAsset;

pub(crate) struct TerrainModelBuilder {
    pub(crate) models: ModelCollection<Cartesian3D>,
    pub(crate) assets: Vec<Vec<SpawnableAsset>>,
}

impl TerrainModelBuilder {
    pub(crate) fn new() -> Self {
        Self {
            models: ModelCollection::new(),
            assets: Vec::new(),
        }
    }


    pub(crate) fn create_model<T>(
        &mut self,
        template: T,
        assets: Vec<SpawnableAsset>,
    ) -> &mut Model<Cartesian3D>
    where
        T: Into<ModelTemplate<Cartesian3D>>,
    {
        let model_ref = self.models.create(template);
        self.assets.push(assets);
        model_ref
    }

    pub(crate) fn into_parts(self) -> (Vec<Vec<SpawnableAsset>>, ModelCollection<Cartesian3D>) {
        (self.assets, self.models)
    }
}