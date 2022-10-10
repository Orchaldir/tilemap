use crate::math::side::Side;
use crate::renderer::style::node::NodeStyle;
use crate::renderer::style::wall::WallStyle;
use crate::tilemap::border::WallId;
use crate::tilemap::node::get_nodes_size;
use crate::tilemap::tilemap2d::Tilemap2d;
use crate::utils::resource::ResourceManager;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Nodes are the 4 corners of each [`tile`](crate::tilemap::tile::Tile)
/// and the start & end point of each [`border`](crate::tilemap::border::Border).
/// How a node is rendered is indirectly determined by the 4 borders surrounding each node
/// and their [`wall styles`](crate::renderer::style::wall::WallStyle).
#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    NoNode,
    OuterNode(&'a NodeStyle),
}

impl<'a> Node<'a> {
    pub fn calculate_half(&self) -> u32 {
        match self {
            Node::NoNode => 0,
            Node::OuterNode(style) => style.get_half(),
        }
    }
}

/// Calculates the [`node`](Node) at each node.
pub fn calculate_node_styles<'a>(
    node_styles: &'a ResourceManager<NodeStyle>,
    wall_styles: &'a ResourceManager<WallStyle>,
    tilemap: &'a Tilemap2d,
) -> Vec<Node<'a>> {
    calculate_dominant_wall_styles(tilemap)
        .iter()
        .map(|o| match o {
            None => Node::NoNode,
            Some(wall_id) => {
                let node_id = wall_styles.get(*wall_id).get_node_style();
                Node::OuterNode(node_styles.get(node_id))
            }
        })
        .collect()
}

/// Calculates the dominant [`wall style`](crate::renderer::style::wall::WallStyle) at each node.
fn calculate_dominant_wall_styles(tilemap: &Tilemap2d) -> Vec<Option<WallId>> {
    let size = get_nodes_size(tilemap.get_size());
    let mut node_styles = Vec::with_capacity(size.count());
    let mut index = 0;

    for _y in 0..size.height() {
        for _x in 0..size.width() {
            node_styles.push(calculate_dominant_wall_style(tilemap, index));
            index += 1;
        }
    }

    node_styles
}

/// Calculates the dominant [`wall style`](crate::renderer::style::wall::WallStyle) at the node.
fn calculate_dominant_wall_style(tilemap: &Tilemap2d, index: usize) -> Option<WallId> {
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

        #[rustfmt::skip]
        assert_eq!(
            calculate_dominant_wall_styles(&tilemap),
            vec![
                None, None, None,
                Some(2), Some(2), Some(2),
                None, Some(3), None
            ]
        );
    }

    #[test]
    fn test_line_with_same_wall_style() {
        let size = Size2d::new(2, 2);
        let mut tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.set_border(2, Back, Wall(2));
        tilemap.set_border(3, Back, Wall(2));

        #[rustfmt::skip]
        assert_eq!(
            calculate_dominant_wall_styles(&tilemap),
            vec![
                None, None, None,
                Some(2), None, Some(2),
                None, None, None
            ]
        );
    }

    #[test]
    fn test_four_different_wall_styles_at_node() {
        let size = Size2d::new(2, 2);
        let mut tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.set_border(1, Left, Wall(13));
        tilemap.set_border(2, Back, Wall(12));
        tilemap.set_border(3, Back, Wall(11));
        tilemap.set_border(3, Left, Wall(10));

        #[rustfmt::skip]
        assert_eq!(
            calculate_dominant_wall_styles(&tilemap),
            vec![
                None, Some(13), None,
                Some(12), Some(10), Some(11),
                None, Some(10), None
            ]
        );
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
