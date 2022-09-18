use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Health {
            current: 20,
            max: 20,
        },
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    match rng.roll_dice(1, 10) {
        // 8 in 10 chance of getting a goblin
        1..=8 => ecs.push((
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
        // 2 in 10 chance of getting an orc
        _ => ecs.push((
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('o'),
            },
            // Orc will chase the player
            ChasingPlayer {},
            Health { current: 2, max: 2 },
            Name("Orc".to_string()),
        )),
    };
}
