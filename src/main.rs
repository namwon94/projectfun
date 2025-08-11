use std::io;
use std::cmp::Ordering;
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::str::FromStr;

/*
turning_p[1]을 비교하는 이유
    -> std::env::args()로 가져오는 인자들의 벡터(Vec<String>)에서 실행 파일 자체의 이름(실행 바이너리 경로) 과 첫 번째 커맨드라인 인자를 가져오므로 
        첫 번쨰 커맨드라인 인자와 비교하고 싶으면 turning_p[1]로 비교해야 된다.
*/
fn main() {
    let turning_p: Vec<String> = std::env::args().collect();
    let turning_index = ["guessing", "var", "arr", "fun", "control", "scope", "reference", "first_word", "chg_f_c"];
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
    }
    
    else if turning_p.len() > 1 && turning_p[1] == "chg_f_c" {
        chg_f_c();
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

fn chg_f_c() {
    println!("Which one do you want to change? Celsius or Fahrenheit");
    println!("Please input the word that you want to change");

    const D32: Decimal = dec!(32.0);
    const D5: Decimal = dec!(5.0);
    const D9: Decimal = dec!(9.0);
    let mut tem = String::new();

    io::stdin().read_line(&mut tem).expect("Failed to read line");

    if tem.trim() == "Celsius" {
        let mut cel = String::new();
        let fah: Decimal;

        loop {
            cel.clear();
            println!("Please input Celsius");

            io::stdin().read_line(&mut cel).expect("Failed to read line");

            let cel: Decimal = match Decimal::from_str(cel.trim()) {
                Ok(num) => num,
                Err(_) => {
                    println!("you have to input number. : {cel}"); 
                    continue
                }
            };
            fah = (cel*D9/D5+D32).round_dp(1);
            break;
        }
        
        println!("The Fahrenheit is {fah}°F");
    }else if tem.trim() == "Fahrenheit" {
        let mut fah = String::new();
        let cel: Decimal;

        loop {
            fah.clear();
            println!("Please input Fahrenheit");

            io::stdin().read_line(&mut fah).expect("Failed to read line");

            let fah: Decimal = match Decimal::from_str(fah.trim()) {
                Ok(num) => num,
                Err(_) => {
                    println!("you have to input number.");
                    continue
                }
            };
            cel = ((fah-D32)*D5/D9).round_dp(1);
            break;
        }

        println!("The Celsius is {cel}°C");

    }else{
        println!("You input the wrong word.")
    }

}

/*
화씨 온도와 섭씨 온도 간 변환하기
n번째 피보나치 수 생성하기
크리스마스 캐롤 ‘The Twelve Days of Christmas’ 노래의 반복성을 활용하여 가사 출력해보기
*/

fn scope() {
    /*
    String : 다른 문자열 타입 -> 이 타입은 힙에 할당된 데이터를 다룸 
        -> 컴파일 타임에 크기를 알 수 없는 텍스트도 저장할 수 있다. -> 실행 중 메모리 할당자로부터 메모리를 요청해야됨 / String 사용을 마쳤을 때 메모리를 해제할 방법이 필요
    */
    //이중 콜론(::) : 우리가 함수를 사용할 때 string_from 같은 함수명을 사용하지 않고 String 타입에 있는 특정된 from 함수라는 것을 지정할 수 있게 해주는 에미스페이스 연산자이다.
    let hello = String::from("hello"); //hello는 이 지점부터 유효

    println!("String::from을 사용 : {hello}"); //hello을 가지고 무언가 한다.

    /*
    s1에 저장된 내용은 포인터, 길이(String의 내용이 현재 사용하고 있는 메모리의 바이트 단위), 용량(메모리 할당자가 String에 할당한 메모리의 양) 이다.
        -> 해당 내용이 스택에 저장 문자열 내용은 힙 메모리에 저장
    s2에 s1을 대입하면 String 데이터가 복사된다. 이때 데이터는 스택에 있는 데이터 즉 포인터, 길이, 용량 값을 말하며 포인터가 가르키는 힙 영역의 데이터는 복사되지 않는다.
     */
    let s1 = String::from("hello");
    let s2 = s1;
    /*
    s2, s1이 스코프 밖으로 벗어날 때 각각 메모리를 해제하게 되면 중복 해제(double free)에러가 발생한다. -> 메모리 안정성 버그 중 하나이며 보안을 취약하게 만드는 메모리 손상의 원인
        -> 메모리 안정성을 보장하기 위해서는 let s2 = s1; 라인 뒤로는 s1이 더 이상 유효하지 않다고 판단 -> 해당 println!("{}, world!", s1); 는 에러가 발생한다.
    다른 언어에서는 '얕은복사(shallow copy)', '깊은 복사(deep copy)'를 러스트에서는 '이동(move)', 'clone' 이라는 공용 메서드를 사용
    */
    println!("이동(move) :  {s2}");

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {s3}, s4 = {s4}");
    /*
    clone 호출을 보고, 이 지점에서 성은ㅇ에 영향이 갈 수도 있는 코드가 실행될 것을 알 수 있다. clone은 해당 위치에서 무언가 다른 일이 수행될 것을 알려주는 시각적인 표시이기도 하다.
    */

    let x = 5;
    let y = x;
    
    println!("x : {x}, y : {y}");
    /*
    정수형 등 컴파일 타임에 크기가 고정되는 타입은 모두 스택에 저장되기 때문에 궅이 y를 생성하고 x를 무효화할 필요가 없다.
    */
} //이 스코프가 종료되고, hello는 더 이상 유효하지 않는다.
// 러스트는 변ㅅ가 스코프 밖으로 벗어나면 drop이라는 특별한 함수를 호출한다. 이 함수는 해당 타입을 개발한 개발자가 직접 메모리 해제 코드를 작성해 넣을 수 있게 되어있고,
// 위의 경우 String 개발자가 작성한 메모리 해제 코드가 실행된다. (drop은 닫힌 중괄호가 나타나는 지점에서 자동으로 호출)


fn reference() {
    let s1 = String::from("hello");
    //& : 참조자를 나타냄
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");
    /*
    참조자를 만드는 행위를 '대어(borrow)' 라고 한다. / 변수가 기본적으로 불변성을 지니듯 참조자도 마찬가지로 참조하는 것을 수정할 수 없다. 
        -> 가변 참조자는 똑같이 'mut'를 사용 
        -> 가변 참조자의 큰 제약사항 : 어떤 값에 대한 가변 참조자가 있다면, 그 값에 대한 참조자는 더 이상 만들 수 없다. 또한 어떤 값에 대한 불변 참조자가 있는 동안 같은 값의 가변 참조자를 만드는 것도 불가능
            여러 개의 불변 참조자를 만드는 것은 가능
    같은 데이터에 대하여 동시에 여러 가변 참조자의 사용을 막는 이러한 제약은 값의 변경에 대한 제어가 원활하도록 해준다. 대부분의 언어들이 언제든 값 변경을 허용하기 때문에, 갓 입문한 사람들은 장애물처럼 다가올 수 있다.
    하지만 러스트는 이 제약 덕분에 컴파일 타임에 데이터 경합(data race)을 방지할 수 있다.
    */
}

fn calculate_length(s: &String) -> usize { // s는 String의 참조자이다.
    s.len()
}// 여기서 s가 스코프 밖으로 벗어난다. 하지만 참조하는 것을 소유하고 있진 않으므로, 버려지지 않는다.

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    /*
    iter() : 컬렉션의 각 요소를 반환한다.
    enumerate() : iter의 각 결과값을 튜플로 감싸 반환한다. / 반환하는 튜플은 첫번째 요소가 인덱스, 두번째 요소가 해당 요소의 참조자로 이루어짐
     */
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}