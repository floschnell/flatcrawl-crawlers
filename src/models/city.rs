use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum City {
  Munich,
  Wuerzburg,
  Augsburg,
  Kempten,
}
