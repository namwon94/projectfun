pub fn var() {
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