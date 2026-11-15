//! Subclassing: which class skill lines a character really has.
//!
//! A character has three class skill lines, the three of its own class. Subclassing lets it replace some of them by
//! a class line of another class. The rules of the game, as modelled here:
//!
//! * up to **two** of the three own lines can be replaced, so at least one own line always stays
//!   ([`MAX_SUBCLASS_SWAPS`]);
//! * a replacement is one whole line of **another** class (never an own-class line, never a weapon, guild or other
//!   non-class line);
//! * the two replacements cannot come from the **same** class;
//! * replacing even one own line **removes Class Mastery** (the five passives of skill lines 351 to 357), which only a
//!   character with all three own lines has;
//! * the passives of a line follow the line: a Dragonknight with Shadow has Shadow's passives, and has lost those of
//!   the line it gave up.
//!
//! [`Subclassing::resolve`] turns a class and a list of swaps into the lines the character has. A swap that breaks a
//! rule is not applied and is named in [`Subclassing::issues`]: the sheet only ever describes a character the game
//! could have, and the problem is reported rather than hidden (the same way `Player::set_issues` reports sets).

use std::fmt;

use eso_skill_data::enums::skill_line::SkillLine;
use serde::{Deserialize, Serialize};

use crate::condition::PlayerClass;

/// How many own class lines can be replaced.
pub const MAX_SUBCLASS_SWAPS: usize = 2;

/// One own class line given up for a line of another class. A build lists these in `Build::subclass`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubclassSwap {
    /// The character's own line that is given up.
    #[serde(with = "crate::skill_serde::skill_line")]
    pub replaces: SkillLine,
    /// The line of another class that takes its place.
    #[serde(with = "crate::skill_serde::skill_line")]
    pub with: SkillLine,
}

impl SubclassSwap {
    pub fn new(replaces: SkillLine, with: SkillLine) -> Self {
        Self { replaces, with }
    }
}

/// What is wrong with a subclass choice. All of them are things the game does not allow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubclassIssue {
    /// More than [`MAX_SUBCLASS_SWAPS`] swaps are listed; only the first two are considered.
    TooManySwaps { count: usize },
    /// `replaces` is not one of the character's own class lines (a weapon line, another class's line ...).
    NotOneOfYourLines { replaces: SkillLine },
    /// The same own line is replaced by two swaps.
    ReplacedTwice { replaces: SkillLine },
    /// `with` is not a class line at all (a weapon, guild, world ... line).
    NotAClassLine { with: SkillLine },
    /// `with` is a line of the character's own class: that is not subclassing.
    YourOwnClassLine { with: SkillLine },
    /// A second line from a class that already gave one.
    TwoLinesFromOneClass { class: PlayerClass, with: SkillLine },
    /// A class line is slotted that the character does not have (its class's, or one it replaced). Found by
    /// `Player::subclass_issues`, which knows the bars; [`Subclassing::resolve`] never reports it.
    SlottedOutsideYourLines { ability_id: u32, line: SkillLine },
}

impl fmt::Display for SubclassIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubclassIssue::TooManySwaps { count } => {
                write!(f, "{count} skill lines replaced, at most {MAX_SUBCLASS_SWAPS} can be")
            }
            SubclassIssue::NotOneOfYourLines { replaces } => {
                write!(f, "{} is not one of your class skill lines", replaces.as_str())
            }
            SubclassIssue::ReplacedTwice { replaces } => write!(f, "{} is replaced twice", replaces.as_str()),
            SubclassIssue::NotAClassLine { with } => write!(f, "{} is not a class skill line", with.as_str()),
            SubclassIssue::YourOwnClassLine { with } => {
                write!(f, "{} is a line of your own class, not a subclass line", with.as_str())
            }
            SubclassIssue::TwoLinesFromOneClass { class, with } => {
                write!(f, "{} is a second skill line from {class:?}: only one line per other class", with.as_str())
            }
            SubclassIssue::SlottedOutsideYourLines { ability_id, line } => {
                write!(f, "ability {ability_id} is from {}, which you do not have", line.as_str())
            }
        }
    }
}

/// The class skill lines a character has, worked out from its class and its swaps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subclassing {
    pub class: PlayerClass,
    /// The three class lines the character has: its own, with a replaced one's place taken by the line that replaced
    /// it. Fewer than three only if the parser does not know a line of the class (a test pins that it does).
    pub lines: Vec<SkillLine>,
    /// The swaps that took effect, in the order given.
    pub applied: Vec<SubclassSwap>,
    /// Why the swaps that did not take effect were refused. Empty for a legal choice.
    pub issues: Vec<SubclassIssue>,
}

