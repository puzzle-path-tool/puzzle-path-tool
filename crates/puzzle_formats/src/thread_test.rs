use std::{
    error::Error,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI32, Ordering},
    },
    thread,
};

#[derive(Default)]
struct CancelToken {
    is_cancelled: AtomicBool,
}

#[derive(Default)]
struct Status {
    progress: AtomicI32,
    max_progress: AtomicI32,
}

fn manage_work() -> Result<(), Box<dyn Error>> {
    let status = Arc::new(Status::default());
    let cancel_token = Arc::new(CancelToken::default());

    let t1 = {
        let status = status.clone();
        let cancel_token = cancel_token.clone();
        thread::spawn(move || {
            do_work(&status, &cancel_token);
        })
    };

    let t2 = {
        let status = status.clone();
        let cancel_token = cancel_token.clone();
        thread::spawn(move || {
            do_work(&status, &cancel_token);
        })
    };

    t1.join().map_err(|_| "")?;
    t2.join().map_err(|_| "")?;
    Ok(())
}

fn do_work(status: &Arc<Status>, cancel_token: &Arc<CancelToken>) {
    if cancel_token.is_cancelled.load(Ordering::Relaxed) {
        return;
    }
    status.progress.store(32, Ordering::Relaxed);
}
