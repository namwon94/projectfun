pub fn strings() {
    //모두 동일한 사용방법
    let s = String::new();

    let data = "initial contents";
    let s2 = data.to_string();

    let s3 = "initial contents".to_string();

    let s4 = String::from("initial contents");

    println!("s : {}, s2 : {}, s3 : {}, s4 : {}", s, s2, s3, s4);

    let mut t = String::from("foo");
    t.push_str("bar");

    println!("t : {}",t);

    //push_str 메서드는 문자열 슬라이스를 매개변수로 갖는데 이는 매개변수의 소유권을 가져올 필요가 없기 때문이다.
    let mut t2 = String::from("foo");
    let t3 = "bar";
    t2.push_str(t3);

    println!("t3 is {t3}");

    //push 메서드는 한 개의 글자를 매개변수로 받아서 String에 추가한다.
    let mut r = String::from("lo");
    r.push('l');

    println!("r : {}", r);

    let i = String::from("Hello, ");
    let i2 = String::from("world!");
    let i3 = i + &i2; //i은 여기로 이동되어 더 이상 사용할 수 없음을 주의

    println!("i3 : {}", i3);
    /*
    i이 더하기 연산 이후에 더 이상 유효하지 않은 이유와 i2의 참조자가 사용되는 이유
        -> + 연산자를 사용했을 때 호출되는 함수의 시그니터와 맞춰야 하기 때문이다. + 연산자는 add 메서드를 사용한다. fn add(self, s: &str) -> String{}
            -> String에는 &str만 더할 수 있고, 두 String끼리는 더하지 못한다. 하지만 &i2는 &String이지, &str은 아니다. 근데 어째서 컴파일 되는가?
                -> 러스트는 역참조 강제 변환(deref coercion)을 사용하는데 add 함수 내에서 사용되는 &i2를 &i2[..]로 바꾼다.
            -> add가 매개변수의 소유권을 가져가지는 않으므로 i2는 이 연산 이후에도 계속 유효한 String일 것이다.
    */

    let n = String::from("tic");
    let n2 = String::from("tac");
    let n3 = String::from("toc");
    let n4 = format!("{n}-{n2}-{n3}");
    //format! 매크로로 만들어진 코드는 참조자를 이용하므로 이 호출은 아무 매개변수의 소유권도 가져가지 않는다.

    println!("n4 : {}", n4);
}