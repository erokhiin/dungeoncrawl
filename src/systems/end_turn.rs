use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(TheGlitch)]
#[read_component(Player)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState
) {

    let mut player_hp = <(&Health, &Point)>::query()
        .filter(component::<Player>());
    let mut amulet = <&Point>::query()
        .filter(component::<TheGlitch>());

    let amulet_position = amulet.iter(ecs).nth(0).unwrap();

    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state
    };

    player_hp.iter(ecs)
        .for_each(|(hp, pos)| {
            if hp.current < 1 {
                new_state = TurnState::GameOver;
            }
            if pos == amulet_position {
                new_state = TurnState::Victory;
            }
        });

    *turn_state = new_state;
}
