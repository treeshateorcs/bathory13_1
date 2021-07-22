#![allow(dead_code)]
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use std::io::stdout;
use std::io::Write;
use std::str::FromStr;

fn main() {
  stdout()
    .queue(crossterm::terminal::EnterAlternateScreen)
    .unwrap();
  crossterm::terminal::enable_raw_mode().unwrap();
  stdout().queue(crossterm::cursor::Hide).unwrap();
  let mut screen = Screen::ArtistAlbumTrack {
    sides: true,
    curr: 0,
    total: 0,
  };
  loop {
    screen.switch_screen();
    stdout().flush().unwrap();
    match crossterm::event::read().unwrap() {
      crossterm::event::Event::Key(crossterm::event::KeyEvent { code, .. }) => match code {
        crossterm::event::KeyCode::Esc => break,
        //        crossterm::event::KeyCode::Enter => {
        //          screen.enter();
        //        }
        crossterm::event::KeyCode::Char(c) => match c {
          'q' => break,
          '1' => {
            screen = Screen::ArtistAlbumTrack {
              sides: true,
              curr: 0,
              total: 0,
            };
          }
          '2' => {
            screen = Screen::Library {
              sides: false,
              curr: 0,
              total: 0,
            };
          }
          '5' => {
            screen = Screen::Browser {
              sides: false,
              curr: 4,
              total: 1,
              path: std::path::PathBuf::from_str("/home/tho").unwrap(),
            };
          }
          'j' => {
            screen.scroll_down();
          }
          'k' => {
            screen.scroll_up();
          }
          _ => {}
        },
        crossterm::event::KeyCode::Up => {
          screen.scroll_up();
        }
        crossterm::event::KeyCode::Down => {
          screen.scroll_down();
        }
        _ => {}
      },
      _ => {}
    }
  }
  stdout()
    .execute(crossterm::terminal::LeaveAlternateScreen)
    .unwrap();
  stdout().execute(crossterm::cursor::Show).unwrap();
}

enum Screen {
  ArtistAlbumTrack {
    sides: bool,
    curr: usize,
    total: usize,
  },
  Library {
    sides: bool,
    curr: u8,
    total: usize,
  },
  PlaylistTrack {
    sides: bool,
    curr: u8,
    total: usize,
  },
  PlayQueue {
    sides: bool,
    curr: u8,
    total: usize,
  },
  Browser {
    sides: bool,
    curr: usize,
    total: usize,
    path: std::path::PathBuf,
  },
  LibraryFilters {
    sides: bool,
    curr: u8,
    total: usize,
  },
  Settings {
    sides: bool,
    curr: u8,
    total: usize,
  },
}

impl Screen {
  //  pub fn enter(&self) {
  //    match self {
  //      Self::Browser => {}
  //      _ => {}
  //    }
  //  }

  pub fn switch_screen(&mut self) {
    self.print_header();
    self.print_body();
  }

