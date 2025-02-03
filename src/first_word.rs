pub fn first_word() -> String {
    println!("Enter the word : ");

    let mut str = String::new();
    let mut ans = String::new();

    io::stdin().read_line(&mut str).expect("failed to read");
    println!("{str}");
    for i in str.trim().chars() {
        if i == ' ' {
            return ans;
        }
        ans.push(i);
    }

    return ans;
}

pub fn first_word_2(s: &String) -> usize {
    let bytes = s.as_bytes();
    // let temp = bytes.join("##");
    // println!("{temp}");

    for (i, &item) in bytes.iter().enumerate() {
        println!("for loop {i} item val = {item} ");
        if item == b' ' {
            return i;
        }
    }

    s.len()
}