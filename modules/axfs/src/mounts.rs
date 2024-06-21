use alloc::sync::Arc;
use axfs_vfs::{VfsNodeType, VfsOps, VfsResult};

use crate::fs;

#[cfg(feature = "devfs")]
pub(crate) fn devfs() -> Arc<fs::devfs::DeviceFileSystem> {
    let null = fs::devfs::NullDev;
    let zero = fs::devfs::ZeroDev;
    let bar = fs::devfs::ZeroDev;
    let devfs = fs::devfs::DeviceFileSystem::new();
    let foo_dir = devfs.mkdir("foo");
    devfs.add("null", Arc::new(null));
    devfs.add("zero", Arc::new(zero));
    foo_dir.add("bar", Arc::new(bar));
    Arc::new(devfs)
}

#[cfg(feature = "ramfs")]
pub(crate) fn ramfs() -> VfsResult<Arc<fs::ramfs::RamFileSystem>> {
    let tmpfs = fs::ramfs::RamFileSystem::new();
    let tmp_root = tmpfs.root_dir();

    // Create /tmp/index.html
    tmp_root.create("index.html", VfsNodeType::File)?;
    let file_html = tmp_root.clone().lookup("./index.html")?;
    file_html.write_at(0, r#"<!DOCTYPE html>
<html lang="en-US">
  <head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">


<title>辰龙操作系统（ChenLongOS）</title>

<meta property="og:title" content="辰龙操作系统（ChenLongOS）" />
  </head>
  <body>
    <div class="container-lg px-3 my-5 markdown-body"> \

      <h1><a href="https://chenlongos.com/">欢迎来到辰龙社区</a></h1>
      <img src="https://chenlongos.cn/images/black-logo.png" alt="ChenLongOS Logo">
      <p>This is <span style="color: blue;">blue</span> text.</p>
      <p>This is <span style="color: red;">red</span> text.</p>
      <p>This is <span style="color: green;">green</span> text.</p>
    </div>
  </body>
</html>
"#.as_bytes())?;

    Ok(Arc::new(tmpfs))
}

#[cfg(feature = "procfs")]
pub(crate) fn procfs() -> VfsResult<Arc<fs::ramfs::RamFileSystem>> {
    let procfs = fs::ramfs::RamFileSystem::new();
    let proc_root = procfs.root_dir();

    // Create /proc/sys/net/core/somaxconn
    proc_root.create("sys", VfsNodeType::Dir)?;
    proc_root.create("sys/net", VfsNodeType::Dir)?;
    proc_root.create("sys/net/core", VfsNodeType::Dir)?;
    proc_root.create("sys/net/core/somaxconn", VfsNodeType::File)?;
    let file_somaxconn = proc_root.clone().lookup("./sys/net/core/somaxconn")?;
    file_somaxconn.write_at(0, b"4096\n")?;

    // Create /proc/sys/vm/overcommit_memory
    proc_root.create("sys/vm", VfsNodeType::Dir)?;
    proc_root.create("sys/vm/overcommit_memory", VfsNodeType::File)?;
    let file_over = proc_root.clone().lookup("./sys/vm/overcommit_memory")?;
    file_over.write_at(0, b"0\n")?;

    // Create /proc/self/stat
    proc_root.create("self", VfsNodeType::Dir)?;
    proc_root.create("self/stat", VfsNodeType::File)?;

    Ok(Arc::new(procfs))
}

#[cfg(feature = "sysfs")]
pub(crate) fn sysfs() -> VfsResult<Arc<fs::ramfs::RamFileSystem>> {
    let sysfs = fs::ramfs::RamFileSystem::new();
    let sys_root = sysfs.root_dir();

    // Create /sys/kernel/mm/transparent_hugepage/enabled
    sys_root.create("kernel", VfsNodeType::Dir)?;
    sys_root.create("kernel/mm", VfsNodeType::Dir)?;
    sys_root.create("kernel/mm/transparent_hugepage", VfsNodeType::Dir)?;
    sys_root.create("kernel/mm/transparent_hugepage/enabled", VfsNodeType::File)?;
    let file_hp = sys_root
        .clone()
        .lookup("./kernel/mm/transparent_hugepage/enabled")?;
    file_hp.write_at(0, b"always [madvise] never\n")?;

    // Create /sys/devices/system/clocksource/clocksource0/current_clocksource
    sys_root.create("devices", VfsNodeType::Dir)?;
    sys_root.create("devices/system", VfsNodeType::Dir)?;
    sys_root.create("devices/system/clocksource", VfsNodeType::Dir)?;
    sys_root.create("devices/system/clocksource/clocksource0", VfsNodeType::Dir)?;
    sys_root.create(
        "devices/system/clocksource/clocksource0/current_clocksource",
        VfsNodeType::File,
    )?;
    let file_cc = sys_root
        .clone()
        .lookup("devices/system/clocksource/clocksource0/current_clocksource")?;
    file_cc.write_at(0, b"tsc\n")?;

    Ok(Arc::new(sysfs))
}
