use std::ffi::OsStr;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{fs::File, path::Path};

use clap::{Parser, Subcommand, Args};
use indicatif::{ProgressBar, ProgressStyle};
use libnspire::{dir::EntryType, PID, PID_CX2, VID};

use std::ptr::NonNull;
use std::time::Instant;

#[link(name = "usb-1.0")]
extern "C" {
    fn libusb_init_context(
        ctx: *mut *mut libusb1_sys::libusb_context,
        options: *const LibusbInitOption,
        num_options: i32,
    ) -> i32;
}

extern "C" {
    #[link_name = "libusb_wrap_sys_device"]
    fn wrap_sys_device_fd(
        ctx: *mut libusb1_sys::libusb_context,
        fd: isize,
        handle: *mut *mut libusb1_sys::libusb_device_handle,
    ) -> i32;
}

#[repr(C)]
struct LibusbInitOption {
    option: i32,
    value: LibusbInitOptionValue,
}

#[repr(C)]
union LibusbInitOptionValue {
    ival: i32,
    log_cbval: *mut std::ffi::c_void,
}

#[derive(Parser, Debug)]
#[clap(author, about, version)]
struct Opt {
  #[clap(subcommand)]
  cmd: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
  Upload(Upload),
  Download(Download),
  UploadOS(UploadOS),
  Copy(Copy),
  Move(Move),
  Mkdir(Mkdir),
  Rmdir(Rmdir),
  Ls(Ls),
  /// Read-only USB and filesystem-service diagnostics
  Diagnose,
  /// View license information
  License,
}

/// Upload files to the calculator
#[derive(Args, Debug)]
struct Upload {
  /// Files to upload
  #[clap(required = true, parse(from_os_str))]
  files: Vec<PathBuf>,
  /// Destination path
  dest: String,
}

/// Download files from the calculator
#[derive(Args, Debug)]
struct Download {
  /// Files to download
  #[clap(required = true)]
  files: Vec<String>,
  /// Destination path
  #[clap(required = true, parse(from_os_str))]
  dest: PathBuf,
}

/// Upload and install a .tcc/.tco/.tcc2/.tco2/.tct2 OS file
#[derive(Args, Debug)]
struct UploadOS {
  /// Path to the OS file
  #[clap(required = true, parse(from_os_str))]
  file: PathBuf,

  /// Disables the file extension check
  #[clap(long)]
  no_check_os: bool,
}

/// Copy a file to a different location
#[derive(Args, Debug)]
struct Copy {
  /// Path to file
  #[clap(required = true)]
  from_path: String,

  /// Path to new location
  #[clap(required = true)]
  dist_path: String,
}

/// Move a file or directory to a new location
#[derive(Args, Debug)]
struct Move {
  /// Path to file
  #[clap(required = true)]
  from_path: String,

  /// Path to new location
  #[clap(required = true)]
  dist_path: String,
}

/// Create a directory
#[derive(Args, Debug)]
struct Mkdir {
  /// Path to directory
  #[clap(required = true)]
  path: String,
}

/// Delete a directory
#[derive(Args, Debug)]
struct Rmdir {
  /// Path to directory
  #[clap(required = true)]
  path: String,
}

/// List the contents of a directory
#[derive(Args, Debug)]
struct Ls {
  /// Path to directory
  #[clap(required = true)]
  path: String,
}

fn get_dev() -> Option<libnspire::Handle<rusb::Context>> {
  let start = Instant::now();
  eprintln!("[V2 DIAG] stage=usb_fd_lookup");
  let fd: isize = (std::env::var("TERMUX_USB_FD").or_else(|_| std::env::var("USB_FD")))
    .ok()?
    .parse()
    .ok()?;

  eprintln!("[V2 DIAG] stage=usb_fd_available elapsed_ms={}", start.elapsed().as_millis());
  unsafe {
    let option = LibusbInitOption {
      option: 2, // LIBUSB_OPTION_NO_DEVICE_DISCOVERY
      value: LibusbInitOptionValue { ival: 0 },
    };

    let mut raw_ctx: *mut libusb1_sys::libusb_context = std::ptr::null_mut();

    eprintln!("[V2 DIAG] stage=libusb_context_start");
    let rc = libusb_init_context(&mut raw_ctx, &option, 1);
    if rc != 0 || raw_ctx.is_null() {
      eprintln!("libusb_init_context failed: {}", rc);
      return None;
    }

    eprintln!("[V2 DIAG] stage=libusb_context_result rc={} elapsed_ms={}", rc, start.elapsed().as_millis());
    let context = rusb::Context::from_libusb(
      NonNull::new(raw_ctx)?
    );

    let mut raw_handle: *mut libusb1_sys::libusb_device_handle =
      std::ptr::null_mut();

    eprintln!("[V2 DIAG] stage=usb_fd_wrap_start");
    let rc = wrap_sys_device_fd(
      raw_ctx,
      fd,
      &mut raw_handle,
    );

    eprintln!("[V2 DIAG] stage=usb_fd_wrap_result rc={} elapsed_ms={}", rc, start.elapsed().as_millis());
    if rc != 0 || raw_handle.is_null() {
      eprintln!("libusb_wrap_sys_device failed: {}", rc);
      return None;
    }

    let device = rusb::DeviceHandle::from_libusb(
      context,
      NonNull::new(raw_handle)?
    );

    eprintln!("[V2 DIAG] stage=nspire_session_start");
    match libnspire::Handle::new(device) {
      Ok(handle) => {
        eprintln!("[V2 DIAG] stage=nspire_session_ok elapsed_ms={}", start.elapsed().as_millis());
        Some(handle)
      },
      Err(e) => {
        eprintln!("libnspire initialization failed: {:?}", e);
        None
      }
    }
  }
}

