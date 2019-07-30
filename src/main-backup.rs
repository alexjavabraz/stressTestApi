mod getBalance;
mod createAccount;
mod transactionCount;
mod totalAmount;
mod listAllTransactions;
mod createTransfer;
mod login;

#[macro_use]
extern crate serde_derive;
extern crate reqwest;
#[macro_use]
extern crate serde_json;

use std::thread;
use std::time;
use rand::Rng;
use reqwest::Client;
use std::time::{Duration, Instant};

static NTHREADS: i32 = 1000;

fn getBalance(i: i32) {
            getBalance::call_api_balance(i);
}

fn createAccount(i: i32) {
            createAccount::call_api(i);
}

fn total_transactions(i: i32) {
            transactionCount::call_api(i);
}

fn total_amount(i: i32) {
            totalAmount::call_api(i);
}

fn list_all_transactions(i: i32) {
            listAllTransactions::call_api_list_transactions(i);
}

fn create_transaction(i: i32) {
            createTransfer::callApi(i);
}

fn main() {
    let start = Instant::now();

    for i in 0..NTHREADS {
        let result = thread::spawn(move || {
            println!("Starting...{}", i);
            getBalance(i);
            createAccount(i);
            total_transactions(i);
            total_amount(i);
            list_all_transactions(i);
            create_transaction(i);
        });

        if i % 20 == 0 {
            thread::sleep(time::Duration::from_secs(5));
        }
    }

    

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);

}