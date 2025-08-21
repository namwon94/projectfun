use std::collections::HashMap;

/*
해시맵은 세 가지 일반적인 컬렉션 중 덜 사용됨 그래서 프렐루드의 자동으로 가져오는 기능에는 포함되지 않는다. 또한 표준 라이브러리로부터의 지원을 덜 받는다. 예를들어 해시맵을 생성하는 기본 매크로가 없다.
*/
pub fn hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score : {}", score);

    for (key, value) in &scores {
        println!("{key} : {value}");
    }

    //i32 처럼 Copy 트레이트를 구현한 타입의 값은 해시맵 안으로 복사된다. String처럼 소유권이 있는 값의 경우, 해시맵이 그 값의 소유자가 된다.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //field_name, field_value는 이 시점부터 유효하지 않는다. 사용 시도후 무슨 컴파일러 에러가 발생하는지 알아보자
    //해시맵에 값들의 참조자들을 삽입한다면, 이 값들은 해시맵으로 이동되지 않을 것이다. 하지만 참조자가 가리키고 있는 해시맵이 유효할 때까지 계속 유효해야 한다.
    //println!("field_name : {field_name}, field_value : {field_value}");

    //entry - API 사용
    scores.entry(String::from("Red")).or_insert(40);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("scores : {:?}", scores);

    let text = "hello world wonderful world";
    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map2);
    //or_insert 메서드는 실제로는 해당 키에 대한 값의 가변 참조자(&mut V)를 반환한다. 여기서는 count변수에 가변 참조자를 저장하였고, 여기에 값을 할당하기 위해 먼저 *를 사용하여 count에 대한 역참조 해야됨.
}