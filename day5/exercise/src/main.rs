// use crate::io::ErrorKind;
// use std::fs::File;
// use std::io;
// use std::io::BufReader;
// use std::io::BufWriter;
// use std::io::Read;
// use std::io::Write;
// use std::path::Path;
// // use std::io::ErrorKind;
// fn main() -> io::Result<()> {
//     let input_filename = "/Users/zhaowenye/Documents/StudyRust/zhaowenming/day5/exercise/src/1.txt";
//     let output_filename =
//         "/Users/zhaowenye/Documents/StudyRust/zhaowenming/day5/exercise/src/3.txt";
//     copy_file(input_filename, output_filename);
//     Ok(())
// }
// fn copy<R, W>(reader: &mut R, writer: &mut W) -> io::Result<u64>
// where
//     R: Read,
//     W: Write,
// {
//     const BUF_SIZE: usize = 1024 * 64;
//     let mut buf = [0u8; BUF_SIZE];
//     let mut write_len = 0u64;
//     loop {
//         let len = match reader.read(&mut buf) {
//             Ok(0) => return Ok(write_len),
//             Ok(len) => len,
//             Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
//             Err(e) => return Err(e),
//         };

//         writer.write_all(&buf[..len])?;
//         write_len += len as u64;
//     }
// }

// fn copy_file<P: AsRef<Path>>(source_file: P, target_file: P) -> io::Result<()> {
//     let input_file = File::open(source_file)?;
//     let mut reader = BufReader::new(input_file);

//     let output_file = File::create(target_file)?;
//     let mut writer = BufWriter::new(output_file);

//     copy(&mut reader, &mut writer)?;
//     Ok(())
// }




use std::{sync::mpsc,thread,time::Duration};
fn main() {
  let (sender,receiver) = mpsc::channel::<String>();
  let _logger_thread = thread::spawn(move || {
    while let Ok(s) = receiver.recv() {
      println!("{:?}: {}",thread::current().id(), s);
    }
  });
  let mut sender_threads = vec![];

  for _ in 0..5 {
    let tx = sender.clone();
    sender_threads.push(thread::spawn(move || {
      let msg = format!("This is a message from sender thread {:?}", thread::current().id());
      tx.send(msg).unwrap();
      thread::sleep(Duration::from_secs(1));
    }));
  }

  for st in sender_threads {
    st.join().unwrap();
  }
}
