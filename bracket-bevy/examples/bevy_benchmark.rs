use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    let bterm = BTermBuilder::simple_80x50().with_random_number_generator(true);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bterm)
        .add_system(tick)
        .run();
}

fn tick(ctx: Res<BracketContext>, rng: Res<RandomNumbers>) {
    for y in 0..50 {
        for x in 0..80 {
            let val = rng.rand::<u64>();
            let back = RGB::from_u8(
                (val & 0xFF) as u8,
                ((val >> 8) & 0x5F) as u8,
                ((val >> 16) & 0x3F) as u8,
            );
            let fore = RGB::from_u8(
                ((val >> 16) & 0xFF) as u8,
                ((val >> 24) & 0xFF) as u8,
                ((val >> 32) & 0xFF) as u8,
            );
            let ascii = ((val >> 40) & 0xFF) as u16;
            ctx.set(x, y, fore, back, ascii);
        }
    }
    ctx.draw_box(30, 20, 20, 5, WHITE, BLACK);
    ctx.print_centered(22, &format!("{} fps", ctx.fps as u32));
}
