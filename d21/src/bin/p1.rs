type Cost = i32;
type Damage = i32;
type Armor = i32;
type Item = (Cost, Damage, Armor);
struct Loadout {
    weapon: Item,
    armor: Option<Item>,
    ring_1: Option<Item>,
    ring_2: Option<Item>,
}

type HitPoints = i32;
#[derive(Clone)]
struct Status {
    hit_points: HitPoints,
    damage: Damage,
    armor: Armor,
}

fn main() {
    let boss_status: Vec<i32> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.split_once(": ")
                .map(|(_, rhs)| rhs.to_string())
                .expect("No ': ' pattern in line")
        })
        .map(|stat| {
            stat.parse::<u16>()
                .expect("Stat couldn't fit into an unsigned 16 bit integer") as i32
        })
        .collect();
    let boss_status = Status {
        hit_points: boss_status[0],
        damage: boss_status[1],
        armor: boss_status[2],
    };
    let player_hit_points = 100;

    let weapons = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armor = [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
    let rings = [
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let weapon_combinations: Vec<Item> = weapons.to_vec();
    let mut armor_combinations: Vec<Option<Item>> = armor.into_iter().map(Some).collect();
    armor_combinations.push(None);
    let mut ring_combinations: Vec<(Option<Item>, Option<Item>)> = (0..rings.len())
        .flat_map(|i| (i + 1..rings.len()).map(move |j| (i, j)))
        .map(|(i, j)| (rings[i], rings[j]))
        .map(|(r1, r2)| (Some(r1), Some(r2)))
        .collect();
    ring_combinations.append(
        &mut rings
            .iter()
            .cloned()
            .map(Some)
            .map(|item| (item, None))
            .collect(),
    );
    ring_combinations.push((None, None));

    let output = weapon_combinations
        .into_iter()
        .flat_map(|weapon| {
            armor_combinations
                .iter()
                .cloned()
                .map(move |armor| (weapon, armor))
        })
        .flat_map(|(weapon, armor)| {
            ring_combinations
                .iter()
                .cloned()
                .map(move |(ring_1, ring_2)| Loadout {
                    weapon,
                    armor,
                    ring_1,
                    ring_2,
                })
        })
        .map(
            |Loadout {
                 weapon,
                 armor,
                 ring_1,
                 ring_2,
             }| {
                let (weapon_cost, weapon_damage, _) = weapon;
                let loadout_cost: i32 = [
                    Some(weapon_cost),
                    armor.map(|(armor_cost, _, _)| armor_cost),
                    ring_1.map(|(ring_1_cost, _, _)| ring_1_cost),
                    ring_2.map(|(ring_2_cost, _, _)| ring_2_cost),
                ]
                .into_iter()
                .flatten()
                .sum();
                let player_damage: i32 = [
                    Some(weapon_damage),
                    ring_1.map(|(_, ring_1_damage, _)| ring_1_damage),
                    ring_2.map(|(_, ring_2_damage, _)| ring_2_damage),
                ]
                .into_iter()
                .flatten()
                .sum();
                let player_armor: i32 = [
                    armor.map(|(_, _, armor_armor)| armor_armor),
                    ring_1.map(|(_, _, ring_1_armor)| ring_1_armor),
                    ring_2.map(|(_, _, ring_2_armor)| ring_2_armor),
                ]
                .into_iter()
                .flatten()
                .sum();
                let player_status = Status {
                    hit_points: player_hit_points,
                    damage: player_damage,
                    armor: player_armor,
                };
                (loadout_cost, player_status)
            },
        )
        .filter(|(_loadout_cost, player_status)| {
            let mut boss_status = boss_status.clone();
            let mut player_status = player_status.clone();
            loop {
                boss_status.hit_points -= (player_status.damage - boss_status.armor).max(1);
                player_status.hit_points -= (boss_status.damage - player_status.armor).max(1);
                if boss_status.hit_points <= 0 {
                    break true;
                } else if player_status.hit_points <= 0 {
                    break false;
                }
            }
        })
        .map(|(loadout_cost, _player_status)| loadout_cost)
        .min()
        .expect("No combinations of items won the fight");
    println!("{}", output);
}
