use re0::world::WorldTemplate;

#[test]
fn test() {
    let mut world = WorldTemplate::default();
    world.parse(include_str!("世界.re0")).unwrap();
    println!("{:#?}", world);
}
