pub const SPECIAL_CHARS: [char; 14] = [
    '!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-',
];

pub const NB_OF_CHARS: u8 = 76;

pub fn gen_all_chars(/*give config in order to enable special chars, or everythign else*/) -> Vec<char> {
    // Enough space to hold all the alphanumeric characters (upper and lower case)
    // as well as the 14 special characters defined in constants.rs
    let mut chars: Vec<char> = Vec::with_capacity(NB_OF_CHARS.into());

    chars.append(&mut SPECIAL_CHARS.to_vec());

    for i in 'A' as u8..'Z' as u8 + 1 {
        chars.push(i as char);
    }

    for i in 'a' as u8..'z' as u8 + 1 {
        chars.push(i as char);
    }

    for i in '0' as u8..'9' as u8 + 1 {
        chars.push(i as char);
    }

    return chars;
}