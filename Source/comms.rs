use serialport;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub type DeckTrigger = u8;

pub fn listen(port_name: String, baud_rate: u32, tx: Sender<DeckTrigger>) {
    thread::spawn(move || {
        let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect(
            "Error :: Failed to open Communication port at {port_name} with Baud Rate {baud_rate}",
        );

        let _ = port.clear(serialport::ClearBuffer::Input);

        let mut buf = vec![0; 1];
        loop {
            if port.read_exact(&mut buf).is_ok() {
                let _ = tx.send(buf[0]);
            }
        }
    });
}
