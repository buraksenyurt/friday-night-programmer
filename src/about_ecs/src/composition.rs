// Components
struct Position(f32, f32);
struct Velocity(f32, f32);
struct Health(i32);
struct Strength(i32);
struct Mana(i32);
struct Damage(i32);
struct Range(f32);

// Entities
struct Warrior {
    id: String,
    position: Position,
    health: Health,
    strength: Strength,
}

struct Healer {
    id: String,
    position: Position,
    health: Health,
    mana: Mana,
}

struct Archer {
    id: String,
    position: Position,
    health: Health,
    damage: Damage,
    range: Range,
}

struct Villager {
    id: String,
    position: Position,
    health: Health,
}

struct Tower {
    position: Position,
    damage: Damage,
    range: Range,
}

// Systems
fn attack(id: &str, strength: &Strength) {
    println!("{} attacks with strength {}", id, strength.0);
}
fn take_damage(health: &mut Health, amount: i32) {
    health.0 -= amount;
}

fn shoot_arrow(name: &str, damage: &Damage, range: &Range) {
    println!(
        "{} shoots an arrow with damage {} at range {}",
        name, damage.0, range.0
    );
}

fn heal(id: &str, mana: &Mana, target: &mut Health) {
    target.0 += mana.0;
    println!("{} heals with power {}", id, mana.0);
}

fn build(position: &Position, damage: &Damage, range: &Range) {
    println!(
        "Located on {}:{} with {} damage and range {}",
        position.0, position.1, damage.0, range.0
    );
}

pub fn run() {
    let mut warrior = Warrior {
        id: "Red Skull".to_string(),
        position: Position(50.0, 10.0),
        health: Health(100),
        strength: Strength(25),
    };

    let mut healer = Healer {
        id: "Athena".to_string(),
        position: Position(1.0, 1.0),
        health: Health(80),
        mana: Mana(30),
    };

    let archer = Archer {
        id: "Legolas".to_string(),
        position: Position(2.0, 2.0),
        health: Health(70),
        damage: Damage(40),
        range: Range(100.0),
    };

    let tower = Tower {
        position: Position(5.0, 5.0),
        damage: Damage(60),
        range: Range(50.0),
    };

    attack(&warrior.id, &warrior.strength);
    heal(&healer.id, &healer.mana, &mut warrior.health);
    shoot_arrow(&archer.id, &archer.damage, &archer.range);
    take_damage(&mut warrior.health, 45);
    build(&tower.position, &tower.damage, &tower.range);

    println!("{} has {} health left.", warrior.id, warrior.health.0);
}