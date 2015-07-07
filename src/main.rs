extern crate clap;
extern crate notify;
extern crate term;

use clap::App;
use notify::{RecommendedWatcher, Error, Watcher};
use std::io::Write;
use std::thread;
use std::string::String;
use std::path::Path;
use std::process::{Command, Stdio};
use std::convert::Into;
use std::sync::mpsc::channel;
use term::color;

fn out<S:Into<String>>(label: &str, msg: S, color: color::Color) -> std::io::Result<()> {
  let mut out = term::stdout().unwrap();
  try!(out.reset());
  try!(out.fg(color));
  try!(write!(&mut std::io::stdout(), "{:>12}", format!("Capgun {}", label)));
  try!(out.reset());
  try!(write!(&mut std::io::stdout(), " {}\n", msg.into()));
  try!(out.flush());
  Ok(())
}

fn err<S:Into<String>>(label: &str, msg: S) -> std::io::Result<()> {
  let mut out = term::stderr().unwrap();
  try!(out.reset());
  try!(out.fg(color::RED));
  try!(write!(&mut std::io::stderr(), "{:>12}", format!("Capgun {}", label)));
  try!(out.reset());
  try!(write!(&mut std::io::stderr(), " {}\n", msg.into()));
  try!(out.flush());
  Ok(())
}

fn main() {
  let matches = App::new("capgun")
    .about("fires off commands when files change")
    .args_from_usage(
      "-c, --command=<CMD> 'cmd to run when files under <INPUT> change'
       <INPUT> 'Sets the directory to watch'"
    )
    .get_matches();

  let cmd = matches.value_of("CMD").unwrap();
  let input = matches.value_of("INPUT").unwrap();

  let _ = out("Watching", input, color::BRIGHT_CYAN);

  let (tx, rx) = channel();
  let watch: Result<RecommendedWatcher, Error> = Watcher::new(tx);

  match watch {
    Ok(mut watcher) =>  {
      match watcher.watch(&Path::new(input)) {
        Ok(_) => {},
        Err(e) => {
          let _ = err("Error", format!("Failed to load gun {:?}", e));
          return
        },
      }

      loop {
        thread::sleep_ms(5000);
        match rx.recv() {
          Ok(e) => fire(e, cmd),
          Err(e) => {
            let _ = err("Missfire", &(format!("{:?}", e)[..]));
            return
          }
        }
      }
    },
    Err(_) => println!("Couldn't generate directory watcher")
  }
}

fn fire(e: notify::Event, cmd:  &str) {
  match e.op {
    Ok(_) => {
      let mut split = cmd.split(" ");
      let mut task = Command::new(split.next().unwrap());
      for s in split {
        task.arg(s);
      }
      let output = task
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap_or_else(|e| panic!("Failed to fire: {} {}", cmd, e));

      println!("{}", String::from_utf8_lossy(&output.stdout));
      match writeln!(&mut std::io::stderr(), "{}", String::from_utf8_lossy(&output.stderr)) {
        Err(x) => panic!("Unable to write to stderr: {}", x),
        Ok(_) => {}
      }
      if output.status.success() {
        let _ = out("Hit", cmd, color::BRIGHT_GREEN);
      } else {
        let _ = err("Miss", cmd);
      }
    },
    Err(e) => println!("{:?}", e),
  }
}
