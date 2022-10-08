extern crate tilemap;
extern crate tilemap_svg;

use tilemap::math::color::{BLACK, BLUE, CYAN, GREEN, ORANGE, RED, YELLOW};
use tilemap::math::side::Side;
use tilemap::math::size2d::Size2d;
use tilemap::renderer::style::aab::BoxStyle;
use tilemap::renderer::style::floor::FloorStyle;
use tilemap::renderer::style::solid::SolidStyle;
use tilemap::renderer::style::wall::WallStyle;
use tilemap::renderer::style::StyleMgr;
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
fn create_wall_example() -> Tilemap2d {
    let tiles = Size2d::new(12, 6);
    let mut tilemap = Tilemap2d::default(tiles, Tile::Floor(0)).unwrap();

    tilemap.set_tile(40, Tile::Solid(0));
    tilemap.set_tile(32, Tile::Solid(1));

    tilemap.set_border(1, Side::Front, Border::Wall(0));
    tilemap.set_border(2, Side::Front, Border::Wall(0));
    tilemap.set_border(3, Side::Front, Border::Wall(0));
    tilemap.set_border(4, Side::Front, Border::Wall(0));
    tilemap.set_border(5, Side::Front, Border::Wall(0));
    tilemap.set_border(6, Side::Front, Border::Wall(0));
    tilemap.set_border(7, Side::Front, Border::Wall(0));
    tilemap.set_border(8, Side::Front, Border::Wall(0));
    tilemap.set_border(9, Side::Front, Border::Wall(0));
    tilemap.set_border(10, Side::Front, Border::Wall(0));

    tilemap.set_border(13, Side::Left, Border::Wall(0));
    tilemap.set_border(25, Side::Left, Border::Wall(0));
    tilemap.set_border(37, Side::Left, Border::Wall(0));
    tilemap.set_border(49, Side::Left, Border::Wall(0));

    tilemap.set_border(23, Side::Left, Border::Wall(0));
    tilemap.set_border(35, Side::Left, Border::Wall(0));
    tilemap.set_border(47, Side::Left, Border::Wall(0));
    tilemap.set_border(59, Side::Left, Border::Wall(0));

    tilemap.set_border(49, Side::Front, Border::Wall(0));
    tilemap.set_border(50, Side::Front, Border::Wall(0));
    tilemap.set_border(51, Side::Front, Border::Wall(0));
    tilemap.set_border(52, Side::Front, Border::Wall(0));
    tilemap.set_border(53, Side::Front, Border::Wall(0));
    tilemap.set_border(54, Side::Front, Border::Wall(0));
    tilemap.set_border(55, Side::Front, Border::Wall(0));
    tilemap.set_border(56, Side::Front, Border::Wall(0));
    tilemap.set_border(57, Side::Front, Border::Wall(0));
    tilemap.set_border(58, Side::Front, Border::Wall(0));

    tilemap
}

fn render(viewer: &dyn View, tilemap: &Tilemap2d, path: &str) {
    let box_style = BoxStyle::new(RED, YELLOW, GREEN);
    let floor_style = FloorStyle::new("floor", CYAN);
    let solid_style0 = SolidStyle::new("solid0", BoxStyle::shaded(ORANGE));
    let solid_style1 = SolidStyle::new("solid1", BoxStyle::shaded(BLUE));
    let wall_style = WallStyle::new("wall", box_style, 10);
    let style = StyleMgr::without_manager(
        vec![floor_style],
        vec![solid_style0, solid_style1],
        vec![wall_style],
        BLACK,
    );
    let svg_size = viewer.get_size(tilemap.get_size());
    let mut builder = SvgBuilder::new(svg_size);

    viewer.render(&tilemap, &mut builder, &style);
    viewer.render_grid(tilemap.get_size(), &mut builder, &style);

    let svg = builder.finish();
    svg.save(path).unwrap();
}