pub fn cwd() -> PathBuf {
  #[cfg(target_os = "linux")]
  if std::env::var_os("APPIMAGE").is_some() && std::env::var_os("APPDIR").is_some() {
    if let Some(cwd) = std::env::var_os("OWD") {
      return cwd.into();
    }
  };
  std::env::current_dir().expect("Couldn't get current directory")
}

fn main() {
  let opt: Opt = Opt::parse();
  if let Some(cmd) = opt.cmd {
    match cmd {
      SubCommand::Upload(Upload { files, mut dest }) => {
        if let Some(handle) = get_dev() {
          for file in files {
            let mut buf = vec![];
            File::open(cwd().join(&file))
              .unwrap()
              .read_to_end(&mut buf)
              .unwrap();
            let name = file
              .file_name()
              .expect("Failed to get file name")
              .to_string_lossy()
              .to_string();
            let bar = ProgressBar::new(buf.len() as u64);
            bar.set_style(ProgressStyle::default_bar().template("{spinner:.green} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"));
            bar.set_message(&format!("Upload {}", name));
            bar.enable_steady_tick(100);
            if dest.ends_with('/') {
              dest.remove(dest.len() - 1);
            }
            let res = handle.write_file(&format!("{}/{}", dest, name), &buf, &mut |remaining| {
              bar.set_position((buf.len() - remaining) as u64)
            });

            match res {
              Ok(_) => {
                bar.finish_with_message(&format!("Upload {}: Ok", dest));
              }
              Err(error) => {
                bar.abandon_with_message(&format!("Failed: {}", error));
                std::process::exit(1);
              }
            }
          }
        } else {
          eprintln!("Couldn't find any device");
          std::process::exit(1);
        }
      }
      SubCommand::Download(Download { dest, files }) => {
        if let Some(handle) = get_dev() {
          for file in files {
            let attr = handle.file_attr(&file);
            match attr {
              Ok(attr) => {
                let path = Path::new(&file);
                let dest_path = Path::join(&dest, path.file_name().unwrap().to_str().unwrap());
                match File::create(dest_path) {
                  Ok(mut dest_file) => {
                    let mut buf = vec![0u8; attr.size() as usize];

                    let bar = ProgressBar::new(buf.len() as u64);
                    bar.set_style(ProgressStyle::default_bar().template("{spinner:.green} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"));
                    bar.set_message(&format!(
                      "Download {}",
                      path.file_name().unwrap().to_str().unwrap()
                    ));
                    bar.enable_steady_tick(100);

                    let len = buf.len();

                    let res = handle.read_file(&file, &mut buf, &mut |remaining| {
                      bar.set_position((len - remaining) as u64);
                    });

                    match res {
                      Ok(_) => {
                        bar.set_message("Writing file to disk");

                        match dest_file.write_all(&buf) {
                          Ok(_) => {
                            bar.finish_with_message("Transfer completed");
                          }
                          Err(error) => {
                            bar.abandon_with_message(&format!(
                              "Failed to write file to disk: {}",
                              error
                            ));
                          }
                        }
                      }
                      Err(error) => {
                        bar.abandon_with_message(&format!("Failed to transfer file: {}", error))
                      }
                    }
                  }
                  Err(error) => {
                    eprintln!("Failed to open destination file: {}", error);
                  }
                }
              }
              Err(error) => {
                eprintln!("Failed to read file info: {}", error);
              }
            }
          }
        } else {
          eprintln!("Couldn't find any device");
          std::process::exit(1);
        }
      }
      SubCommand::UploadOS(UploadOS { file, no_check_os }) => {
        if let Some(handle) = get_dev() {
          let calc_info = handle.info().expect("Failed to obtain device info");

          let file_ext = file
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .to_string();

          let mut buf = vec![];
          let mut f = File::open(cwd().join(&file)).unwrap_or_else(|err| {
            eprintln!("Failed to open file: {}", err);
            std::process::exit(1);
          });

          if format!(".{}", file_ext) != calc_info.os_extension {
            if no_check_os {
              eprintln!(
                "Warning: {} expects file of type {}",
                calc_info.name, calc_info.os_extension
              );
            } else {
              eprintln!(
                "Error: {} expects file of type {}",
                calc_info.name, calc_info.os_extension
              );
              eprintln!("Provide --no-check-os to bypass this check.");
              std::process::exit(1);
            }
          }

          f.read_to_end(&mut buf).unwrap();

          let name = file
            .file_name()
            .expect("Failed to get file name")
            .to_string_lossy()
            .to_string();

          let bar = ProgressBar::new(buf.len() as u64);
          bar.set_style(ProgressStyle::default_bar().template("{spinner:.green} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"));
          bar.set_message(&format!("Upload OS {}", name));
          bar.enable_steady_tick(100);

          let res = handle.send_os(&buf, &mut |remaining| {
            bar.set_position((buf.len() - remaining) as u64);
          });

          match res {
            Ok(_) => {
              bar.finish();
            }
            Err(error) => {
              bar.abandon_with_message(&format!("OS Upload failed: {}", error));
            }
          }
        } else {
          eprintln!("Couldn't find any device");
          std::process::exit(1);
        }
      }
      SubCommand::Copy(Copy {
        from_path,
        dist_path,
      }) => {
        if let Some(handle) = get_dev() {
          match handle.copy_file(&from_path, &dist_path) {
            Ok(_) => {
              println!("Copy {} => {}: Ok", from_path, dist_path);
            }
            Err(error) => {
              eprintln!("Failed to copy file or directory: {}", error);
            }
          }
        } else {
          eprintln!("Couldn't find any device");
          std::process::exit(1);
        }
      }
      SubCommand::Move(Move {
        from_path,
        dist_path,
      }) => {
        if let Some(handle) = get_dev() {
          match handle.move_file(&from_path, &dist_path) {
            Ok(_) => {
              println!("Move {} => {}: Ok", from_path, dist_path);
            }
            Err(error) => {
              eprintln!("Failed to move file or directory: {}", error);
            }
          }
        } else {
          eprintln!("Couldn't find any device");
          std::process::exit(1);
        }
      }
      SubCommand::Mkdir(Mkdir { path }) => {
        if let Some(handle) = get_dev() {
          match handle.create_dir(&path) {
            Ok(_) => {
              println!("Create {}: Ok", path);
            }
            Err(error) => {
              eprintln!("Failed to create directory: {}", error);
            }
          }
        } else {
          eprintln!("Couldn't find any device");
          std::process::exit(1);
        }
      }
      SubCommand::Rmdir(Rmdir { path }) => {
        if let Some(handle) = get_dev() {
          match handle.delete_dir(&path) {
            Ok(_) => {
              println!("Remove {}: Ok", path);
            }
            Err(error) => {
              eprintln!("Failed to delete directory: {}", error);
            }
          }
        } else {
          eprintln!("Couldn't find any device");
          std::process::exit(1);
        }
      }
      SubCommand::Ls(Ls { path }) => {
        if let Some(handle) = get_dev() {
          match handle.list_dir(&path) {
            Ok(dir_list) => {
              for item in dir_list.iter() {
                println!(
                  "{}{}",
                  item.name().to_str().unwrap(),
                  if item.entry_type() == EntryType::Directory {
                    "/"
                  } else {
                    ""
                  }
                );
              }
            }
            Err(error) => {
              eprintln!("Failed to list directory: {}", error);
              std::process::exit(1);
            }
          }
        } else {
          eprintln!("Couldn't find any device");
          std::process::exit(1);
        }
      }
      SubCommand::Diagnose => {
        eprintln!("[V2 DIAG] mode=read_only no_reset=true no_write=true");
        if let Some(handle) = get_dev() {
          for path in ["/", "/documents"] {
            let start = Instant::now();
            eprintln!("[V2 DIAG] stage=list_start path={}", path);
            match handle.list_dir(path) {
              Ok(entries) => eprintln!("[V2 DIAG] stage=list_ok path={} entries={} elapsed_ms={}", path, entries.len(), start.elapsed().as_millis()),
              Err(err) => eprintln!("[V2 DIAG] stage=list_error path={} error={:?} elapsed_ms={}", path, err, start.elapsed().as_millis()),
            }
          }
        } else {
          eprintln!("[V2 DIAG] stage=connection_failed");
          std::process::exit(2);
        }
      }
      SubCommand::License => {
        println!("License text was not included in this recovered source snapshot; see upstream project licenses.");
        println!(include_str!("NOTICE.txt"), env!("CARGO_PKG_REPOSITORY"));
      }
    }
  } else {
    println!("N-Link Termux CLI");
    println!("Run with --help to see commands.");
  }
}
