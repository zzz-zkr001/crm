use crm::pb::User;
use prost::Message;

fn main() -> anyhow::Result<()> {
    let user = User::new(111, "zhaokeran", "zhaokeran@example.com");
    let encoded = user.encode_to_vec();
    println!("user: {:?}", encoded);
    let decoded = User::decode(&encoded[..])?;
    println!("decoded: {:?}", decoded);
    Ok(())
}
