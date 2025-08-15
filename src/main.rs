use std::{io, process};
use projectfun::routes::{
    guessing, var, arr, fun, control, chg_f_c, fibonacci, fibonacci_memo, scope, reference,
    first_word, rectangles, enums,
};

/*
화씨 온도와 섭씨 온도 간 변환하기 (clear)
n번째 피보나치 수 생성하기 (clear)
크리스마스 캐롤 ‘The Twelve Days of Christmas’ 노래의 반복성을 활용하여 가사 출력해보기
*/

/*
turning_p[1]을 비교하는 이유
    -> std::env::args()로 가져오는 인자들의 벡터(Vec<String>)에서 실행 파일 자체의 이름(실행 바이너리 경로) 과 첫 번째 커맨드라인 인자를 가져오므로 
        첫 번쨰 커맨드라인 인자와 비교하고 싶으면 turning_p[1]로 비교해야 된다.
*/
fn main() {
    let turning_p: Vec<String> = std::env::args().collect();
    let turning_index 
        = ["guessing", "var", "arr", "fun", "control", "scope", "reference", "first_word", "chg_f_c", "rectangles", "fibonacci", "fibonacci_memo", "enums"];
    if turning_p.len() > 1 && turning_p[1] == "guessing" {
        guessing();
    }else if turning_p.len() > 1 && turning_p[1] == "var" {
        var();
    }else if turning_p.len() > 1 && turning_p[1] == "arr" {
        arr();
    }else if turning_p.len() > 1 && turning_p[1] == "fun" {
        let check_num = fun(5);

        println!("check_num : {check_num}");
    }else if turning_p.len() > 1 && turning_p[1] == "control" {
        control();
    }else if turning_p.len() > 1 && turning_p[1] == "scope" {
        scope();
    }else if turning_p.len() > 1 && turning_p[1] == "reference" {
        reference();
    }else if turning_p.len() > 1 && turning_p[1] == "first_word" {
        let mut s = String::from("hello world");
        let word = first_word(&s);

        s.clear(); // 이 코드는 String을 비워서 ""으로 만든다.

        //여기서 word에는 여전히 5가 들어있지만, 이 5를 의미있게 쓸 수 있는 문자열은 더 이상 없다. word는 이제 전혀 유효하지 않다.
        println!("word : {word}, s : {s}");
    }else if turning_p.len() > 1 && turning_p[1] == "rectangles" {
        rectangles();
    }else if turning_p.len() > 1 && turning_p[1] == "enums" {
        enums();
    }
    
    else if turning_p.len() > 1 && turning_p[1] == "chg_f_c" {
        chg_f_c();
    }else if turning_p.len() > 1 && turning_p[1] == "fibonacci" {
        println!("Please input a number");

        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("Faile to read line");

        let number:u32 = match number.trim().parse(){
        Ok(num) => num,
        Err(_) => {
                println!("you are not input number");
                process::exit(1)
            }
        };

        for i in 0..number {
            println!("fibonacci({}) = {}", i, fibonacci(i));
        }

    }else if turning_p.len() > 1 && turning_p[1] == "fibonacci_memo" {
        println!("Please input a number");

        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("Faile to read line");

        let number:usize = match number.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("you are not input number");
            process::exit(1)
            }
        };

        let mut memo = vec![0; number+1];

        for i in 0..number {
            println!("fibonacci({}) = {}", i, fibonacci_memo(i, &mut memo));
        }
    }
    /*else if turning_p.len() > 1 && turning_p[1] == "struct_user" {
        user();
    }*/
    else {
        println!("Please input argument : {:?}", turning_index);
    }
}

/*
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn user() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // user1 과 동일한 값은 이런식으로 ..user1으로 표기하는 방벙이 있다.
    };
    // user2를 생선한 이후에는 user1을 더 이상 사용할 수 없다 -> user1의 username 필드의 String이 user2로 이동되기 때문이다. -> username은 힙에 저장된 데이터 이기 때문이다.

    user1.email = String::from("anotheremail@example.com");
}
// 필드 초기화 축약법(field init shorthand) : 매개변수명과 구조체 필드명이 동일한 경우 반복해서 작성하는 대신 작성을 안한다.

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count:1,
    }
}
*/