  fn print_header(&mut self) {
    stdout()
      .queue(crossterm::style::SetAttribute(
        crossterm::style::Attribute::Reset,
      ))
      .unwrap();
    stdout()
      .queue(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
      ))
      .expect("failed to clear screen");
    stdout().queue(crossterm::cursor::MoveTo(0, 0)).unwrap();
    let mut header = String::new();
    let w = crossterm::terminal::size().unwrap().0;
    match self {
      Self::ArtistAlbumTrack { .. } => {
        header.push_str(" Artist / Album");
        while header.len() < (w / 3) as usize + 1 {
          header.push(' ');
        }
        header.push_str(" Track");
        while header.len() < w as usize - "Library ".len() {
          header.push(' ');
        }
        header.push_str("Library ");
      }
      Self::Library { .. } => {
        header.push_str(" Library");
        while header.len() < w as usize {
          header.push(' ');
        }
      }
      Self::Browser { path, .. } => {
        header.push_str(" Browser - ");
        header.push_str(path.to_str().unwrap());
        while header.len() < w as usize {
          header.push(' ');
        }
      }
      _ => {}
    }
    stdout()
      .queue(crossterm::style::SetForegroundColor(
        crossterm::style::Color::Grey,
      ))
      .unwrap();
    stdout()
      .queue(crossterm::style::SetBackgroundColor(
        crossterm::style::Color::DarkBlue,
      ))
      .unwrap();
    stdout()
      .queue(crossterm::style::SetAttribute(
        crossterm::style::Attribute::Bold,
      ))
      .unwrap();
    stdout().queue(crossterm::style::Print(header)).ok();
  }

  fn print_body(&mut self) {
    let (w, h) = crossterm::terminal::size().unwrap();
    let mut height: usize = 1;
    let files: std::fs::ReadDir;
    if let Self::Browser {
      sides: _,
      curr,
      total,
      path,
    } = self
    {
      files = std::fs::read_dir(path).expect("failed to read directory");
      stdout().queue(crossterm::cursor::MoveTo(0, 1)).unwrap();
      let files = files
        .filter(|f| {
          !f.as_ref()
            .unwrap()
            .file_name()
            .into_string()
            .unwrap()
            .starts_with('.')
        })
        .collect::<Vec<Result<std::fs::DirEntry, std::io::Error>>>();
      *total = files.len();
      stdout()
        .queue(crossterm::style::SetAttribute(
          crossterm::style::Attribute::Reset,
        ))
        .unwrap();
      stdout()
        .queue(crossterm::style::SetForegroundColor(
          crossterm::style::Color::DarkBlue,
        ))
        .unwrap();
      stdout()
        .queue(crossterm::style::SetBackgroundColor(
          crossterm::style::Color::Black,
        ))
        .unwrap();
      stdout()
        .queue(crossterm::style::SetAttribute(
          crossterm::style::Attribute::Bold,
        ))
        .unwrap();
      for file in files {
        let file = file.unwrap();
        let filename = file.file_name();
        let mut filename = filename.into_string().unwrap();
        if filename.starts_with('.') {
          continue;
        }
        if *curr == height {
          stdout()
            .queue(crossterm::style::SetForegroundColor(
              crossterm::style::Color::White,
            ))
            .unwrap();
          stdout()
            .queue(crossterm::style::SetBackgroundColor(
              crossterm::style::Color::DarkBlue,
            ))
            .unwrap();
          while filename.len() < w as usize {
            filename.push(' ');
          }
        } else {
          stdout()
            .queue(crossterm::style::SetForegroundColor(
              crossterm::style::Color::DarkBlue,
            ))
            .unwrap();
          stdout()
            .queue(crossterm::style::SetBackgroundColor(
              crossterm::style::Color::Black,
            ))
            .unwrap();
          stdout()
            .queue(crossterm::style::SetAttribute(
              crossterm::style::Attribute::Bold,
            ))
            .unwrap();
        }
        if !file.file_type().unwrap().is_dir() {
          stdout()
            .queue(crossterm::style::SetForegroundColor(
              crossterm::style::Color::White,
            ))
            .unwrap();
          stdout()
            .queue(crossterm::style::SetAttribute(
              crossterm::style::Attribute::Reset,
            ))
            .unwrap();
        }
        stdout().queue(crossterm::style::Print(filename)).ok();
        height += 1;
        if height as u16 == h - 3 {
          break;
        }
        stdout()
          .queue(crossterm::cursor::MoveToNextLine(1))
          .unwrap();
      }
    }
  }

  pub fn scroll_down(&mut self) {
    let (_, h) = crossterm::terminal::size().unwrap();
    match self {
      Self::Browser { curr, total, .. } => {
        if *curr < h as usize - 4 && curr < total {
          *curr += 1;
        }
      }
      _ => {}
    }
  }

  pub fn scroll_up(&mut self) {
    match self {
      Self::Browser { curr, .. } => {
        if *curr > 1 {
          *curr -= 1
        }
      }
      _ => {}
    }
  }
}
