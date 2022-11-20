#[derive(Debug)]
pub enum CharacterKind {
    Mankind,
}

#[derive(Debug)]
pub struct Character {
    name: String,
    power: u32,
    hp: u32,
    kind: CharacterKind,
    role: Box<Role>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum RoleKind {
    OrdinaryRole,
    Magician,
    Knight
}

#[derive(Debug)]
pub struct Role {
    kind: RoleKind,
    skills: Box<Vec<Skill>>,
}

#[derive(Debug, Clone)]
pub struct Skill {
    name: String,
    damage: u32,
    quality: u32,
}

/// 泛型 trait
pub trait Influence<F> {
    type Consequence;
    fn affection(&mut self, factor: F) -> Self::Consequence;
}

impl Character {
    fn new(name: &str, kind: CharacterKind) -> Character {
        Character {
            name: name.to_string(),
            power: 100,
            hp: 1000,
            kind: kind,
            role: Box::new(Role::new())
        }
    }

    fn learn_skill(&mut self, new_skill: Skill) {
        self.role.skills.push(new_skill);
    }

    fn change_role(&mut self, new_role: RoleKind) {
        self.role.change(new_role);
    }
}

impl Role {

    fn new() -> Role {
        Role::with_role_kind(RoleKind::OrdinaryRole)
    }

    fn with_role_kind(kind: RoleKind) -> Role {
        kind.init_skills()
    }

    fn change(&mut self, new_kind: RoleKind) -> Option<()> {
        if new_kind == RoleKind::OrdinaryRole ||
            self.kind != RoleKind::OrdinaryRole {
            return None;
        }

        let skills = self.skills.clone();

        let new_role = new_kind.init_skills();

        self.kind = new_role.kind;
        self.skills = new_role.skills;

        for skill in skills.into_iter() {
            self.skills.push(skill)
        }

        Some(())
    }
}

impl RoleKind {
    fn init_skills(&self) -> Role {
        match self {
            RoleKind::OrdinaryRole =>
                Role {kind: RoleKind::OrdinaryRole,
                    skills: Box::new(vec![Skill {name: "Attack".to_string(), damage: 102, quality: 10}])},
            RoleKind::Magician =>
                Role {kind: RoleKind::Magician,
                    skills: Box::new(vec![Skill {name: "Fire".to_string(), damage: 200, quality: 12}])},
            RoleKind::Knight =>
                Role {kind: RoleKind::Knight,
                    skills: Box::new(vec![Skill {name: "Charge Forward".to_string(), damage: 210, quality: 12}])}
        }
    }
}

impl Influence<Role> for Character {

    type Consequence = String;

    fn affection(&mut self, factor: Role) -> Self::Consequence {
        self.role = Box::new(factor);
        format!("{}'s role has changed to {:?}", self.name, self.role.kind)
    }

}

impl Influence<Skill> for Character {
    type Consequence = u32;

    fn affection(&mut self, factor: Skill) -> Self::Consequence {
        self.power += factor.quality as u32;
        self.power
    }
}

#[test]
fn test_character() {
    let mut an = Character::new("An", CharacterKind::Mankind);
    println!("{:?}", &an);

    let cons = Influence::<Skill>::affection(&mut an,
                                  Skill {name: "Po".to_string(), damage: 200, quality: 13});
    println!("{}", cons);

    an.role.change(RoleKind::Magician);
    println!("{:?}", &an);

}