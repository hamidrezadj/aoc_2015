type Turns = i32;
type HitPoints = i32;
type ManaPoints = i32;

enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone)]
struct BossStatus {
    hit_points: HitPoints,
    damage: HitPoints,
    poison: Option<Turns>,
}

#[derive(Clone)]
struct PlayerStatus {
    hit_points: HitPoints,
    armor: HitPoints,
    mana_points: ManaPoints,
    mana_points_spent: ManaPoints,
    shield: Option<Turns>,
    recharge: Option<Turns>,
}

fn main() {
    let boss_status: Vec<i32> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.split_once(": ")
                .map(|(_, stat)| stat.to_string())
                .expect("No ': ' pattern in input line")
        })
        .map(|stat| {
            stat.parse::<u16>()
                .expect("Status can't fit in an unsigned 16 bit integer") as i32
        })
        .collect();
    let boss_status = BossStatus {
        hit_points: boss_status[0],
        damage: boss_status[1],
        poison: None,
    };
    let player_status = PlayerStatus {
        hit_points: 50,
        armor: 0,
        mana_points: 500,
        mana_points_spent: 0,
        shield: None,
        recharge: None,
    };

    let mut minimum_mana_points_spent = ManaPoints::MAX;
    let mut states: Vec<(BossStatus, PlayerStatus)> = Default::default();
    states.push((boss_status, player_status));
    while let Some((boss_status, player_status)) = states.pop() {
        if player_status.mana_points_spent > minimum_mana_points_spent {
            continue;
        }
        for spell in Spell::all() {
            let mut boss_status = boss_status.clone();
            let mut player_status = player_status.clone();

            // Player turn
            // Lose HP due to hard mode
            player_status.hit_points -= 1;
            if player_status.hit_points <= 0 {
                continue;
            }
            // Shield turns update
            player_status.shield = player_status
                .shield
                .inspect(|turns| {
                    if *turns <= 1 {
                        player_status.armor -= Spell::shield_armor_amount();
                    }
                })
                .filter(|turns| *turns > 1)
                .map(|turns| turns - 1);
            // Poison turns update
            if boss_status.poison.is_some() {
                boss_status.hit_points -= Spell::poison_damage();
                if boss_status.hit_points <= 0 {
                    minimum_mana_points_spent =
                        minimum_mana_points_spent.min(player_status.mana_points_spent);
                    continue;
                }
            }
            boss_status.poison = boss_status
                .poison
                .filter(|turns| *turns > 1)
                .map(|turns| turns - 1);
            // Recharge turns update
            if player_status.recharge.is_some() {
                player_status.mana_points += Spell::recharge_amount();
            }
            player_status.recharge = player_status
                .recharge
                .filter(|turns| *turns > 1)
                .map(|turns| turns - 1);
            // Cast spell
            player_status.mana_points -= spell.mana_cost();
            if player_status.mana_points <= 0 {
                continue;
            }
            player_status.mana_points_spent += spell.mana_cost();
            match spell {
                Spell::MagicMissile => {
                    boss_status.hit_points -= Spell::magic_missile_damage();
                    if boss_status.hit_points <= 0 {
                        minimum_mana_points_spent =
                            minimum_mana_points_spent.min(player_status.mana_points_spent);
                        continue;
                    }
                }
                Spell::Drain => {
                    boss_status.hit_points -= Spell::drain_damage();
                    if boss_status.hit_points <= 0 {
                        minimum_mana_points_spent =
                            minimum_mana_points_spent.min(player_status.mana_points_spent);
                        continue;
                    }
                    player_status.hit_points += Spell::drain_damage();
                }
                Spell::Shield => {
                    player_status.shield = Some(Spell::shield_turns());
                    player_status.armor += Spell::shield_armor_amount();
                }
                Spell::Poison => {
                    boss_status.poison = Some(Spell::poison_turns());
                }
                Spell::Recharge => {
                    player_status.recharge = Some(Spell::recharge_turns());
                }
            }

            // Boss turn
            // Shield turns update
            player_status.shield = player_status
                .shield
                .inspect(|turns| {
                    if *turns <= 1 {
                        player_status.armor -= Spell::shield_armor_amount();
                    }
                })
                .filter(|turns| *turns > 1)
                .map(|turns| turns - 1);
            // Poison turns update
            if boss_status.poison.is_some() {
                boss_status.hit_points -= Spell::poison_damage();
                if boss_status.hit_points <= 0 {
                    minimum_mana_points_spent =
                        minimum_mana_points_spent.min(player_status.mana_points_spent);
                    continue;
                }
            }
            boss_status.poison = boss_status
                .poison
                .filter(|turns| *turns > 1)
                .map(|turns| turns - 1);
            // Recharge turns update
            if player_status.recharge.is_some() {
                player_status.mana_points += Spell::recharge_amount();
            }
            player_status.recharge = player_status
                .recharge
                .filter(|turns| *turns > 1)
                .map(|turns| turns - 1);
            // Boss attack
            player_status.hit_points -= boss_status.damage - player_status.armor;
            if player_status.hit_points <= 0 {
                continue;
            }

            // Update states queue
            states.push((boss_status, player_status));
        }
    }
    println!("{}", minimum_mana_points_spent);
}

impl Spell {
    fn mana_cost(&self) -> ManaPoints {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn all() -> [Spell; 5] {
        [
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ]
    }

    fn poison_damage() -> HitPoints {
        3
    }

    fn recharge_amount() -> ManaPoints {
        101
    }

    fn shield_armor_amount() -> HitPoints {
        7
    }

    fn magic_missile_damage() -> HitPoints {
        4
    }

    fn drain_damage() -> HitPoints {
        2
    }

    fn shield_turns() -> HitPoints {
        6
    }

    fn poison_turns() -> HitPoints {
        6
    }

    fn recharge_turns() -> ManaPoints {
        5
    }
}
