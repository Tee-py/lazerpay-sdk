use uuid::Uuid;


pub fn generate_reference() -> String {
    let res = Uuid::new_v4();
    res.to_string()
}
