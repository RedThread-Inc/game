use bevy_procedural_tilemaps::prelude::*;

pub(crate) struct TerrainSockets {
    pub(crate) dirt: DirtLayerSockets,
    pub(crate) void: Socket,
    pub(crate) grass: GrassLayerSockets,
    pub(crate) yellow_grass: YellowGrassLayerSockets,
    pub(crate) water: WaterLayerSockets,
    pub(crate) props: PropsLayerSockets,
}

pub(crate) fn create_sockets(socket_collection: &mut SocketCollection) -> TerrainSockets {
    let mut new_socket = || -> Socket { socket_collection.create() };

    let sockets = TerrainSockets {
        dirt: DirtLayerSockets {
            layer_up: new_socket(),
            material: new_socket(),
            layer_down: new_socket(),
        },
        void: new_socket(),
        grass: GrassLayerSockets {
            layer_up: new_socket(),
            material: new_socket(),
            layer_down: new_socket(),
            void_and_grass: new_socket(),
            grass_and_void: new_socket(),
            grass_fill_up: new_socket(),
        },
        yellow_grass: YellowGrassLayerSockets {
            layer_up: new_socket(),
            layer_down: new_socket(),
            yellow_grass_fill_down: new_socket(),
        },
        water: WaterLayerSockets {
            layer_up: new_socket(),
            layer_down: new_socket(),
            material: new_socket(),
            void_and_water: new_socket(),
            water_and_void: new_socket(),
            ground_up: new_socket(),
        },
        props: PropsLayerSockets {
            layer_up: new_socket(),
            layer_down: new_socket(),
            props_down: new_socket(),
            big_tree_1_base: new_socket(),
            big_tree_2_base: new_socket(),
        },
    };
    sockets
}

pub(crate) struct DirtLayerSockets {
    pub(crate) layer_up: Socket,
    pub(crate) layer_down: Socket,
    pub(crate) material: Socket,
}

pub(crate) struct GrassLayerSockets {
    pub(crate) layer_up: Socket,
    pub(crate) layer_down: Socket,
    pub(crate) material: Socket,
    pub(crate) void_and_grass: Socket,
    pub(crate) grass_and_void: Socket,
    pub(crate) grass_fill_up: Socket,
}

pub(crate) struct YellowGrassLayerSockets {
    pub(crate) layer_up: Socket,
    pub(crate) layer_down: Socket,
    pub(crate) yellow_grass_fill_down: Socket,
}

pub(crate) struct WaterLayerSockets {
    pub(crate) layer_up: Socket,
    pub(crate) layer_down: Socket,
    pub(crate) material: Socket,
    pub(crate) void_and_water: Socket,
    pub(crate) water_and_void: Socket,
    pub(crate) ground_up: Socket,
}

pub(crate) struct PropsLayerSockets {
    pub(crate) layer_up: Socket,
    pub(crate) layer_down: Socket,
    pub(crate) props_down: Socket,
    pub(crate) big_tree_1_base: Socket,
    pub(crate) big_tree_2_base: Socket,
}