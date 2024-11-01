use core_affinity;
use std::io::Error;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, Instant};

const CORE_AFFINITY_SLOT_TICKER: usize = 1;
const THREAD_PRIORITY_SLOT_TICKER: i32 = 99;

fn create_slot_ticker(slot_duration: Duration) -> Receiver<(u64, Instant, Instant)> {
    let (slot_ticker_sender, slot_ticker_receiver) = channel();

    thread::spawn(move || {
        let core_id = core_affinity::CoreId {
            id: CORE_AFFINITY_SLOT_TICKER,
        };

        if !core_affinity::set_for_current(core_id) {
            eprintln!("[Server] Failed to set core affinity for slot ticker");
        }

        #[cfg(target_os = "linux")]
        unsafe {
            let mut sched_param: libc::sched_param = std::mem::zeroed();
            sched_param.sched_priority = THREAD_PRIORITY_SLOT_TICKER;

            if libc::pthread_setschedparam(libc::pthread_self(), libc::SCHED_RR, &sched_param) != 0
            {
                eprintln!("Failed to set SCHED_RR. Error: {}", Error::last_os_error());
                return;
            }
        }

        let mut absolute_slot_number = 0;
        let mut last_tick = Instant::now();

        loop {
            let now = Instant::now();
            let expected_next_tick = last_tick + slot_duration;

            if now >= last_tick + slot_duration {
                if slot_ticker_sender
                    .send((absolute_slot_number, now, expected_next_tick))
                    .is_err()
                {
                    break;
                }
                last_tick = now;
                absolute_slot_number += 1;
                thread::sleep(slot_duration * 4 / 5);
            }
        }
    });

    slot_ticker_receiver
}

fn main() {
    let receiver = create_slot_ticker(Duration::from_micros(100));

    let mut jitters = Vec::new();

    while let Ok((slot_number, actual_time, expected_time)) = receiver.recv() {
        let jitter = if actual_time > expected_time {
            actual_time.duration_since(expected_time)
        } else {
            expected_time.duration_since(actual_time)
        };

        let jitter = jitter.as_nanos() as i64;

        jitters.push(jitter);
        if slot_number > 0 && slot_number % 100_000 == 0 {
            break;
        }
    }

    // Print detailed statistics
    println!("\nStatistics over 100,000 slots");
    let mean_jitter = if !jitters.is_empty() {
        jitters.iter().sum::<i64>() as f64 / jitters.len() as f64
    } else {
        0.0
    };
    println!("Mean jitter: {:.0} ns", mean_jitter);

    if let Some(&max) = jitters.iter().max() {
        println!("Max jitter: {} ns", max);
    }
    println!("Median jitter: {} ns", {
        let mut sorted = jitters.clone();
        sorted.sort_unstable();
        if sorted.is_empty() {
            0
        } else {
            sorted[sorted.len() / 2]
        }
    });

    // Calculate percentiles
    if !jitters.is_empty() {
        let mut sorted = jitters.clone();
        sorted.sort_unstable();
        let p95_idx = (sorted.len() as f64 * 0.95) as usize;
        let p99_idx = (sorted.len() as f64 * 0.99) as usize;
        println!("95th percentile: {} ns", sorted[p95_idx]);
        println!("99th percentile: {} ns", sorted[p99_idx]);
    }
}
