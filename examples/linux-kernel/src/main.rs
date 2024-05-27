use fuser_git::fuser::MountOption;
use fuser_git::{fuser, FuserGit};
use std::thread::sleep;
use std::time::Duration;

pub fn main() {
    let fs = FuserGit::new();
    let mount = std::env::current_dir().unwrap().join("kernel");
    eprintln!("mounting linux kernel at: {:?}", &mount);
    std::fs::create_dir_all(&mount).unwrap();
    fuser::mount2(
        fs,
        mount,
        &[
            MountOption::RO,
            MountOption::FSName("fuser_git".to_string()),
            MountOption::AutoUnmount,
        ],
    )
    .unwrap();
    eprintln!("Hello!");
    sleep(Duration::from_secs(60));
    eprintln!("Bye!");
}
