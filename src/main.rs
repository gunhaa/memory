use kernel32;
use winapi::{self, SYSTEM_INFO};

use winapi::{
    DWORD,
    HANDLE,
    LPVOID,
    PVOID,
    SIZE_T,
    LPSYSTEM_INFO,
    MEMORY_BASIC_INFORMATION as MEMINFO,
};
//  가상 메모리 레이아웃
fn main() {

    let this_pid: DWORD;
    let this_proc: HANDLE;
                      // 롱포인터
    let min_app_addr: LPVOID;
    let max_app_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEMINFO;

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMINFO>();

    // memory initialize
    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    // system call
    unsafe {
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        kernel32::GetSystemInfo(
            &mut proc_info as LPSYSTEM_INFO
        );
    }

    min_app_addr = proc_info.lpMinimumApplicationAddress;
    max_app_addr = proc_info.lpMaximumApplicationAddress;

    // 현재 실행 중인 프로세스의 ID와, 해당 프로세스를 참조하는 핸들의 메모리 주소를 확인
    println!("{:?} @ {:p}", this_pid, this_proc);

    // 시스템의 정보가 들어 있는 SYSTEM_INFO 구조체
    println!("{:?}", proc_info);

    // 현재 프로세스에서 접근 가능한 가상 메모리 주소 공간의 시작과 끝
    println!("min : {:p}, max: {:p}", min_app_addr, max_app_addr);

    loop {

        // 루프 안에서 mem_info는 VirtualQueryEx에 의해 계속 덮어씌워진다

        // rc가 0이면 메모리 탐색 종료된다
        let rc: SIZE_T = unsafe {
            kernel32::VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEMINFO_SIZE as SIZE_T)
        };

        if rc == 0 {
            break;
        }
    
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;

    }

    // VirtualQueryEx로 조회한 메모리 블록의 상태(mem_info)
    println!("{:#?}" , mem_info);
}
