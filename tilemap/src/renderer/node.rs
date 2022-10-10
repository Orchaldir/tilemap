use crate::math::side::Side;
use crate::tilemap::border::WallId;
use crate::tilemap::tilemap2d::Tilemap2d;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Calculates the dominant [`wall style`](crate::renderer::style::wall::WallStyle) at the node.
pub fn calculate_dominant_wall_style(tilemap: &Tilemap2d, index: usize) -> Option<WallId> {
    let sides_per_style = calculate_sides_per_style(tilemap, index);
    let is_intersection = sides_per_style.len() > 1;
    let top_styles = get_top_styles(sides_per_style);

    match top_styles.len() {
        1 => handle_one_style(&top_styles[0], is_intersection),
        n if n > 1 => top_styles.iter().map(|s| s.0).min(),
        _ => None,
    }
}

fn handle_one_style(top_style: &(WallId, Vec<Side>), is_intersection: bool) -> Option<WallId> {
    if is_inner_node(top_style, is_intersection) {
        return None;
    }

    Some(top_style.0)
}

fn is_inner_node(top_style: &(WallId, Vec<Side>), is_intersection: bool) -> bool {
    !is_intersection && top_style.1.len() == 2 && is_straight(top_style)
}

/// Does the wall style form a straight line at a node?
fn is_straight(style: &(WallId, Vec<Side>)) -> bool {
    let side0 = style.1[0];
    let side1 = style.1[1];

    side0.is_straight(side1)
}

/// Calculates the [`wall styles`](crate::renderer::style::wall::WallStyle) with the highest count.
fn get_top_styles(input: HashMap<WallId, Vec<Side>>) -> Vec<(WallId, Vec<Side>)> {
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
fn calculate_sides_per_style(tilemap: &Tilemap2d, node_index: usize) -> HashMap<WallId, Vec<Side>> {
    let mut wall_styles = HashMap::new();

    for side in Side::iterator() {
        let wall_style = tilemap
            .get_border_at_node(node_index, *side)
            .get_wall_style();

        if let Some(id) = wall_style {
            match wall_styles.entry(id) {
                Entry::Vacant(e) => {
                    e.insert(vec![*side]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(*side);
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
    fn test_wall_style_twice_at_node_dominates() {
        let size = Size2d::new(2, 2);
        let mut tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.set_border(2, Back, Wall(2));
        tilemap.set_border(3, Back, Wall(2));
        tilemap.set_border(3, Left, Wall(3));

        assert_eq!(calculate_dominant_wall_style(&tilemap, 0), None);
        assert_eq!(calculate_dominant_wall_style(&tilemap, 1), None);
        assert_eq!(calculate_dominant_wall_style(&tilemap, 2), None);
        assert_eq!(calculate_dominant_wall_style(&tilemap, 3), Some(2));
        assert_eq!(calculate_dominant_wall_style(&tilemap, 4), Some(2));
        assert_eq!(calculate_dominant_wall_style(&tilemap, 5), Some(2));
        assert_eq!(calculate_dominant_wall_style(&tilemap, 6), None);
        assert_eq!(calculate_dominant_wall_style(&tilemap, 7), Some(3));
        assert_eq!(calculate_dominant_wall_style(&tilemap, 8), None);
    }

    #[test]
    fn test_line_with_same_wall_style() {
        let size = Size2d::new(2, 2);
        let mut tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.set_border(2, Back, Wall(2));
        tilemap.set_border(3, Back, Wall(2));

        assert_eq!(calculate_dominant_wall_style(&tilemap, 3), Some(2));
        assert_eq!(calculate_dominant_wall_style(&tilemap, 4), None);
        assert_eq!(calculate_dominant_wall_style(&tilemap, 5), Some(2));
    }

    #[test]
    fn test_four_different_wall_styles_at_node() {
        let size = Size2d::new(2, 2);
        let mut tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.set_border(1, Left, Wall(13));
        tilemap.set_border(2, Back, Wall(12));
        tilemap.set_border(3, Back, Wall(11));
        tilemap.set_border(3, Left, Wall(10));

        assert_eq!(calculate_dominant_wall_style(&tilemap, 0), None);
        assert_eq!(calculate_dominant_wall_style(&tilemap, 1), Some(13));
        assert_eq!(calculate_dominant_wall_style(&tilemap, 2), None);
        assert_eq!(calculate_dominant_wall_style(&tilemap, 3), Some(12));
        assert_eq!(calculate_dominant_wall_style(&tilemap, 4), Some(10));
        assert_eq!(calculate_dominant_wall_style(&tilemap, 5), Some(11));
        assert_eq!(calculate_dominant_wall_style(&tilemap, 6), None);
        assert_eq!(calculate_dominant_wall_style(&tilemap, 7), Some(10));
        assert_eq!(calculate_dominant_wall_style(&tilemap, 8), None);
    }

    #[test]
    fn test_get_top_styles_empty() {
        assert_eq!(get_top_styles(HashMap::new()), Vec::new());
    }

    #[test]
    fn test_get_top_styles_one() {
        assert_eq!(
            get_top_styles(map! {
            1 => vec![Back, Left],
            2 => vec![Right],
            }),
            vec![(1, vec![Back, Left])]
        );
    }

    #[test]
    fn test_get_top_styles_two() {
        let top_styles = get_top_styles(map! {
        1 => vec![Back, Left],
        2 => vec![Right, Front],
        });

        assert_eq!(2, top_styles.len());
        assert!(top_styles.contains(&(1, vec![Back, Left])));
        assert!(top_styles.contains(&(2, vec![Right, Front])));
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
            1 => vec![Back, Left],
            2 => vec![Right],
            3 => vec![Front],
            }
        );
    }
}
