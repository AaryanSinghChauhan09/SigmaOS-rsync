// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-rsync: rsync component model.
//!
//! Real implementation: rsync_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsyncItem {
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
pub enum RsyncError {
    EmptyRsyncItemName,
    DuplicateRsyncItem(String),
    EmptyEventSubject,
}

impl fmt::Display for RsyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRsyncItemName => write!(f, "rsync_item name must not be empty"),
            Self::DuplicateRsyncItem(id) => write!(f, "duplicate rsync_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RsyncError {}

pub struct Rsync {
    pub name: String,
    rsync_items: BTreeMap<String, RsyncItem>,
    events: BTreeMap<String, Event>,
}

impl Rsync {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rsync_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_rsync_item(&mut self, item: RsyncItem) -> Result<(), RsyncError> {
        if item.name.trim().is_empty() {
            return Err(RsyncError::EmptyRsyncItemName);
        }
        let id = item.id.clone();
        if self.rsync_items.contains_key(&id) {
            return Err(RsyncError::DuplicateRsyncItem(id.clone()));
        }
        self.rsync_items.insert(id, item);
        Ok(())
    }

    pub fn rsync_item_count(&self) -> usize {
        self.rsync_items.len()
    }

    pub fn get_rsync_item(&self, id: &str) -> Option<&RsyncItem> {
        self.rsync_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RsyncError> {
        if event.subject.trim().is_empty() {
            return Err(RsyncError::EmptyEventSubject);
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

    fn sample_rsync_item(id: &str, name: &str) -> RsyncItem {
        RsyncItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Rsync::new("Rsync");
        assert_eq!(m.name, "Rsync");
        assert_eq!(m.rsync_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_rsync_item() {
        let mut m = Rsync::new("Rsync");
        m.add_rsync_item(sample_rsync_item("a1", "Alpha")).unwrap();
        assert_eq!(m.rsync_item_count(), 1);
        assert!(m.get_rsync_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Rsync::new("Rsync");
        assert!(m.add_rsync_item(sample_rsync_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Rsync::new("Rsync");
        m.add_rsync_item(sample_rsync_item("x", "One")).unwrap();
        assert!(m.add_rsync_item(sample_rsync_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Rsync::new("Rsync");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
