use std::env;
use std::process;
use crate::config::{Config, run};

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
        eprintln!("Porblem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}, In file {}", config.query, config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error:{e}");
        process::exit(1);
    }; 
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