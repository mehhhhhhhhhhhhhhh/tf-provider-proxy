use std::path::PathBuf;
//use std::thread;
use clap::Parser;

use tf_provider_proxy::provider::{Client, get_provider_schema};

#[derive(Parser, Debug)]
struct Args {
    subprocess: PathBuf,
}

#[tokio::main]
async fn main() {
    ctrlc::set_handler(move || {
        eprintln!("received interrupt!");
        // if we don't catch this, client doesn't get dropped properly on exit
    }).unwrap();

    let args = Args::parse();
    let client = Client::with_binary(&args.subprocess).await;
    let mut client2 = client.client;
    // TODO figure out how to automatically retain (not drop) process while client is retained

    let req = tonic::Request::new(get_provider_schema::Request {});
    let res = client2.get_schema(req).await.unwrap().into_inner();
    //eprintln!("{:?}", res);
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
    //drop(child_g);

    //let run_on = thread::spawn(move || {
    //    loop {
    //        let mut buf = String::new();
    //        let res = output.read_line(&mut buf);
    //        print!("{}", buf);
    //        if let Ok(amt) = res {
    //            if amt == 0 {
    //                break;
    //            }
    //        }
    //    }
    //    //process::exit(junk.wait().unwrap().code().unwrap());
    //    //TODO pukes about partial move
    //});
    //run_on.join().unwrap();
}
