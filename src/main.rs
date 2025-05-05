

fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("{}", counter);
        if counter >=10{
            break counter*2;
        }
    };
    print!("Результат равен: {} \n", result);
    
    loop_while();
}


fn loop_while(){
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        print!("Значение массива: {} \n", element);
    }
}
