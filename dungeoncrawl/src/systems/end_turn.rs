use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let amulet_pos = <&Point>::query()
        .filter(component::<AmuletOfYala>())
        .iter(ecs)
        .next()
        .unwrap();
    let current_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };
    <(&Health, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|(hp, pos)| {
            if pos == amulet_pos {
                new_state = TurnState::Victory;
            } else if hp.current < 1 {
                new_state = TurnState::GameOver;
            }
        });
    *turn_state = new_state;
}
