use std::io::{Read, Write};
use std::collections::HashMap;
use bincode::{self, serde};
use field::Field;
use spatial::Spatial;
use flags::Flags;
use location::Location;
use components;
use stats;

pub const GAME_VERSION: &'static str = "0.1.0";

Ecs! {
    desc: components::Desc,
    map_memory: components::MapMemory,
    health: components::Health,
    brain: components::Brain,
    item: components::Item,
    composite_stats: components::CompositeStats,
    stats: stats::Stats,
}

/// Toplevel game state object.
#[derive(Serialize, Deserialize)]
pub struct World {
    /// Game version. Not mutable in the slightest, but the simplest way to
    /// get versioned save files is to just drop it here.
    version: String,
    /// Entity component system.
    pub ecs: Ecs,
    /// Terrain data.
    pub terrain: Field<u8>,
    /// Optional portals between map zones.
    pub portals: HashMap<Location, Location>,
    /// Spatial index for game entities.
    pub spatial: Spatial,
    /// Global gamestate flags.
    pub flags: Flags,
}

impl<'a> World {
    pub fn new() -> World {
        World {
            version: GAME_VERSION.to_string(),
            ecs: Ecs::new(),
            terrain: Field::new(0),
            portals: HashMap::new(),
            spatial: Spatial::new(),
            flags: Flags::new(1),
        }
    }

    pub fn load<R: Read>(reader: &mut R) -> serde::DeserializeResult<World> {
        let ret: serde::DeserializeResult<World> =
            serde::deserialize_from(reader, bincode::SizeLimit::Infinite);
        if let &Ok(ref x) = &ret {
            if &x.version != GAME_VERSION {
                panic!("Save game version {} does not match current version \
                        {}",
                       x.version,
                       GAME_VERSION);
            }
        }
        ret
    }

    pub fn save<W: Write>(&self, writer: &mut W) -> serde::SerializeResult<()> {
        serde::serialize_into(writer, self, bincode::SizeLimit::Infinite)
    }
}
