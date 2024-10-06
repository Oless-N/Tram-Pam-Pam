use crate::characters::Character;

pub struct Physics;

impl Physics {
    pub fn apply_physics(character: &mut Character) {
        // Проста фізика для бокового виду
        if character.y < 520.0 {
            character.y += 5.0;  // гравітація
        }
    }
}
