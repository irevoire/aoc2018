pub fn react(mut polymer: String) -> String {
    loop {
        let size = polymer.len();
        for a in 'a'..='z' {
            let b = a.to_ascii_uppercase();
            let ab = format!("{}{}", a, b);
            let ba = format!("{}{}", b, a);

            polymer = polymer.replace(&ab, "");
            polymer = polymer.replace(&ba, "");
        }
        if size == polymer.len() {
            break;
        }
    }
    polymer
}
