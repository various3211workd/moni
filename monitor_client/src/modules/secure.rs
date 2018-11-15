extern crate sysinfo;

// デバッガの起動を感知して終了する
pub fn debug_check() -> bool {
    use sysinfo::{ProcessExt, SystemExt};

    let mut system = sysinfo::System::new();

    system.refresh_all();

    for(_, proc_) in system.get_process_list(){
        //println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
        match proc_.name() {
            "ollydbg.exe" | "x32dbg.exe" | "x64dbg.exe" => {
                return true;
            },
            _ => {},
        }
    }
    return false;
}
