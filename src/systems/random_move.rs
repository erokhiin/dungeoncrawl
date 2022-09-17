use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();

    movers.iter(ecs).for_each(|(mover, pos, _)| {
        let mut rng = RandomNumberGenerator::new();

        let delta = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };
        let destination = delta + *pos;

        let is_mover_do_something = delta.x != 0 || delta.y != 0;

        if is_mover_do_something {
            let mut hit_someone = false;

            positions
                .iter(ecs)
                .filter(|(_, pos, _)| **pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *mover,
                                victim: *victim,
                            },
                        ));
                    }
                    hit_someone = true; // This prevents the monster from moving into a tile containing another monster
                });

            if !hit_someone {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *mover,
                        destination,
                    },
                ));
            }
        }
    })
}
