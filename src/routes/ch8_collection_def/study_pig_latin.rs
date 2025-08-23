/*
문자열을 피그 라틴(pig Latin)으로 변경해 보세여. 각 단어의 첫번째 자음은 단어의 끝으로 이동하고 'ay'를 붙이므로, 'first'는 'irst-fay'가 된다.
    모음으로 시작하는 단어는 대신 끝에 'hay'를 붙인다.('apple' -> 'apple-hay'). UTF-8 인코딩에 대한 세부 사항을 명심하기.
*/

pub fn pig_latin(first_word: char, word: String) {
    let str = word.trim();
    let f_word = first_word;
    //e_word: String -> type annotation이 필요한 이유 : collect는 반환타입이 generic으로 반환을 하는데 타입 추론에 실패한 경우 명확한 타입을 알려주기 위해 필요하다.
    let e_word: String = str.chars().skip(1).collect();
    let vowel = ['a', 'e', 'i', 'o', 'u'];

    let pig_latin_word = if vowel.contains(&f_word){
        format!("{}-hay", str)
    }else{
        format!("{}-{}ay", e_word, f_word)
    };
    println!("pig_latin_word : {:?}", pig_latin_word);
}

/*
Rust에서 문자열 리터럴('1hello' 같은 값)은 &str 타입이며, 이 참조의 라이프타임은 프로그램의 전체 수명('static)과 같다. 
즉, 해당 문자열 리터럴은 절대 drop되지 않고 프로그램이 끝날 때까지 메모리에서 살아 있다. 
이런 데이터는 &'static str으로 분류된다.
*/