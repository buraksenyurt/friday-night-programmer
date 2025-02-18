struct Character {
    id: String,
    health_value: f32,
}

impl Character {
    fn new(id: &str, health_value: f32) -> Self {
        Self {
            id: id.to_string(),
            health_value,
        }
    }
    fn take_damage(&mut self, amount: f32) {
        self.health_value -= amount;
    }
}

struct Archer {
    character: Character,
    range: f32,
}

impl Archer {
    fn new(id: &str, health_value: f32, range: f32) -> Self {
        Self {
            character: Character::new(id, health_value),
            range,
        }
    }
    fn fire(&self) {
        println!(
            "Archer {} fires at well with range {} unit",
            self.character.id, self.range
        );
    }
}

struct Warrior {
    character: Character,
    strength: f32,
}

impl Warrior {
    fn new(id: &str, health_value: f32, strength: f32) -> Self {
        Self {
            character: Character::new(id, health_value),
            strength,
        }
    }
    fn attack(&self) {
        println!(
            "Warrior {} attacks with strength {} power",
            self.character.id, self.strength
        );
    }
}

struct Healer {
    character: Character,
    mana_power: f32,
}

impl Healer {
    fn new(name: &str, health_value: f32, mana_power: f32) -> Self {
        Self {
            character: Character::new(name, health_value),
            mana_power,
        }
    }

    fn heal(&self) {
        println!(
            "{} heals with power {} mana",
            self.character.id, self.mana_power
        );
    }
}

pub fn run() {
    let white_hand = Archer::new("wh-666", 100.0, 900.50);
    let mut boramir = Warrior::new("boramir-13", 100.0, 8.75);
    let gandalf = Healer::new("gandalf-13", 100.0, 1.25);

    white_hand.fire();
    boramir.attack();
    boramir.character.take_damage(10.5);
    gandalf.heal();
    println!("Boramir health {}", boramir.character.health_value);
}