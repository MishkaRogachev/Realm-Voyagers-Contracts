use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct HeroStats {
    pub strength: u64,     // Melee damage and physical power
    pub agility: u64,      // Movement, evasion, and initiative
    pub dexterity: u64,    // Precision, accuracy, and stealth
    pub constitution: u64, // Health and physical endurance
    pub intelligence: u64, // Magical power and logical reasoning
    pub wisdom: u64,       // Healing, perception, and magical defense
    pub charisma: u64,     // Social skills and leadership
    pub willpower: u64,    // Mental resilience and focus
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum HeroTag {
    Mage,
    Archer,
    Knight,
}

impl HeroStats {
    pub fn default() -> Self {
        Self {
            strength: 1,
            agility: 1,
            dexterity: 1,
            constitution: 1,
            intelligence: 1,
            wisdom: 1,
            charisma: 1,
            willpower: 1,
        }
    }

    pub fn zero() -> Self {
        Self {
            strength: 0,
            agility: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
            willpower: 0,
        }
    }

    pub fn with_strength(mut self, value: u64) -> Self {
        self.strength = value;
        self
    }

    pub fn with_agility(mut self, value: u64) -> Self {
        self.agility = value;
        self
    }

    pub fn with_dexterity(mut self, value: u64) -> Self {
        self.dexterity = value;
        self
    }

    pub fn with_constitution(mut self, value: u64) -> Self {
        self.constitution = value;
        self
    }

    pub fn with_intelligence(mut self, value: u64) -> Self {
        self.intelligence = value;
        self
    }

    pub fn with_wisdom(mut self, value: u64) -> Self {
        self.wisdom = value;
        self
    }

    pub fn with_charisma(mut self, value: u64) -> Self {
        self.charisma = value;
        self
    }

    pub fn with_willpower(mut self, value: u64) -> Self {
        self.willpower = value;
        self
    }
}

impl std::ops::Add for HeroStats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            strength: self.strength + other.strength,
            agility: self.agility + other.agility,
            dexterity: self.dexterity + other.dexterity,
            constitution: self.constitution + other.constitution,
            intelligence: self.intelligence + other.intelligence,
            wisdom: self.wisdom + other.wisdom,
            charisma: self.charisma + other.charisma,
            willpower: self.willpower + other.willpower,
        }
    }
}

impl HeroTag {
    pub fn base_stats(&self) -> HeroStats {
        match self {
            HeroTag::Mage => HeroStats::default()
                .with_intelligence(1)
                .with_wisdom(1)
                .with_willpower(1),
            HeroTag::Archer => HeroStats::default()
                .with_agility(1)
                .with_dexterity(1)
                .with_wisdom(1),
            HeroTag::Knight => HeroStats::default()
                .with_strength(1)
                .with_constitution(1)
                .with_willpower(1),
        }
    }
}
