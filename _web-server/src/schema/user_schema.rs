use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A common User expression
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct User {
    /// User name
    #[schema(example = "Ryon Gosling")]
    pub username: String,
    /// User password used to auth
    #[schema(example = "I drive")]
    pub password: String,
}
