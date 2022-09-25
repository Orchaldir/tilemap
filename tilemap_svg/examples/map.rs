extern crate tilemap;
extern crate tilemap_svg;

use tilemap::math::size2d::Size2d;
use tilemap::renderer::top_down::TopDownRenderer;
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
    let top_down = TopDownRenderer::new(tile_size);

    top_down.render(&tilemap, &mut builder);

    let svg = builder.finish();
    svg.save("test.svg");
}