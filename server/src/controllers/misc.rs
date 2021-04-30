use rocket::State;

#[get("/")]
pub fn bruh_moment() -> String {
    "Bruh moment".to_string()
}
