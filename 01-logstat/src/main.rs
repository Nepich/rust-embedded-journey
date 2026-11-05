fn main() {
    let res = logstat::kv::parse_kv("speed=42");
    println!("{res:?}");
}
