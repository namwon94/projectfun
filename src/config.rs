use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/*
String 값을 소유한 Config를 정의했다. -> Config가 args의 값에 대한 소유권을 가져가려고 하면 러스트의 대여 규칙을 위반하게 됨.
String 데이터를 관리하는 방법 중 가장 쉬운방법은 clone 메서드를 호출하는 것이다.(다소 비효율적임)
    -> 데이터의 전제 복사본을 만들어 Config 인스턴스가 소유할 수 있게 해주는데 이는 문자열 데이터에 대한 참조자를 저장하는 것에 비해 더 많은 시간과 메모리를 소비한다.
    -> 참조자의 라이프타임을 관리할 필요가 없음 코드를 직관적으로 만들어 줌.

build 한수 성공 시 Config, 에러가 난 경우 &'static str을 갖는 Result를 반환
    -> 에러값은 언제나 'static 라이프타임을 갖는 문자열 리터럴이다.

20250903 수정 : ch13 - 반복자를 사용하여 clone 제거함. clone을 제거하는 이유는 위에 설명 했듯이 다소 비효율적이기 때문이다. 

env::args 함수에 대한 표준 라이브러리 문서에는 반환되는 반복자의 타입이 std::env:Args이며, 이 타입은 Iterator 트레이트를 구현하고 String 값을 반환함을 명시한다. 
*/
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text:/n{contents}");

    Ok(())
}