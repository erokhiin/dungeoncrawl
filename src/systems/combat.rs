use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let attack_damage: i32 = 1;
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(message, attack)| (*message, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
          print!("Health before attack: {}", health.current);
          health.current -= attack_damage;
          if health.current < 1 {
            commands.remove(*victim)
          }
          println!("Health after attack: {}", health.current)
        }
        commands.remove(*message)
    })
}
