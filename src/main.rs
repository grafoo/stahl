use std::path::Path;
use std::io::ErrorKind;
use std::fs;
extern crate nix;
use nix::mount::{mount, umount, MS_NOSUID, MS_NODEV, MS_NOEXEC, MS_RELATIME, MS_BIND};

fn main() {
    const ROOTFS: &'static str = "rootfs";
    const NONE: Option<&'static [u8]> = None;

    // assign absolute path of root filesystem in rootfs which will be used for chroot.
    // create rootfs if it doesn't exist.
    let rootfs = Path::new(ROOTFS).canonicalize().unwrap_or_else(
        |e| if e.kind() ==
            ErrorKind::NotFound
        {
            fs::create_dir(ROOTFS)
                .map(|_| Path::new(ROOTFS).canonicalize().unwrap())
                .unwrap_or_else(|e| panic!("{}", e))
        } else {
            panic!("{}", e)
        },
    );

    // assign pseudo filesystem paths of rootfs.
    // create them if they don't exist.
    // pseudofs is a vec which holds all pseudofs paths.
    let rootfs_proc = rootfs.join("proc");
    let rootfs_sys = rootfs.join("sys");
    let rootfs_dev = rootfs.join("dev");
    let rootfs_dev_pts = rootfs.join("dev/pts");

    let pseudofs = vec![&rootfs_proc, &rootfs_sys, &rootfs_dev_pts, &rootfs_dev];

    for fs in pseudofs.iter() {
        if !fs.is_dir() {
            fs::create_dir_all(fs).unwrap();
        }
    }

    // mount the pseudo filesystems.
    mount(
        Some("proc"),
        &rootfs_proc,
        Some("proc"),
        MS_NOSUID | MS_NODEV | MS_NOEXEC | MS_RELATIME,
        NONE,
    ).unwrap();

    mount(
        Some("sys"),
        &rootfs_sys,
        Some("sysfs"),
        MS_NOSUID | MS_NODEV | MS_NOEXEC | MS_RELATIME,
        NONE,
    ).unwrap();

    mount(Some("/dev"), &rootfs_dev, Some("devtmpfs"), MS_BIND, NONE).unwrap();

    mount(
        Some("/dev/pts"),
        &rootfs_dev_pts,
        Some("devpts"),
        MS_BIND,
        NONE,
    ).unwrap();

    // unmount the pseudo filesystems.
    for fs in pseudofs.iter() {
        umount(fs.as_path()).unwrap();
    }
}
