use std::time::Instant;

fn sum_baseline(data: &[u64]) -> u64 {
    let mut total = 0;
    for &v in data {
        total += v;
    }
    total
}

fn main() {
    let data: Vec<u64> = (0..200_000).collect();

    let t0 = Instant::now();
    let total = sum_baseline(&data);
    let elapsed = t0.elapsed();

    println!("total={total}");
    println!("elapsed_us={}", elapsed.as_micros());

    // 这个示例只是“可重复测量”的最小基线，不代表真实业务基准。
    // 章节重点是流程：先测量，再定位，再优化，再复测。
}
