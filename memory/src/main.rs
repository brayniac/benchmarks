use rustcommon_waterfall::*;
use rustcommon_heatmap::Nanoseconds;
// use rustcommon_heatmap::Duration;
use rustcommon_heatmap::Heatmap;
// use core::time::Duration;
// use std::time::Instant;

const KB: usize = 1024;
const MB: usize = 1024 * KB;
// const GB: usize = 1024 * MB;

type Duration = rustcommon_heatmap::Duration<Nanoseconds<u64>>;
type Instant = rustcommon_heatmap::Instant<Nanoseconds<u64>>;

fn main() {
    println!("Hello, world!");

    let mut heatmap = Heatmap::<u64, u64>::new(1_000_000_000, 3, Duration::from_secs(300), Duration::from_millis(100));

    let stop = Instant::now() + Duration::from_secs(300);

    loop {
        let start = Instant::now();

        if start > stop {
            break;
        }

        copy(MB);

        let elapsed = start.elapsed().as_nanos() as u64;

        // println!("elapsed: {}", elapsed);
        heatmap.increment(start, elapsed, MB as u64);

    }

    WaterfallBuilder::new("waterfall.png")
        .label(100, "100ns")
        .label(1000, "1us")
        .label(10000, "10us")
        .label(100000, "100us")
        .label(1000000, "1ms")
        .label(10000000, "10ms")
        .label(100000000, "100ms")
        .scale(Scale::Logarithmic)
        .palette(Palette::Ironbow)
        .build(&heatmap);

    // let mut size = KB;

    // while size < MB {
    //     let rate = copy(size);
    //     println!("size: {} KB rate: {:.02} MB/s", size / KB, rate);
    //     size *= 2;
    // }

    // while size <= GB {
    //     let rate = copy(size);
    //     println!("size: {} MB rate: {:.02} MB/s", size / MB, rate);
    //     size *= 2;
    // }
}

// allocate two regions of the given size, copy data from one to the other, return the
// rate in MB/s
fn copy(size: usize) {
    let region = vec![0; size * 2];

    let mut region = region.into_boxed_slice();

    for i in 0..size * 2 {
        region[i] = 0;
    }

    let a_ptr = region.as_ptr();
    let b_ptr = unsafe { region.as_mut_ptr().add(size) };

    unsafe {
        std::ptr::copy_nonoverlapping(a_ptr, b_ptr, size);
    }
}