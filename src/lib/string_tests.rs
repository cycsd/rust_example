#[test]
pub fn to_chars_vec(){
    let s = "hello, world".to_string();
    let chars:Vec<char> = s.chars().collect();
    assert_eq!(chars,vec!['h', 'e', 'l', 'l', 'o', ',', ' ', 'w', 'o', 'r', 'l', 'd']);
}

#[test]
pub fn from_chars_collect(){
    let chars = vec!['h', 'e', 'l', 'l', 'o', ',', ' ', 'w', 'o', 'r', 'l', 'd'];
    let s:String = chars.iter().collect();
    assert_eq!(s,"hello, world")
}