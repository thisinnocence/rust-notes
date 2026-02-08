#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pair {
    pub a: u32,
    pub b: u32,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sum_pair(pair: *const Pair, out: *mut u32) -> i32 {
    if pair.is_null() || out.is_null() {
        return -1;
    }

    // SAFETY:
    // 1) 上面已检查 pair 非空。
    // 2) 调用方承诺 pair 指向有效 Pair。
    let p = unsafe { *pair };

    // SAFETY:
    // 1) 上面已检查 out 非空。
    // 2) 调用方承诺 out 指向可写 u32。
    unsafe { *out = p.a + p.b };

    0
}

fn main() {
    let pair = Pair { a: 40, b: 2 };
    let mut out = 0_u32;

    // SAFETY:
    // 1) pair 和 out 都是当前栈上的有效对象。
    // 2) 指针在本次调用期间保持有效。
    let rc = unsafe { sum_pair(&pair as *const Pair, &mut out as *mut u32) };
    println!("ffi rc={rc}, out={out}");
}

#[cfg(test)]
mod tests {
    use super::{sum_pair, Pair};

    #[test]
    fn test_sum_pair_ok() {
        let pair = Pair { a: 10, b: 32 };
        let mut out = 0_u32;
        // SAFETY: pair/out 都是有效指针。
        let rc = unsafe { sum_pair(&pair as *const Pair, &mut out as *mut u32) };
        assert_eq!(rc, 0);
        assert_eq!(out, 42);
    }

    #[test]
    fn test_sum_pair_null() {
        let mut out = 0_u32;
        // SAFETY: 传空指针是为了验证错误返回码分支。
        let rc = unsafe { sum_pair(std::ptr::null(), &mut out as *mut u32) };
        assert_eq!(rc, -1);
    }
}
