extern crate clap;
extern crate notify;
extern crate term;

use clap::App;
use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

use std::io::Write;
use std::thread;
use std::string::String;
use std::path::Path;
use std::process::Command;

fn out(label: &str, msg: &str, color: term::color::Color) -> std::io::Result<()> {
  let mut out = term::stdout().unwrap();
  try!(out.reset());
  try!(out.fg(color));
  try!(write!(&mut std::io::stdout(), "{:>12}", label.to_owned()));
  try!(out.reset());
  try!(write!(&mut std::io::stdout(), " {}\n", msg.to_owned()));
  try!(out.flush());
  Ok(())
}

fn err(label: &str, msg: &str, color: term::color::Color) -> std::io::Result<()> {
  let mut out = term::stderr().unwrap();
  try!(out.reset());
  try!(out.fg(color));
  try!(write!(&mut std::io::stderr(), "{:>12}", label.to_owned()));
  try!(out.reset());
  try!(write!(&mut std::io::stderr(), " {}\n", msg.to_owned()));
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

  let _ = out("Watching", input, term::color::BRIGHT_CYAN);

  let (tx, rx) = channel();
  let watch: Result<RecommendedWatcher, Error> = Watcher::new(tx);

  match watch {
    Ok(mut watcher) =>  {
      match watcher.watch(&Path::new(input)) {
        Ok(_) => {},
        Err(e) => {
          println!("Error creating watcher: {:?}", e);
          return
        },
      }

      loop {
        thread::sleep_ms(5000);
        match rx.recv() {
          Ok(e) => fire(e, cmd),
          Err(e) => println!("{:?}", e),
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
      let output = task.output()
        .unwrap_or_else(|e| { panic!("Failed to execute cargo: {}", e) });

      println!("{}", String::from_utf8_lossy(&output.stdout));
      match writeln!(&mut std::io::stderr(), "{}", String::from_utf8_lossy(&output.stderr)) {
        Err(x) => panic!("Unable to write to stderr: {}", x),
        Ok(_) => {}
      }
      if output.status.success() {
        let _ = out("Hit", cmd, term::color::BRIGHT_GREEN);
      } else {
        let _ = err("Miss", cmd, term::color::RED);
      }
    },
    Err(e) => println!("{:?}", e),
  }
}
