use log::info;
use std::io::{self, BufRead, Read, Result, Stdin, Stdout, Write};

pub struct Interactor<R, W> {
    input: R,
    output: W,
}

impl<R: BufRead, W: Write> Interactor<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Interactor {
            input: reader,
            output: writer,
        }
    }
}

impl Interactor<Stdin, Stdout> {
    pub fn standard() -> Self {
        let input = io::stdin();
        let output = io::stdout();
        Interactor { input, output }
    }
}

impl<R: Read, W> Read for Interactor<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.input.read(buf)
    }
}

// impl<R: BufRead, W> BufRead for Interactor<R, W> {
//     fn fill_buf(&mut self) -> Result<&[u8]> {
//         self.input.fill_buf()
//     }

//     fn consume(&mut self, amt: usize) {
//         self.input.consume(amt)
//     }
// }

impl<R, W: Write> Write for Interactor<R, W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.output.flush()
    }
}

pub fn start_repl<T, R, W>(socket: &mut T, interactor: &mut Interactor<R, W>) -> Result<()>
where
    T: Read + Write,
    R: Read,
    W: Write,
{
    interactor.write_all("Welcome to Sprockets, the repl for sockets".as_bytes())?;
    loop {
        interactor.write_all("\n>".as_bytes())?;
        interactor.flush()?;
        let mut input = String::new();

        let read = interactor.read_to_string(&mut input)?;
        info!("Read {:?} bytes. Input is {}", read, input);

        if input.eq(&String::from("quit")) {
            break;
        }

        socket.write_all(input.as_bytes())?;

        let mut response = String::new();
        let bytes = socket.read_to_string(&mut response)?;
        info!("Read {} bytes. Response is {}", bytes, response);

        let written = interactor.write_all(response.as_bytes())?;

        // let unmatched = input.chars().fold(0, |count, c| match c {
        //     '{' | '(' | '[' => count + 1,
        //     '}' | '(' | '[' => count - 1,
        //     _ => count,
        // });
        // info!("Found {} unmatched chars", unmatched);
    }
    Ok(())
}
