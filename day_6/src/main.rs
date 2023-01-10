use std::collections::HashSet;

fn main() {
    if let Err(e) = do_main() {
        eprintln!("{}", e);
    }
}

fn do_main() -> shared::Result<()> {
    let contents = shared::read_file_from_args()?;

    // ------------------------------------------------
    // Part 1
    // ------------------------------------------------

    let start_of_packet_marker = Marker::find_in_string(&contents, 4);

    println!(
        "End start-of-packer marker: {}",
        start_of_packet_marker.position
    );

    // ------------------------------------------------
    // Part 2
    // ------------------------------------------------

    let start_of_message_marker = Marker::find_in_string(&contents, 14);

    println!(
        "End start-of-message marker: {}",
        start_of_message_marker.position
    );

    Ok(())
}

struct Marker {
    packet_size: usize,
    position: usize,
    value: String,
}

impl Marker {
    pub fn new(packet_size: usize) -> Self {
        Self {
            packet_size,
            position: 0,
            value: String::new(),
        }
    }

    pub fn find_in_string(data: &String, size: usize) -> Self {
        let mut marker = Marker::new(size);

        for chr in data.chars() {
            marker.append(chr);

            if marker.is_valid() {
                break;
            }
        }

        marker
    }

    pub fn append(&mut self, ch: char) -> &Self {
        if self.value.len() >= self.packet_size {
            self.value.remove(0);
        }

        self.value.push(ch);
        self.position += 1;

        self
    }

    pub fn is_valid(&self) -> bool {
        if self.position < self.packet_size {
            return false;
        }

        let unique_chs: HashSet<char> = self.value.chars().collect();

        return unique_chs.len() == self.packet_size;
    }
}
