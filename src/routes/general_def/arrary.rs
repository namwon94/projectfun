use std::io;

/*
이 예제는 러스트의 안전성 원칙이 동작하는 하나의 예입니다. 
많은 저수준 언어들에서는 이러한 검사가 이루어지지 않고, 여러분이 잘못된 인덱스를 제공하면 유효하지 않은 메모리에 접근이 가능한다. 
러스트는 이런 메모리 접근을 허용하고 계속 실행하는 대신 즉시 실행을 종료함으로써 이런 종류의 에러로부터 여러분을 보호한다.
*/
pub fn arr() {
    let a = [1, 2, 3, 4, 5];
    println!("Pleas enter an array index.");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is : {element}");
}