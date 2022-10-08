pub type FloorId = usize;
pub type SolidId = usize;

/// A space in the [`tilemap`](crate::tilemap::tilemap2d::Tilemap2d).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    /// Empty like a hole to a lower level or the empty sky around a flying island.
    Empty,
    /// The ground outside or the floor of a building.
    Floor(FloorId),
    /// Full of a solid material like earth or stone. E.g. underground
    Solid(SolidId),
}
