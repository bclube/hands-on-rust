use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let victims = <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect::<Vec<(Entity, Entity)>>();
    let player_entity = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .copied();
    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;
            if health.current < 1 && player_entity.map_or(false, |pe| victim != &pe) {
                commands.remove(*victim);
            };
        };
        commands.remove(*message);
    });
}
