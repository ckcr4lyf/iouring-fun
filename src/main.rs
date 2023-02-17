use io_uring::{opcode, types, IoUring};
use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::{fs, io};

fn main() -> io::Result<()> {
    println!("\nSTARTING\n");
    let mut ring = IoUring::new(8)?;

    let fd = fs::File::options()
    .read(true)
    .write(true)
    .open("README.md")?;
    let mut buf = vec![0; 1024];

    let read_e = opcode::Read::new(types::Fd(fd.as_raw_fd()), buf.as_mut_ptr(), buf.len() as _)
        .build()
        .user_data(0x42);

    // Note that the developer needs to ensure
    // that the entry pushed into submission queue is valid (e.g. fd, buffer).
    unsafe {
        ring.submission()
            .push(&read_e)
            .expect("submission queue is full");
    }

    // ring.submit_and_wait(1)?;

    // let cqe = ring.completion().next().expect("completion queue is empty");
    // assert_eq!(cqe.user_data(), 0x42);
    // assert!(cqe.result() >= 0, "read error: {}", cqe.result());

    let mut data: [u8; 100] = [0x41; 100];

    let write_e = opcode::Write::new(types::Fd(fd.as_raw_fd()), data.as_mut_ptr(), 50).build();
    ring.submit_and_wait(1)?;
    let cqe = ring.completion().next().expect("completion queue is empty");
    assert!(cqe.result() >= 0, "read error: {}", cqe.result());
    
    unsafe {
        ring.submission()
            .push(&write_e)
            .expect("submission queue is full");
    }

    ring.submit_and_wait(1)?;

    
    let cqe = ring.completion().next().expect("completion queue is empty");
    assert!(cqe.result() >= 0, "write error: {}", cqe.result());

    // println!("Data is {}", std::str::from_utf8(&buf).unwrap());

    // let mut fd2 = fs::File::open("README.md")?;
    // let mut buf2: [u8; 1024] = [0; 1024];
    // fd2.read(&mut buf2);

    // println!("Data2 is {}", std::str::from_utf8(&buf2).unwrap());

    Ok(())
}