impl Subclassing {
    /// Apply `swaps` to the lines of `class`. Only the first [`MAX_SUBCLASS_SWAPS`] entries are looked at; each must
    /// replace a different own line with a line of a class that is not `class` and has not been used by an earlier swap.
    /// A swap that fails is left out and reported.
    pub fn resolve(class: PlayerClass, swaps: &[SubclassSwap]) -> Self {
        let own = class.skill_lines();
        let mut lines = own.to_vec();
        let mut applied: Vec<SubclassSwap> = Vec::new();
        let mut issues = Vec::new();
        let mut classes_used: Vec<PlayerClass> = Vec::new();

        if swaps.len() > MAX_SUBCLASS_SWAPS {
            issues.push(SubclassIssue::TooManySwaps { count: swaps.len() });
        }
        for swap in swaps.iter().take(MAX_SUBCLASS_SWAPS) {
            if !own.contains(&swap.replaces) {
                issues.push(SubclassIssue::NotOneOfYourLines { replaces: swap.replaces });
                continue;
            }
            if applied.iter().any(|done| done.replaces == swap.replaces) {
                issues.push(SubclassIssue::ReplacedTwice { replaces: swap.replaces });
                continue;
            }
            let Some(owner) = PlayerClass::of_skill_line(swap.with) else {
                issues.push(SubclassIssue::NotAClassLine { with: swap.with });
                continue;
            };
            if owner == class {
                issues.push(SubclassIssue::YourOwnClassLine { with: swap.with });
                continue;
            }
            if classes_used.contains(&owner) {
                issues.push(SubclassIssue::TwoLinesFromOneClass { class: owner, with: swap.with });
                continue;
            }
            let Some(at) = lines.iter().position(|line| *line == swap.replaces) else { continue };
            lines[at] = swap.with;
            classes_used.push(owner);
            applied.push(*swap);
        }
        Self { class, lines, applied, issues }
    }

    /// Does the character have this class line (its own, or one it took)?
    pub fn has_line(&self, line: SkillLine) -> bool {
        self.lines.contains(&line)
    }

    /// A character that replaced none of its own lines.
    pub fn is_pure(&self) -> bool {
        self.applied.is_empty()
    }

    /// Class Mastery needs all three own lines: any swap that took effect removes it.
    pub fn has_class_mastery(&self) -> bool {
        self.is_pure()
    }

    /// Does the character have a class passive, given the class its record names (`EffectSource::Class { name }`,
    /// the class's `{:?}`) and the skill line the passive is in?
    ///
    /// * a Class Mastery passive (its line is one of [`class_mastery_line`]) needs Class Mastery and the passive's
    ///   own class;
    /// * any other class passive needs its line: that is the rule that gives a Dragonknight with Shadow the Shadow
    ///   passives and takes away the ones of a line it replaced;
    /// * a passive whose line is not known falls back to the class it names being the character's own class, as
    ///   before subclassing existed.
    pub fn has_class_passive(&self, class_name: &str, line: Option<SkillLine>) -> bool {
        match line {
            Some(line) if is_class_mastery_line(line) => {
                self.has_class_mastery() && class_name == format!("{:?}", self.class)
            }
            Some(line) => self.has_line(line),
            None => class_name == format!("{:?}", self.class),
        }
    }
}

/// The skill line that holds a class's five Class Mastery passives (skill lines 351 to 357).
pub fn class_mastery_line(class: PlayerClass) -> Option<SkillLine> {
    let id: u32 = match class {
        PlayerClass::Dragonknight => 351,
        PlayerClass::Arcanist => 352,
        PlayerClass::Necromancer => 353,
        PlayerClass::Warden => 354,
        PlayerClass::Templar => 355,
        PlayerClass::Nightblade => 356,
        PlayerClass::Sorcerer => 357,
    };
    SkillLine::from_id(&id)
}

