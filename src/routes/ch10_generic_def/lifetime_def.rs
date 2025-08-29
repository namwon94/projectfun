pub fn lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("i.part : {}", i.part);
}

/*
이 함수를 정의하는 시점에서는 함수가 전달받을 구체적인 값을 알 수 없으니 if의 경우가 실행될지, else의 경우가 실행될지 알 수 없다.
전달받은 참조자의 구체적인 '라이프타임'도 알 수 없다.
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}