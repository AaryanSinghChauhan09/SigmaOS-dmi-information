// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-dmi-information: dmi information component model.
//!
//! Real implementation: informations, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Information {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmiInformationError {
    EmptyInformationName,
    DuplicateInformation(String),
    EmptyEventSubject,
}

impl fmt::Display for DmiInformationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInformationName => write!(f, "information name must not be empty"),
            Self::DuplicateInformation(id) => write!(f, "duplicate information {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DmiInformationError {}

pub struct DmiInformation {
    pub name: String,
    informations: BTreeMap<String, Information>,
    events: BTreeMap<String, Event>,
}

impl DmiInformation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            informations: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_information(&mut self, item: Information) -> Result<(), DmiInformationError> {
        if item.name.trim().is_empty() {
            return Err(DmiInformationError::EmptyInformationName);
        }
        let id = item.id.clone();
        if self.informations.contains_key(&id) {
            return Err(DmiInformationError::DuplicateInformation(id.clone()));
        }
        self.informations.insert(id, item);
        Ok(())
    }

    pub fn information_count(&self) -> usize {
        self.informations.len()
    }

    pub fn get_information(&self, id: &str) -> Option<&Information> {
        self.informations.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DmiInformationError> {
        if event.subject.trim().is_empty() {
            return Err(DmiInformationError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_information(id: &str, name: &str) -> Information {
        Information {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DmiInformation::new("DmiInformation");
        assert_eq!(m.name, "DmiInformation");
        assert_eq!(m.information_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_information() {
        let mut m = DmiInformation::new("DmiInformation");
        m.add_information(sample_information("a1", "Alpha")).unwrap();
        assert_eq!(m.information_count(), 1);
        assert!(m.get_information("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DmiInformation::new("DmiInformation");
        assert!(m.add_information(sample_information("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DmiInformation::new("DmiInformation");
        m.add_information(sample_information("x", "One")).unwrap();
        assert!(m.add_information(sample_information("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DmiInformation::new("DmiInformation");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
