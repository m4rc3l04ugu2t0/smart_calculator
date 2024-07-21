pub fn mount_order(input: &str) {
    let mut count = 0;
    let mut order = Vec::new();
    let mut tmp = Vec::new();

    while count < input.len() {
        for c in input.chars() {
            match c {
                '0'..='9' => {
                    if count > 1 {
                        order.push(c);
                    }
                    tmp.push(c);
                }
                '+' | '/' | '-' | '*' | '^' => {
                    if count > 1 {
                        order.push(c);
                    }
                    tmp.push(c);
                }
                '(' => {
                    order.push(c)
                    count += 1;
                }
                _ => {}
            }
        }
    }
}
