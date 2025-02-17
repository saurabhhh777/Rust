// use std::string;

fn main(){
    let greeting = String::from("Again OG Gang is here");
    let first_word  = find_the(&greeting);
    print!("First word: {}",first_word);

}

fn find_the(s:&String)-> String{
    let mut ans = String::new();
    for char in s.chars(){
        ans.push(char);
        if char == ' ' {
            break;
        }
        
    }
    return  ans;
}