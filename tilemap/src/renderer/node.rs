use crate::math::side::Side;
use crate::tilemap::border::WallId;
use crate::tilemap::tilemap2d::Tilemap2d;
use map_macro::set;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

/// Calculates how many sides each [`wall style`](crate::renderer::style::wall::WallStyle) has at a node.
pub fn calculate_sides_per_style(
    tilemap: &Tilemap2d,
    node_index: usize,
) -> HashMap<WallId, HashSet<Side>> {
    let mut wall_styles = HashMap::new();

    for side in Side::iterator() {
        let wall_style = tilemap
            .get_border_at_node(node_index, *side)
            .get_wall_style();

        if let Some(id) = wall_style {
            match wall_styles.entry(id) {
                Entry::Vacant(e) => {
                    e.insert(set![*side]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(*side);
                }
            }
        }
    }

    wall_styles
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::side::Side::*;
    use crate::math::size2d::Size2d;
    use crate::tilemap::border::Border::Wall;
    use crate::tilemap::tile::Tile::Empty;
    use map_macro::map;

    #[test]
    fn test_get_border_at_node() {
        let size = Size2d::new(3, 3);
        let mut tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.set_border(0, Front, Wall(1));
        tilemap.set_border(0, Right, Wall(1));
        tilemap.set_border(4, Back, Wall(2));
        tilemap.set_border(4, Left, Wall(3));

        assert_eq!(
            calculate_sides_per_style(&tilemap, 5),
            map! {
            1 => set![Back, Left],
            2 => set![Right],
            3 => set![Front],
            }
        );
    }
}
