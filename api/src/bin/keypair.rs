fn main() {
    sodiumoxide::init().unwrap();

    let (public, secret) = sodiumoxide::crypto::box_::gen_keypair();

    println!(
        "{}",
        serde_json::to_string(&serde_json::json!({"public": public, "secret": secret})).unwrap()
    );
}
