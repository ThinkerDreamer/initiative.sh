use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum MilitaryType {
    Barracks,
    Base,
    Castle,
    Citadel,
    Fort,
    Fortress,
    Keep,
    Stronghold,
    Tower,
}
