fn main() {
    let res = logstat::kv::parse_kv("speed=42");
    let top = logstat::words::top_words("a a a a a b b c c c d", 2);
    println!("{res:?}, top {top:?}");
}
