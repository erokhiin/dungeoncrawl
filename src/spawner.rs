use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Health {
            current: 10,
            max: 10,
        },
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    match rng.roll_dice(1, 10) {
        1..=6 => ecs.push((
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('g'),
            },
            // Goblin is a monster that moves randomly
            MovingRandomly {},
            Health { current: 1, max: 1 },
            Name("Goblin".to_string()),
        )),
        _ => ecs.push((
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('o'),
            },
            // Orc will chaise the player
            ChasingPlayer {},
            Health { current: 2, max: 2 },
            Name("Orc".to_string()),
        )),
    };
}
