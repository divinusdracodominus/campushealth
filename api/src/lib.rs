#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate err_derive;

pub mod code_test;

pub mod schema;
use schema::*;

use sha256::digest;

pub fn gen_hash_uuid(content: String) -> Uuid {
    Uuid::from_bytes_le(digest(content).as_bytes().try_into().unwrap())
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum ApiError {
    #[error(display = "missing event instance in deduplication")]
    MissingInstance,
}

use std::collections::HashMap;
use uuid::Uuid;

impl eventinstances {
    #[inline(always)]
    pub fn new(id: Uuid, event_id: Uuid, begin: i64, end: i64) -> Self {
        Self {
            id,
            event_id,
            begin,
            duration: Some(end - begin),
            end,
        }
    }
    #[inline(always)]
    pub fn create_new(event_id: Uuid, begin: i64, end: i64) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_id,
            begin,
            duration: Some(end - begin),
            end,
        }
    }
}

impl events {
    /// ## Purpose
    /// this function matches events with the same title in the same calendar, and removes
    /// the duplicated events making them into instances instead
    ///
    /// ## Returns
    /// (HashMap<title, events>, HashMap<event_id, eventinstance>)
    pub fn deduplicate(
        events: Vec<events>,
        instance_list: HashMap<Uuid, eventinstances>,
    ) -> Result<(HashMap<String, events>, HashMap<Uuid, eventinstances>), ApiError> {
        let mut tempset: HashMap<String, events> = HashMap::new();
        let mut instances: HashMap<Uuid, eventinstances> = HashMap::new();
        for event in events.into_iter() {
            if let Some(event_val) = tempset.get(&event.title) {
                let instance = match instance_list.get(&event.id) {
                    Some(instance) => instance,
                    None => return Err(ApiError::MissingInstance),
                };
                instances.insert(
                    event.id.clone(),
                    eventinstances::create_new(instance.id.clone(), instance.begin, instance.end),
                );
            } else {
                tempset.insert(event.title.clone(), event);
            }
        }
        Ok((tempset, instances))
    }
}

impl calendarentry {
    pub fn gen_hash_uuid(&self) -> Uuid {
        let content = match &self.description {
            Some(description) => format!("{}{}", self.title, description),
            None => self.title.clone(),
        };
        gen_hash_uuid(content)
    }
}

impl From<calendarentry> for events {
    fn from(entry: calendarentry) -> events {
        events {
            id: entry.gen_hash_uuid(),
            title: entry.title,
            description: entry.description,
            color: None,
            days_of_week: None,
            image_description: None,
            image_url: None,
            repeats: None,
            organizer: None,
            repitition: None,
        }
    }
}
