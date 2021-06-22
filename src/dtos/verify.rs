#[derive(Serialize, Deserialize)]
pub struct VerifyQuery {
    pub email: Option<String>,
}
