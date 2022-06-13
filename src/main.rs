use std::io;


fn add(list: Vec<&str>){
    let mut result=0;
    for number in list{
        result=result+number.parse::<i32>().unwrap();
    }
    println!("{}", result);
}

fn minus(list: Vec<&str>){
    let mut temp_list=list.clone();
    let mut initial=temp_list[0].parse::<i32>().unwrap();
    temp_list.remove(0);
    for number in temp_list{
        initial=initial-number.parse::<i32>().unwrap();
    }
    println!("{}", initial);
}

fn main() {
        let mut user_input=String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to get console input");
        let mut separated: Vec<&str> = user_input.split_whitespace().collect();
        let operation=String::from(separated[0]);
        separated.remove(0);
        match operation.as_ref() {
            "add"=>{
                add(separated)},
            "minus"=>{
                minus(separated)},
            _=>println!(""),
        }


}