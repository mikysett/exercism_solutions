pub struct Allergies(i64);

const ALLERGEN_COUNT: u32 = 8;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score as i64)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & *allergen as i64 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..ALLERGEN_COUNT)
            .map(|x| unsafe { std::mem::transmute(1 << x) })
            .filter(|curr_allergen| self.is_allergic_to(curr_allergen))
            .collect()
    }
}
