#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Blueprint {
    pub id: u64,
    pub ore_robot: Resources,
    pub clay_robot: Resources,
    pub obsidian_robot: Resources,
    pub geode_robot: Resources,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Resources {
    pub ore: u16,
    pub clay: u16,
    pub obsidian: u16,
}

impl Resources {
    pub fn empty() -> Resources {
        Resources { ore: 0, clay: 0, obsidian: 0 }
    }
    pub fn of_ore(ore: u16) -> Resources {
        Resources { ore, clay: 0, obsidian: 0 }
    }
    pub fn of_clay(clay: u16) -> Resources {
        Resources { ore: 0, clay, obsidian: 0 }
    }
    pub fn of_obsidian(obsidian: u16) -> Resources {
        Resources { ore: 0, clay: 0, obsidian }
    }
    pub fn exceeds(&self, other: Resources) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian
    }
    pub fn add(&self, other: Resources) -> Resources {
        Resources {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
        }
    }
    pub fn remove(&self, other: Resources) -> Resources {
        Resources {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
        }
    }
}
