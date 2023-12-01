use std::time::{Duration, SystemTime};

pub fn run_day<F>(title: &str, day: i32, part: i32, actions: Vec<F>)
where
    F: Fn() -> String,
{
    println!("\n### Day {} - {}: Part {}", day, title, part);

    let (res, dur) = bench(&actions[0]);
    println!("Test: {:?}, {:?}", res, dur);

    let (res, dur) = bench(&actions[1]);
    println!("Main: {:?}, {:?}", res, dur);
}

fn bench<F>(action: F) -> (String, Duration)
where
    F: Fn() -> String,
{
    let start = SystemTime::now();
    let res = action();
    return (res, SystemTime::now().duration_since(start).unwrap());
}
