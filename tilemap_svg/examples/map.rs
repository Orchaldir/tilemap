extern crate tilemap;
extern crate tilemap_svg;

use tilemap::math::color::{BLACK, CYAN, GREEN, RED, YELLOW};
use tilemap::math::side::Side;
use tilemap::math::size2d::Size2d;
use tilemap::renderer::style::Style;
use tilemap::renderer::view::isometric::IsometricView;
use tilemap::renderer::view::three_four::ThreeFourView;
use tilemap::renderer::view::top_down::TopDownView;
use tilemap::renderer::view::View;
use tilemap::tilemap::border::Border;
use tilemap::tilemap::tile::Tile;
use tilemap::tilemap::tilemap2d::Tilemap2d;
use tilemap_svg::renderer::SvgBuilder;

fn main() {
    let tilemap = create_wall_example();

    let tile_side = 100;
    let tile_size = Size2d::square(tile_side);
    let height = 200;

    let isometric = IsometricView::new(tile_side, height);
    let three_four = ThreeFourView::new(tile_size, height);
    let top_down = TopDownView::new(tile_size);

    render(&isometric, &tilemap, "test_isometric.svg");
    render(&three_four, &tilemap, "test_34.svg");
    render(&top_down, &tilemap, "test_top.svg");
}

fn create_tile_example() -> Tilemap2d {
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

    tilemap
}

fn create_wall_example() -> Tilemap2d {
    let tiles = Size2d::new(12, 6);
    let mut tilemap = Tilemap2d::default(tiles, Tile::Floor(0)).unwrap();

    tilemap.set_border(32, Side::Back, Border::Wall(0));
    tilemap.set_border(32, Side::Left, Border::Wall(0));
    tilemap.set_border(32, Side::Front, Border::Wall(0));
    tilemap.set_border(32, Side::Right, Border::Wall(0));

    tilemap
}

fn render(viewer: &dyn View, tilemap: &Tilemap2d, path: &str) {
    let style = Style::new_simple(CYAN, RED, BLACK, YELLOW, GREEN, 10);
    let svg_size = viewer.get_size(tilemap.get_size());
    let mut builder = SvgBuilder::new(svg_size);

    viewer.render(&tilemap, &mut builder, &style);
    viewer.render_grid(tilemap.get_size(), &mut builder, &style);

    let svg = builder.finish();
    svg.save(path).unwrap();
}
