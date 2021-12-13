// [note] 实现线程池

// 线程池结构体
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Message>,
}

impl ThreadPool {

    // 实例化线程池
    pub fn new(size: usize) -> ThreadPool {

        // 线程池数量必须大于0 否则会panic
        assert!(size > 0);

        // 使用消息传递在线程间传送数据 （发送者(sender) 和 接收者(receiver)）
        let (sender, receiver) = std::sync::mpsc::channel();

        let receiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, std::sync::Arc::clone(&receiver)))
        }
        ThreadPool { workers, sender } 
    }

    pub fn execute<F>(&self, f: F)
    where
        // 泛型F的特征是FnOnce（函数只允许被执行一次）+ 可以跨线程边界传输的类型（含义上 Send 表示跨线程 move）+ 'static
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);

            // 无法移出,它在一个可变引用后面的“worker.thread”
            // worker是一个可变引用，但join方法需要它的参数的所有权来解决问题，解决方法是将线程移出worker实例，
            // worker.thread.join().unwrap();

            // 使用Option， 使用take方法如果返回None代表线程已经被清理
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            } 
        }
    }
}


type Job = Box<dyn FnOnce() + Send +'static>;

enum Message {
    NewJob(Job),
    Terminate
}

struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<Message>>>,
    ) -> Worker {
        let thread = std::thread::spawn(move|| loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            // job();
            match message {
                Message::NewJob(job) =>{
                    println!("worker {} get a job; executing", id);
                    job();
                },
                Message::Terminate => {
                    println!("worker {} was told to termainate", id);
                    break;
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}
