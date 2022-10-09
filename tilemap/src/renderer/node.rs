use crate::math::side::Side;
use crate::tilemap::border::WallId;
use crate::tilemap::tilemap2d::Tilemap2d;
use map_macro::set;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

/// Calculates the [`wall styles`](crate::renderer::style::wall::WallStyle) with the highest count.
fn get_top_styles(input: HashMap<WallId, HashSet<Side>>) -> Vec<(WallId, HashSet<Side>)> {
    let mut max_count = 0;
    let mut top_styles = Vec::new();

    for entry in input {
        let count = entry.1.len();

        if count > max_count {
            max_count = count;
            top_styles.clear();
            top_styles.push(entry);
        } else if count > 0 && count == max_count {
            top_styles.push(entry);
        }
    }

    top_styles
}

/// Calculates how many sides each [`wall style`](crate::renderer::style::wall::WallStyle) has at a node.
fn calculate_sides_per_style(
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
    fn test_get_top_styles_empty() {
        assert_eq!(get_top_styles(HashMap::new()), Vec::new());
    }

    #[test]
    fn test_get_top_styles_one() {
        assert_eq!(
            get_top_styles(map! {
            1 => set![Back, Left],
            2 => set![Right],
            }),
            vec![(1, set![Back, Left])]
        );
    }

    #[test]
    fn test_get_top_styles_two() {
        let top_styles = get_top_styles(map! {
        1 => set![Back, Left],
        2 => set![Right, Front],
        });

        assert_eq!(2, top_styles.len());
        assert!(top_styles.contains(&(1, set![Back, Left])));
        assert!(top_styles.contains(&(2, set![Right, Front])));
    }

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
