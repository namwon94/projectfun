use std::io;
use std::cmp::Ordering;
use rand::Rng;

/*
turning_p[1]을 비교하는 이유
    -> std::env::args()로 가져오는 인자들의 벡터(Vec<String>)에서 실행 파일 자체의 이름(실행 바이너리 경로) 과 첫 번째 커맨드라인 인자를 가져오므로 
        첫 번쨰 커맨드라인 인자와 비교하고 싶으면 turning_p[1]로 비교해야 된다.
*/
fn main() {
    let turning_p: Vec<String> = std::env::args().collect();
    let turning_index = ["guessing", "var", "arr", "fun", "control"];
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
    }
    
    
    else {
        println!("Please input argument : {:?}", turning_index);
    }
}

fn guessing() {
   println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is {}", secret_number);

    loop{
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed : {guess}");

        //.cmp : 두 값을 비교하는 메서드
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}

fn var() {
    //mut 키워드를 사용해서 변수를 가변형으로 변경하는 방법
    let mut x = 5;
    println!("x의 값은 : {x}");
    x = 6;
    println!("x의 값은 : {x}");

    //let 키워드를 다시 사용해서 기존의 변수를 다시 사용
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("y의 값은(중괄호) : {y}");
    }
    println!("y의 값은 : {y}");

    //상수(무조건 불변)
    const THREE_HOURS_SECONDS: u32 = 60 * 60* 3;
    println!("THREE_HOURS_SECONDS : {THREE_HOURS_SECONDS}");
}

/*
이 예제는 러스트의 안전성 원칙이 동작하는 하나의 예입니다. 
많은 저수준 언어들에서는 이러한 검사가 이루어지지 않고, 여러분이 잘못된 인덱스를 제공하면 유효하지 않은 메모리에 접근이 가능한다. 
러스트는 이런 메모리 접근을 허용하고 계속 실행하는 대신 즉시 실행을 종료함으로써 이런 종류의 에러로부터 여러분을 보호한다.
*/
fn arr() {
    let a = [1, 2, 3, 4, 5];
    println!("Pleas enter an array index.");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is : {element}");
}

/*
함수는 호출한 코드에 값을 반환할 수 있다. 
반환되는 값을 명명해야 할 필요는 없지만, 그 값의 타입은 화살표 (->) 뒤에 선언되어야 한다. 
러스트에서 함수의 반환 값은 함수 본문의 마지막 표현식의 값과 동일하다. 
return 키워드와 값을 지정하여 함수로부터 일찍 값을 반환할 수 있지만, 대부분의 함수들은 암묵적으로 마지막 표현식 값을 반환한다.

해당 함수 내 check_num +1에 세미콜론 추가 시 표현식에서 구문으로 변경이 되어 컴파일 에러가 발생
    -> 주 에러 메시지 mismatched types는 이 코드의 핵심 문제를 보여준다.
*/
fn fun(check_num: i32) -> i32 {
    check_num + 1
}

fn control() {
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("for element in a : {element}" );
    }

    for number in 1..5 {
        println!("for number in (1..5) : {number}");
    }
}

/*
화씨 온도와 섭씨 온도 간 변환하기
n번째 피보나치 수 생성하기
크리스마스 캐롤 ‘The Twelve Days of Christmas’ 노래의 반복성을 활용하여 가사 출력해보기
*/