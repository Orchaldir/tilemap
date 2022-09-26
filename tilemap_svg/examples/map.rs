extern crate tilemap;
extern crate tilemap_svg;

use tilemap::math::color::{BLACK, CYAN, GREEN, RED};
use tilemap::math::size2d::Size2d;
use tilemap::renderer::style::Style;
use tilemap::renderer::view::three_four::ThreeFourView;
use tilemap::renderer::view::top_down::TopDownView;
use tilemap::tilemap::tile::Tile;
use tilemap::tilemap::tilemap2d::Tilemap2d;
use tilemap_svg::renderer::SvgBuilder;

fn main() {
    let tiles = Size2d::new(3, 3);
    let tile_size = Size2d::square(100);
    let mut tilemap = Tilemap2d::default(tiles, Tile::Empty).unwrap();

    for i in 0..tiles.count() {
        tilemap.set_tile(
            i,
            if i % 2 == 0 {
                Tile::Floor(1)
            } else {
                Tile::Solid(3)
            },
        );
    }

    let mut builder = SvgBuilder::new(tiles * tile_size);
    let top_down = TopDownView::new(tile_size);
    let three_four = ThreeFourView::new(tile_size, 200);
    let style = Style::new_simple(CYAN, RED, GREEN);

    three_four.render(&tilemap, &mut builder, &style);

    let svg = builder.finish();
    svg.save("test.svg").unwrap();
}
