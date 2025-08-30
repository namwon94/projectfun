use std::env;
use std::fs;
use std::process;

/*
어떤 인수에라도 유효하지 않은 유니코드가 들어있다면 std::env::args가 패닉을 일으킬수 있다.
만약 프로그램이 유효하지 않은 유니코드를 포함하는 인수들을 받을 필요가 있다면 std::env::args_os를 대신 사용
*/
pub fn minigrep() {
    //collect는 타입표기가 자주 필요한 함수 중 하나(러스트가 원하는 종류의 컬렉션을 추론할 수 없음)
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    //Config::new에서 'new' 라는 함수는 절대 실패하지 않는다는 가정하에 사용 한다.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Porblem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}, In file {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("with text:/n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

/*
String 값을 소유한 Config를 정의했다. -> Config가 args의 값에 대한 소유권을 가져가려고 하면 러스트의 대여 규칙을 위반하게 됨.
String 데이터를 관리하는 방법 중 가장 쉬운방법은 clone 메서드를 호출하는 것이다.(다소 비효율적임)
    -> 데이터의 전제 복사본을 만들어 Config 인스턴스가 소유할 수 있게 해주는데 이는 문자열 데이터에 대한 참조자를 저장하는 것에 비해 더 많은 시간과 메모리를 소비한다.
    -> 참조자의 라이프타임을 관리할 필요가 없음 코드를 직관적으로 만들어 줌.

build 한수 성공 시 Config, 에러가 난 경우 &'static str을 갖는 Result를 반환
    -> 에러값은 언제나 'static 라이프타임을 갖는 문자열 리터럴이다.
*/
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

/*
cargo run minigrep -- needle haystack 명령어 실행시 나오는 결과 값
------------------------------------------------------------------------------------
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/projectfun minigrep -- needle haystack`
[src/routes/ch12_minigrep/minigrep.rs:10:5] args = [
    "target/debug/projectfun", -> 1. 바이너리 파일의 이름
    "minigrep",
    "--",
    "needle",
    "haystack",
]
------------------------------------------------------------------------------------
1. C에서의 인수 리스트의 동작과 일치하며, 프로그램이 실행 될 때 호출된 이름을 사용할 수 있게 해준다. 
    프로그램의 이름에 접근할 수 있다는 것은 메시지에 이름을 출력하고 싶을 때라던가 
    프로그램을 호출할 때 사용된 커맨드 라인 별칭이 무엇이었는지에 기반하여 프로그램의 동작을 바꾸고 싶을 때 이용
*/