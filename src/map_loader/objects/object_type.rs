#[derive(Debug, Deserialize, Serialize)]
pub enum ObjectType {
    #[serde(rename = "")]
    None,
    Player,
    Enemy,
}
