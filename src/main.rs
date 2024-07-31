extern crate rayon;

use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn check_password(candidate: &str, target_hash: &str) -> bool {
    let candidate_hash = format!("{:x}", md5::compute(candidate));
    candidate_hash == target_hash
}

fn main() {
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let target_hash = "5d41402abc4b2a76b9719d911017c592"; // hash for "hello"
    let found = Arc::new(Mutex::new(None));

    (1..6).into_par_iter().for_each(|length| {
        let found = Arc::clone(&found);
        let charset = charset.chars().collect::<Vec<_>>();
        let mut combination = vec![charset[0]; length];

        loop {
            let candidate = combination.iter().collect::<String>();
            if check_password(&candidate, &target_hash) {
                *found.lock().unwrap() = Some(candidate);
                break;
            }
            let mut i = length;
            while i > 0 {
                i -= 1;
                combination[i] = charset[(charset.iter().position(|&c| c == combination[i]).unwrap() + 1) % charset.len()];
                if combination[i] != charset[0] {
                    break;
                }
            }
            if combination.iter().all(|&c| c == charset[0]) {
                break;
            }
        }
    });

    if let Some(password) = &*found.lock().unwrap() {
        println!("Password found: {}", password);
    } else {
        println!("Password not found");
    }
}
