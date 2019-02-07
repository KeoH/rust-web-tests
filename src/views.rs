use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
pub struct LoginAttemp {
    username: String,
    password: String
}


#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}


#[post("/post", format="json", data="<loginattemp>")]
pub fn post(loginattemp: Json<LoginAttemp>) -> JsonValue {
    if loginattemp.username == "pedrito" {
        json!({ "status": "ok", "username": loginattemp.username })
    } else {
        json!({
            "status": "error",
            "reason": "User not found"
        })
}
}