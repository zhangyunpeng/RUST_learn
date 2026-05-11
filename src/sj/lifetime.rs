/*
 * 无界生命周期
 */
fn f<'a, T>(x: *const T) -> &'a T {
    unsafe { &*x }
}
