use std::collections::HashMap;

fn main() {
    let map1: HashMap<String, u32> = createHashMap();
    let map2: HashMap<String, i32> = createHashMap();
}

fn createHashMap<T>() -> HashMap<String, T> {
    HashMap::new()
}
