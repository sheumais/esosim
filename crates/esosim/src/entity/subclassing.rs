//! Subclassing on the player (the rules themselves are `esosim_types::subclass`).
//!
//! A character has three class skill lines: its class's own, with up to two replaced by one line each of two other
//! classes. What that changes for the calculator:
//!
//! * which abilities count for a passive such as Expert Mage or Pressure Points (`Condition::ClassAbilitiesSlotted`): those
//!   native to the passive's class, whichever class slots them, so the subclassing itself is not consulted;
//! * which class passives the character has: a passive follows its line, so a Dragonknight with Shadow has Shadow's
//!   passives (`passive_effect_rules_with`, `Build::to_player`);
//! * Class Mastery, which only a character with all three own lines has.
//!
//! As with sets, nothing is refused: [`Player::subclass_issues`] reports what the game would not allow. The sheet,
//! though, only ever uses the swaps that are legal (see `Subclassing::resolve`).

use eso_skill_data::enums::skill_line::SkillLine;
use esosim_types::{PlayerClass, SubclassIssue, Subclassing};

use crate::entity::player::Player;

impl Player {
    /// The character's class lines, worked out from its class and `Player::subclass`. `None` while the class is
    /// not set.
    pub fn subclassing(&self) -> Option<Subclassing> {
        self.class.map(|class| Subclassing::resolve(class, &self.subclass))
    }

    /// The three class skill lines the character has. `None` while the class is not set.
    pub fn class_lines(&self) -> Option<Vec<SkillLine>> {
        self.subclassing().map(|s| s.lines)
    }

    /// Whether the character has Class Mastery (all three own lines). `None` while the class is not set.
    pub fn has_class_mastery(&self) -> Option<bool> {
        self.subclassing().map(|s| s.has_class_mastery())
    }

    /// What is wrong with the subclass choice, then every class-line ability on either bar that the character does
    /// not have the line for (an ability of the class it gave up, or of a class it never took a line from). Empty
    /// for a legal build. Needs `ability_skill_lines`, like every skill-line question.
    pub fn subclass_issues(&self) -> Vec<SubclassIssue> {
        let Some(subclassing) = self.subclassing() else { return Vec::new() };
        let mut issues = subclassing.issues.clone();
        let mut seen: Vec<u32> = Vec::new();
        for ability_id in self.primary_abilities.iter().chain(&self.backup_abilities) {
            if seen.contains(ability_id) {
                continue;
            }
            seen.push(*ability_id);
            let Some(line) = self.ability_skill_lines.get(ability_id).copied() else { continue };
            if PlayerClass::of_skill_line(line).is_some() && !subclassing.has_line(line) {
                issues.push(SubclassIssue::SlottedOutsideYourLines { ability_id: *ability_id, line });
            }
        }
        issues
    }
}

#[cfg(test)]
mod tests {
    use esosim_types::SubclassSwap;

    use super::*;

    fn line(name: &str) -> SkillLine {
        (0u32..4096).filter_map(|i| SkillLine::from_id(&i)).find(|l| l.as_str() == name).unwrap()
    }

    /// A Dragonknight with Shadow instead of Draconic Power; abilities 1 to 4 are Ardent Flame, Draconic Power,
    /// Shadow and Mages Guild.
    fn subclassed_dragonknight() -> Player {
        let mut p = Player::new();
        p.class = Some(PlayerClass::Dragonknight);
        p.subclass = vec![SubclassSwap::new(line("Draconic Power"), line("Shadow"))];
        p.ability_skill_lines =
            [(1, line("Ardent Flame")), (2, line("Draconic Power")), (3, line("Shadow")), (4, line("Mages Guild"))].into();
        p
    }

    #[test]
    fn nothing_is_known_without_a_class() {
        let p = Player::new();
        assert_eq!((p.subclassing(), p.class_lines(), p.has_class_mastery()), (None, None, None));
        assert_eq!(p.subclass_issues(), []);
    }

    #[test]
    fn a_pure_class_has_its_own_lines_and_class_mastery() {
        let mut p = subclassed_dragonknight();
        p.subclass.clear();
        let own: Vec<_> = ["Ardent Flame", "Draconic Power", "Earthen Heart"].into_iter().map(line).collect();
        assert_eq!(p.class_lines(), Some(own));
        assert_eq!(p.has_class_mastery(), Some(true));
        assert_eq!(p.subclass_issues(), []);
    }

    #[test]
    fn a_subclassed_character_has_the_new_line_and_no_class_mastery() {
        let p = subclassed_dragonknight();
        let lines = p.class_lines().unwrap();
        assert!(lines.contains(&line("Shadow")) && !lines.contains(&line("Draconic Power")));
        assert_eq!(lines.len(), 3);
        assert_eq!(p.has_class_mastery(), Some(false));
        assert_eq!(p.subclass_issues(), []);
    }

    #[test]
    fn class_abilities_are_counted_by_the_class_the_line_belongs_to() {
        let mut p = subclassed_dragonknight();
        p.primary_abilities = vec![1, 2, 3, 4];
        let count = |p: &Player, class| p.get_number_of_active_class_skills(class);
        // the Shadow ability is a Nightblade one even on a Dragonknight, and the guild one is nobody's
        assert_eq!(count(&p, PlayerClass::Nightblade), 1);
        assert_eq!(count(&p, PlayerClass::Dragonknight), 2, "Ardent Flame and Draconic Power");
        assert_eq!(count(&p, PlayerClass::Sorcerer), 0);
        p.primary_abilities = vec![3];
        assert_eq!(count(&p, PlayerClass::Dragonknight), 0);
    }

    #[test]
    fn an_ability_of_a_line_the_character_does_not_have_is_reported_once() {
        let mut p = subclassed_dragonknight();
        // 2 is a Draconic Power ability (given up); 1, 3 and the guild ability 4 are fine; 2 is on both bars
        p.primary_abilities = vec![1, 2, 3, 4];
        p.backup_abilities = vec![2];
        assert_eq!(
            p.subclass_issues(),
            [SubclassIssue::SlottedOutsideYourLines { ability_id: 2, line: line("Draconic Power") }]
        );
    }

    #[test]
    fn a_bad_swap_is_reported_and_the_sheet_ignores_it() {
        let mut p = subclassed_dragonknight();
        // a second Nightblade line
        p.subclass.push(SubclassSwap::new(line("Ardent Flame"), line("Siphoning")));
        assert_eq!(
            p.subclass_issues(),
            [SubclassIssue::TwoLinesFromOneClass { class: PlayerClass::Nightblade, with: line("Siphoning") }]
        );
        assert!(p.class_lines().unwrap().contains(&line("Ardent Flame")), "the second swap does nothing");
    }
}
