use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct EntityIdPayload {
    pub id: Uuid
}

pub type EntityIdResponse = EntityIdPayload;
