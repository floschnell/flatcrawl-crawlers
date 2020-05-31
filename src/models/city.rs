use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum City {
  Lindenberg,
  Munich,
  Wuerzburg,
  Augsburg,
  Kempten,
}
