extern crate tilemap;
extern crate tilemap_svg;

use tilemap::math::color::{CYAN, GREEN, RED, YELLOW};
use tilemap::math::size2d::Size2d;
use tilemap::renderer::style::Style;
use tilemap::renderer::view::isometric::IsometricView;
use tilemap::renderer::view::three_four::ThreeFourView;
use tilemap::renderer::view::top_down::TopDownView;
use tilemap::renderer::view::View;
use tilemap::tilemap::tile::Tile;
use tilemap::tilemap::tilemap2d::Tilemap2d;
use tilemap_svg::renderer::SvgBuilder;

fn main() {
    let tiles = Size2d::new(3, 3);
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

    let tile_size = Size2d::square(100);
    let height = 200;

    let isometric = IsometricView::new(tile_size);
    let three_four = ThreeFourView::new(tile_size, height);
    let top_down = TopDownView::new(tile_size);

    render(&isometric, &tilemap, "test_isometric.svg");
    render(&three_four, &tilemap, "test_34.svg");
    render(&top_down, &tilemap, "test_top.svg");
}

fn render(viewer: &dyn View, tilemap: &Tilemap2d, path: &str) {
    let style = Style::new_simple(CYAN, RED, YELLOW, GREEN);
    let svg_size = viewer.get_size(tilemap);
    let mut builder = SvgBuilder::new(svg_size);

    viewer.render(&tilemap, &mut builder, &style);

    let svg = builder.finish();
    svg.save(path).unwrap();
}
