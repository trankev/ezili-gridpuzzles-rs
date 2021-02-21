use crate::states;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub tokensets: Vec<states::Tokenset>,
}
