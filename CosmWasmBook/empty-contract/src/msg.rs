// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// pub struct GreetResp {
//     pub message: String,
// }

// #[derive(Serialize, Deserialize)]
// pub enum QueryMsg {
//     Greet {},
// }

// ###################################################################3

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Greet {},
}
