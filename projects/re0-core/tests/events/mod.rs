use re0::world::WorldTemplate;

#[test]
fn test() {
    let mut world = WorldTemplate::default();
    world.parse(include_str!("../../../re0-data/世界/现代/物品/白驹过隙.re0")).unwrap();
    println!("{:#?}", world);
}
