use chrono::Local;
use crossterm::{cursor, queue, style, terminal, Result};
use rand::Rng;
use std::io::Read;
use std::io::{stdout, Write};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

struct MatrixChar {
    x: u16,
    y: u16,
    color: style::Color,
    char: char,
}

impl MatrixChar {
    fn new(x: u16, y: u16, char: char) -> Self {
        MatrixChar {
            x,
            y,
            color: generate_random_color(),
            char,
        }
    }

    fn update(&mut self, terminal_width: u16, terminal_height: u16) {
        self.y += 1;
        if self.y >= terminal_height {
            self.y = 0;
            self.x = rand::thread_rng().gen_range(0..terminal_width);
        }
    }
}

// Generate a random background color
fn generate_random_color() -> style::Color {
    let mut rng = rand::thread_rng();
    let color = rng.gen_range(0..=255);
    style::Color::AnsiValue(color)
}

fn main() -> Result<()> {
    // Create a screen buffer
    let stdout = stdout();
    let shared_stdout = Arc::new(Mutex::new(stdout));

    // Enter alternate screen and raw mode
    terminal::enable_raw_mode()?;
    queue!(
        shared_stdout.lock().unwrap(),
        terminal::EnterAlternateScreen,
        cursor::Hide
    )?;

    // Initialize the RNG
    let rng = Arc::new(Mutex::new(rand::thread_rng()));

    // Set a Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Spawn a separate thread to read user input
    let shared_stdout_clone = shared_stdout.clone();
    std::thread::spawn(move || {
        let mut buffer = [0; 1];
        while let Ok(_) = std::io::stdin().read_exact(&mut buffer) {
            if buffer[0] == b'q' || buffer[0] == b'Q' {
                r.store(false, Ordering::SeqCst);
                let mut stdout = shared_stdout_clone.lock().unwrap();
                // Cleanup and exit
                let _ = terminal::disable_raw_mode();
                let _ = queue!(stdout, terminal::LeaveAlternateScreen, cursor::Show);
                let _ = stdout.flush();
                let _ = terminal::disable_raw_mode();
                // Reset the color of the terminal
                let _ = queue!(stdout, style::SetForegroundColor(style::Color::Reset));
                let _ = stdout.flush();
                exit(0);
            }
        }
    });

    // Create a vector to store MatrixChars
    let mut matrix_chars: Vec<MatrixChar> = vec![];

    // Main loop
    let mut terminal_width = terminal::size()?.0;
    let mut terminal_height = terminal::size()?.1;
    while running.load(Ordering::SeqCst) {
        // Get current date and time
        let now = Local::now();
        let datetime = now.format("%Y-%m-%d %H:%M:%S").to_string();

        // Check if terminal size has changed
        let new_terminal_width = terminal::size()?.0;
        let new_terminal_height = terminal::size()?.1;
        if new_terminal_width != terminal_width || new_terminal_height != terminal_height {
            terminal_width = new_terminal_width;
            terminal_height = new_terminal_height;
            matrix_chars.clear(); // Clear existing MatrixChars on size change
        }

        // Clear the screen
        queue!(
            shared_stdout.lock().unwrap(),
            terminal::Clear(terminal::ClearType::All)
        )?;

        // Print the date and time at the top center with white color
        let center_x = terminal_width / 2;
        let datetime_x = center_x - (datetime.len() as u16 / 2);
        queue!(
            shared_stdout.lock().unwrap(),
            cursor::MoveTo(datetime_x, 0),
            style::SetForegroundColor(style::Color::White),
            style::Print(datetime)
        )?;

        // Print the quit message at the second line from the top center with white color
        let quit_message = "Press 'q' to quit";
        let quit_message_x = center_x - (quit_message.len() as u16 / 2);
        queue!(
            shared_stdout.lock().unwrap(),
            cursor::MoveTo(quit_message_x, 1),
            style::SetForegroundColor(style::Color::White),
            style::Print(quit_message)
        )?;

        // Update and print MatrixChars
        for matrix_char in &mut matrix_chars {
            matrix_char.update(terminal_width, terminal_height);
            queue!(
                shared_stdout.lock().unwrap(),
                cursor::MoveTo(matrix_char.x, matrix_char.y),
                style::SetForegroundColor(matrix_char.color),
                style::Print(matrix_char.char)
            )?;
        }

        // Generate new MatrixChars
        let x = rng.lock().unwrap().gen_range(0..terminal_width);
        let char = rng
            .lock()
            .unwrap()
            .clone()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(1)
            .map(|c| c as char)
            .collect::<String>()
            .chars()
            .next()
            .unwrap();
        matrix_chars.push(MatrixChar::new(x, 0, char));

        // Remove MatrixChars that have fallen off the screen
        matrix_chars.retain(|matrix_char| matrix_char.y < terminal_height);

        // Flush the buffer
        shared_stdout.lock().unwrap().flush()?;

        // Wait before the next frame
        sleep(Duration::from_millis(30));
    }

    // Clean up and exit
    let mut stdout = shared_stdout.lock().unwrap();
    let _ = terminal::disable_raw_mode();
    let _ = queue!(stdout, terminal::LeaveAlternateScreen, cursor::Show);
    let _ = stdout.flush();
    Ok(())
}
