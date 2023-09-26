use async_std::task;

async fn do_work(id: u32) {
    for i in 1..=5 {
        println!("Task {}: Count {}", id, i);
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    }
}

fn main() {
    let task1 = task::spawn(do_work(1));
    let task2 = task::spawn(do_work(2));

    task::block_on(async {
        task1.await;
        task2.await;
    });

    println!("Main task: Done!");
}
