pub fn brackets_are_balanced(string: &str) -> bool {
    // unimplemented!(
    //     "Check if the string \"{}\" contains balanced brackets",
    //     string
    // );
    let mut stack = Vec::new();
    for s in string.chars() {
        match (stack.last(), s) {
            (_, '(') | (_, '[') | (_, '{') => stack.push(s),
            (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') => {
                stack.pop();
            }
            (_, ')') | (_, ']') | (_, '}') => return false,
            _ => continue,
        }
    }
    stack.is_empty()
}
