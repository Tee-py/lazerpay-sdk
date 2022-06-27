use uuid::Uuid;

/// Generates Transaction reference for payments.
pub fn generate_reference() -> String {
    let res = Uuid::new_v4();
    res.to_string()
}