/// Is `line` the Class Mastery line of some class?
pub fn is_class_mastery_line(line: SkillLine) -> bool {
    PlayerClass::ALL.into_iter().any(|class| class_mastery_line(class) == Some(line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skill_serde::skill_line_named;

    fn line(name: &str) -> SkillLine {
        skill_line_named(name).unwrap_or_else(|| panic!("the parser has no skill line `{name}`"))
    }

    fn swap(replaces: &str, with: &str) -> SubclassSwap {
        SubclassSwap::new(line(replaces), line(with))
    }

    fn lines_of(s: &Subclassing) -> Vec<&'static str> {
        s.lines.iter().map(|l| l.as_str()).collect()
    }

    #[test]
    fn no_swaps_is_the_pure_class() {
        let s = Subclassing::resolve(PlayerClass::Dragonknight, &[]);
        assert_eq!(lines_of(&s), ["Ardent Flame", "Draconic Power", "Earthen Heart"]);
        assert!(s.is_pure() && s.has_class_mastery());
        assert!(s.applied.is_empty() && s.issues.is_empty());
    }

    #[test]
    fn one_swap_replaces_that_line_in_place_and_costs_class_mastery() {
        let s = Subclassing::resolve(PlayerClass::Dragonknight, &[swap("Draconic Power", "Shadow")]);
        assert_eq!(lines_of(&s), ["Ardent Flame", "Shadow", "Earthen Heart"]);
        assert!(s.issues.is_empty());
        assert!(!s.is_pure() && !s.has_class_mastery(), "one replaced line is enough to lose it");
        assert!(s.has_line(line("Shadow")) && !s.has_line(line("Draconic Power")));
    }

    #[test]
    fn two_swaps_from_two_different_classes_leave_one_own_line() {
        let swaps = [swap("Ardent Flame", "Storm Calling"), swap("Earthen Heart", "Assassination")];
        let s = Subclassing::resolve(PlayerClass::Dragonknight, &swaps);
        assert_eq!(lines_of(&s), ["Storm Calling", "Draconic Power", "Assassination"]);
        assert!(s.issues.is_empty() && s.applied == swaps);
        let own = PlayerClass::Dragonknight.skill_lines();
        assert_eq!(s.lines.iter().filter(|l| own.contains(l)).count(), 1, "at least one own line always stays");
    }

    #[test]
    fn two_lines_from_the_same_class_are_refused_and_the_second_is_left_out() {
        // Shadow and Siphoning are both Nightblade lines
        let swaps = [swap("Ardent Flame", "Shadow"), swap("Earthen Heart", "Siphoning")];
        let s = Subclassing::resolve(PlayerClass::Dragonknight, &swaps);
        assert_eq!(s.issues, [SubclassIssue::TwoLinesFromOneClass { class: PlayerClass::Nightblade, with: line("Siphoning") }]);
        assert_eq!(lines_of(&s), ["Shadow", "Draconic Power", "Earthen Heart"]);
        assert_eq!(s.applied, [swaps[0]]);
        // the order decides which one is kept
        let reversed = Subclassing::resolve(PlayerClass::Dragonknight, &[swaps[1], swaps[0]]);
        assert_eq!(lines_of(&reversed), ["Ardent Flame", "Draconic Power", "Siphoning"]);
    }

    #[test]
    fn a_third_swap_is_ignored_and_reported() {
        let swaps = [
            swap("Ardent Flame", "Storm Calling"),
            swap("Draconic Power", "Shadow"),
            swap("Earthen Heart", "Aedric Spear"),
        ];
        let s = Subclassing::resolve(PlayerClass::Dragonknight, &swaps);
        assert_eq!(s.issues, [SubclassIssue::TooManySwaps { count: 3 }]);
        assert_eq!(s.applied.len(), 2);
        assert!(s.has_line(line("Earthen Heart")), "the third swap did nothing");
    }

    #[test]
    fn a_line_that_is_not_yours_cannot_be_replaced_and_one_line_cannot_be_replaced_twice() {
        let s = Subclassing::resolve(PlayerClass::Dragonknight, &[swap("Shadow", "Storm Calling")]);
        assert_eq!(s.issues, [SubclassIssue::NotOneOfYourLines { replaces: line("Shadow") }]);
        assert!(s.is_pure(), "nothing took effect, so Class Mastery is still there");

        let twice = [swap("Ardent Flame", "Storm Calling"), swap("Ardent Flame", "Shadow")];
        let s = Subclassing::resolve(PlayerClass::Dragonknight, &twice);
        assert_eq!(s.issues, [SubclassIssue::ReplacedTwice { replaces: line("Ardent Flame") }]);
        assert_eq!(lines_of(&s), ["Storm Calling", "Draconic Power", "Earthen Heart"]);
    }

    #[test]
    fn the_replacement_must_be_a_line_of_another_class() {
        let own = Subclassing::resolve(PlayerClass::Dragonknight, &[swap("Ardent Flame", "Draconic Power")]);
        assert_eq!(own.issues, [SubclassIssue::YourOwnClassLine { with: line("Draconic Power") }]);
        assert!(own.is_pure());

        let guild = Subclassing::resolve(PlayerClass::Dragonknight, &[swap("Ardent Flame", "Mages Guild")]);
        assert_eq!(guild.issues, [SubclassIssue::NotAClassLine { with: line("Mages Guild") }]);
        assert!(guild.is_pure());
    }

    #[test]
    fn a_refused_swap_does_not_use_up_a_class() {
        // the first swap fails (not an own line), so Nightblade is still free for the second
        let swaps = [swap("Shadow", "Siphoning"), swap("Ardent Flame", "Shadow")];
        let s = Subclassing::resolve(PlayerClass::Dragonknight, &swaps);
        assert_eq!(s.issues, [SubclassIssue::NotOneOfYourLines { replaces: line("Shadow") }]);
        assert_eq!(lines_of(&s), ["Shadow", "Draconic Power", "Earthen Heart"]);
    }

    #[test]
    fn a_class_passive_follows_its_line() {
        let swapped = Subclassing::resolve(PlayerClass::Dragonknight, &[swap("Draconic Power", "Shadow")]);
        let pure = Subclassing::resolve(PlayerClass::Dragonknight, &[]);
        let (draconic, shadow) = (Some(line("Draconic Power")), Some(line("Shadow")));
        // Draconic Power's passive is gone, Shadow's (a Nightblade record) has come
        assert!(pure.has_class_passive("Dragonknight", draconic) && !swapped.has_class_passive("Dragonknight", draconic));
        assert!(!pure.has_class_passive("Nightblade", shadow) && swapped.has_class_passive("Nightblade", shadow));
        // a line the character never had
        assert!(!swapped.has_class_passive("Sorcerer", Some(line("Storm Calling"))));
        // an unknown line falls back to the class name
        assert!(pure.has_class_passive("Dragonknight", None) && !pure.has_class_passive("Nightblade", None));
    }

    #[test]
    fn class_mastery_needs_the_pure_class_and_its_own_mastery_line() {
        let mastery = class_mastery_line(PlayerClass::Dragonknight);
        assert!(mastery.is_some_and(is_class_mastery_line));
        let pure = Subclassing::resolve(PlayerClass::Dragonknight, &[]);
        assert!(pure.has_class_passive("Dragonknight", mastery));
        assert!(!pure.has_class_passive("Nightblade", mastery), "another class's mastery");
        let swapped = Subclassing::resolve(PlayerClass::Dragonknight, &[swap("Draconic Power", "Shadow")]);
        assert!(!swapped.has_class_passive("Dragonknight", mastery), "subclassing removes Class Mastery");
        // a refused swap changes nothing, so the mastery stays
        let refused = Subclassing::resolve(PlayerClass::Dragonknight, &[swap("Draconic Power", "Mages Guild")]);
        assert!(refused.has_class_passive("Dragonknight", mastery));
    }

    #[test]
    fn every_class_has_its_own_mastery_line_and_no_mastery_line_is_a_class_line() {
        let mut seen = Vec::new();
        for class in PlayerClass::ALL {
            let mastery = class_mastery_line(class).unwrap_or_else(|| panic!("{class:?}: no mastery line"));
            assert!(!seen.contains(&mastery), "{class:?}: two classes share a mastery line");
            seen.push(mastery);
            assert_eq!(PlayerClass::of_skill_line(mastery), None, "{class:?}");
        }
    }

    #[test]
    fn a_skill_line_belongs_to_one_class() {
        assert_eq!(PlayerClass::of_skill_line(line("Shadow")), Some(PlayerClass::Nightblade));
        assert_eq!(PlayerClass::of_skill_line(line("Storm Calling")), Some(PlayerClass::Sorcerer));
        assert_eq!(PlayerClass::of_skill_line(line("Mages Guild")), None);
        // no line is claimed by two classes
        let mut all: Vec<SkillLine> = PlayerClass::ALL.iter().flat_map(|c| c.skill_lines().iter().copied()).collect();
        let total = all.len();
        all.sort_by_key(|l| l.as_str());
        all.dedup();
        assert_eq!(all.len(), total);
        assert_eq!(total, 21);
    }

    #[test]
    fn a_swap_is_written_with_the_line_names() {
        let s = swap("Draconic Power", "Shadow");
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, r#"{"replaces":"Draconic Power","with":"Shadow"}"#);
        assert_eq!(serde_json::from_str::<SubclassSwap>(&json).unwrap(), s);
        assert!(serde_json::from_str::<SubclassSwap>(r#"{"replaces":"Draconic Power","with":"Wibble"}"#).is_err());
    }

    #[test]
    fn every_issue_can_be_said_in_words() {
        let issue = SubclassIssue::TwoLinesFromOneClass { class: PlayerClass::Nightblade, with: line("Siphoning") };
        assert_eq!(issue.to_string(), "Siphoning is a second skill line from Nightblade: only one line per other class");
        assert_eq!(SubclassIssue::TooManySwaps { count: 3 }.to_string(), "3 skill lines replaced, at most 2 can be");
    }
}
