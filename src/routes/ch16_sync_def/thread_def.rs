use std::thread;
use std::time::Duration;
use std::sync::{Mutex, mpsc, Arc};
//use std::rc::Rc;

/*
생성된 스레드가 실행죄디 않거나, 전부 실행되지 않는 문제는 thread::spawn의 반환 값을 변수에 저장함으로써 해결할 수 있다. 반환타입은 JoinHandle이다. 
JoinHandle : 자신의 join메서드를 호출 했을때 그 스레드가 끝날 때까지 기다리는 소유값 -> 핸들에 대한 스레드가 종료될 때까지 현재 실행 중인 스레드를 '블록'한다.
    -> 블록의 의미 : 그 스레드의 작업을 수행하거나 종료되는 것이 방지된다는 뜻
    -> handle.join()의 호출 위치에 따라 스레드가 동시에 실행되는지의 여부에 영향을 미친다. 
*/
pub fn spawn() {
    //스레드 예제
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    //스레드에 move 클로저 사용예제
    let v = vec![1, 2, 3];
    //러스트는 생성된 스레드가 얼마나 오랫동안 실행될지 알 수 없으므로, v에 대한 참조자가 항상 유효한지 알지 못함. 그래서 move 클로저 사용해야됨.
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();
}

//소유권 규칙은 메시지 전송에서 안전하면서 동시적인 코드를 작성하는데 중요한 역할을 한다.
pub fn channel() {
    //mpsc는 복수생산자, 단일소비자의 약어이다. 
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

        thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    /*
    수신자는 두 개의 유용한 메서드가 있다. recv (메인 스레드의 실행을 블록시키고 채널로부터 값을 받을 때까지 기다린다) / try_recv (블록하지 않고 전달받은 메시지가 있으면 Ok값을 지금 시점에 없으면 Err값을 반환한다)
     */
    //let received = rx.recv().unwrap();
    for received in rx {
        println!("Got : {}", received);
    }
}

/*
Mutex<T>는 스마트 포인터이다. -> lock의 호출이 MutexGuard라느 스마트 포인터를 반환하는데 unwrap호출을 통해 처리되는 LockResult로 감싸져 있다.
    -> MutextGuard는 Deref 와 Drop 구현체가 있다. -> 내부 가변성 제공 -> 데드락(deadlock)을 생성할 위험성이 있다.

Rc<T>는 스레드를 교차하면서 공유하기에는 안전하지 않다. -> Arc<T>가 동시적 상황에서 안전하게 사용할 수 있는 Rc<T>와 같은 타입이다.

*/
pub fn mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}