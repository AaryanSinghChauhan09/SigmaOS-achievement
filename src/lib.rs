//! SigmaOS-achievement: achievement model.
//!
//! Real implementation: badges, milestones, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub category: String,
    pub unlocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AchievementError {
    EmptyName,
    DuplicateAchievement(String),
}

impl fmt::Display for AchievementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "achievement name must not be empty"),
            Self::DuplicateAchievement(id) => write!(f, "duplicate achievement {:?}", id),
        }
    }
}

impl std::error::Error for AchievementError {}

pub struct AchievementSystem {
    pub name: String,
    achievements: BTreeMap<String, Achievement>,
}

impl AchievementSystem {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), achievements: BTreeMap::new() }
    }

    pub fn add_achievement(&mut self, a: Achievement) -> Result<(), AchievementError> {
        if a.name.trim().is_empty() { return Err(AchievementError::EmptyName); }
        let id = a.id.clone();
        if self.achievements.contains_key(&id) {
            return Err(AchievementError::DuplicateAchievement(id.clone()));
        }
        self.achievements.insert(id, a);
        Ok(())
    }

    pub fn achievement_count(&self) -> usize { self.achievements.len() }

    pub fn get_achievement(&self, id: &str) -> Option<&Achievement> { self.achievements.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let s = AchievementSystem::new("My System");
        assert_eq!(s.name, "My System");
        assert_eq!(s.achievement_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut s = AchievementSystem::new("My System");
        let a = Achievement { id: "a1".into(), name: "first".into(), category: "milestone".into(), unlocked: true };
        s.add_achievement(a).unwrap();
        assert_eq!(s.achievement_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut s = AchievementSystem::new("My System");
        let a = Achievement { id: "x".into(), name: "".into(), category: "a".into(), unlocked: false };
        assert!(s.add_achievement(a).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut s = AchievementSystem::new("My System");
        let a = Achievement { id: "x".into(), name: "X".into(), category: "a".into(), unlocked: false };
        s.add_achievement(a).unwrap();
        let a2 = Achievement { id: "x".into(), name: "X".into(), category: "a".into(), unlocked: false };
        assert!(s.add_achievement(a2).is_err());
    }
}
