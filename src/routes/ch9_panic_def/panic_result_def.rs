use std::fs;
use std::io;
//use std::io::ErrorKind;

/*
Result<T, E>의 매개변수는 제네릭 타입 매개변수이다.
Err의 반환 타입이 io::Error인 이유는 File::open과 read_to_string 메서드가 io::Error 타입의 에러 값을 반환하기 때문이다.
*/
pub fn panic_result() -> Result<String, io::Error> {
    //let greeting_file_result = File::open("hello.txt");

    //File::open이 반환하는 Err배리언트 값의 타입은 io::Error인데, 이는 표준 라이브러리에서 제공하는 구조체입다. 이 구조체가 제공하는 kind메서드를 호출하여 io::ErrorKind값을 얻을 수 있다.
    // match 대신 unwrap_or_else 메서드를 사용할 수 있다.
    /*
    1. unwrap_or_else 사용
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file : {:?}", error);
            })
        }else {
            panic!("Problem opening the file : {:?}", error);
        }
    });

    2. 에러 전파하기 -> result로 반환
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    };

    3. ? 연산자
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    4. fs::read_to_string 사용
    */
    fs::read_to_string("hello.txt")
}