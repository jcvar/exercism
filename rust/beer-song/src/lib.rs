pub fn verse(n: u32) -> String {
    // unimplemented!("emit verse {}", n)
    let mut v = String::new();
    if n == 0 {
        v += "No more bottles of beer on the wall, no more bottles of beer.\n";
        v += "Go to the store and buy some more, 99 bottles of beer on the wall.\n";
    } else {
        let b = bottles(n);
        v += &format!("{} of beer on the wall, {} of beer.\n", b, b);
        v += &format!(
            "Take {} down and pass it around, {} of beer on the wall.\n",
            if n == 1 { "it" } else { "one" },
            bottles(n - 1)
        );
    }
    v
}

pub fn bottles(n: u32) -> String {
    match n {
        0 => format!("no more bottles"),
        1 => format!("1 bottle"),
        _ => format!("{} bottles", n),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    // unimplemented!("sing verses {} to {}, inclusive", start, end)
    let mut bottle = start;
    let mut out = String::new();
    while bottle > end {
        out += &verse(bottle);
        out += "\n";
        bottle -= 1;
    }
    out += &verse(bottle);
    out
}
