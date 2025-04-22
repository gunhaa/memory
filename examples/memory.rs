fn main(){
    // 해당 코드는 컴파일 되지 않는다
    // 0x1이 가리키는 건 rust의 할당영역이 아니기에 유효하지 않다
    // error: process didn't exit successfully: `target\debug\examples\memory.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
    let mut n_nonzero = 0;

    for i in 1..10000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };
        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);

}