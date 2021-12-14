use actix::prelude::*;
use futures::future;
use futures::future::Future;
use std::time::Duration;
use std::time::Instant;
use tokio::timer::Delay;

// use std::time::Duration;

// fn sync_send() {
//    let (tx,rx) = mpsc::sync_channel(1);
//    let tx_clone = tx.clone();

//    let _ = tx.send(0);

//    thread::spawn(move|| {
//       let _ = tx.send(1);
//    });

//    let _ = tx_clone.send(2);

//    println!("Received: {}", rx.recv().unwrap());
//    println!("Received: {}", rx.recv().unwrap());
//    println!("Received: {}", rx.recv().unwrap());

//    println!("Received: {:?}", rx.recv());
// }

struct Add(u32, u32);

impl Message for Add {
   type Result = Result<u32, ()>;
}

struct Adder;

impl Actor for Adder {
   type Context = SyncContext<Self>;
}

impl Handler<Add> for Adder {
   type Result = Result<u32, ()>;

   fn handle(&mut self, msg: Add, _: &mut Self::Context) -> Self::Result {
      let sum = msg.0 + msg.1;
      println!("Computed: {} + {} = {}", msg.0, msg.1, sum);
      Ok(msg.0 + msg.1)
   }
}

fn main() {

   System::run(|| {
      let addr = SyncArbiter::start(3, || Adder);
      for n in 5..10{
         addr.do_send(Add(n, n+1));
      }

      tokio::spawn(futures::lazy(||{
         Delay::new(Instant::now() + Duration::from_secs(1)).then(|_| {
            System::current().stop();
            future::ok::<(),()>(())
         })
      }));
   });

   // sync_send();

   // let (tx,rx) = mpsc::sync_channel(1);
   // let (tx, rx) = mpsc::sync_channel(2);
   // let tx_clone = tx.clone();
   // let tx_clone2 = tx.clone();

   // let _ = tx.send(10);

   // thread::spawn(move|| {
   //    // thread::sleep(Duration::from_secs(1));
   //    let _ = tx_clone2.send(1);
   // });
   // let _ = tx.send(5);

   // thread::spawn(move || {
   //    let _ = tx_clone.send(2);
   // });

   // thread::spawn(move|| {
   //    // thread::sleep(Duration::from_secs(1));
   //    let _ = tx.send(3);
   // });

   // println!("Received: {}", rx.recv().unwrap());
   // println!("Received: {}", rx.recv().unwrap());
   // println!("Received: {}", rx.recv().unwrap());

   // println!("Received: {}", rx.recv().unwrap());
   // println!("Received: {:?}", rx.recv());
   // let (tx, rx) = mpsc::channel();

   // let root_name = "Summer_";

   // let mut full_name = vec![];

   // let join_handle = thread::spawn(move || {
   //    while let Ok(v) = rx.recv(){

   //       full_name.push(v);
   //       // println!("Received: {}", v);
   //    }
   // });
   // for i in 0..10 {
   //    thread::sleep(Duration::from_secs(1));
   //    tx.send(format!("{}{}", root_name,i)).unwrap();
   // }

   // join_handle.join().unwrap();
   // let root_name = "Summer_";

   // for i in 0..10 {
   //    let (tx, rx) = mpsc::channel();
   //    let name = format!("name_{}", i);
   //    thread::spawn(move || {
   //       thread::sleep(Duration::from_secs(1));
   //       tx.send(format!("{}{}", root_name,name)).unwrap();
   //    });

   //    let v = rx.recv().unwrap();
   //    println!("Received: {}", v);
   // }

   // join_handle.join().unwrap();

   println!("Hello world");
}
