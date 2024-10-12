use crate::Terminal;
use crossterm::event::KeyCode;
use chrono::NaiveDateTime;

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Select {
    pub options: Vec<String>,
    pub selected: u8,
}

pub struct Guide {
    #[allow(dead_code)]
    terminal: Terminal,
    cursor_position: Position,
    select_browsers: Select,
    should_quit: bool,
    is_interrupted: bool,
    selected_browser_index: u8,
}

impl Guide {
    pub fn wait_for_any_key() {
        let mut input = String::new();
        println!("按回车结束...");
        Terminal::read_line(&mut input).unwrap();
    }

    pub fn select_is_test() -> bool {
        let mut input = String::new();
        let ret;
        println!("本次是否测试(输入y或n)?");
        loop {
            print!("> ");
            Terminal::flush().unwrap();
            Terminal::read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if input == "y" {
                ret = true;
                break
            } else if input == "n" {
                ret = false;
                break
            } else {
                println!("输入错误, 重新输入");
                input = String::new();
            }
        }
    
        // 光标向上一行，将>改成实心圆字符
        println!("\x1B[1A\u{25CF} {}", input);
        ret
    }

    pub fn input_use_delay() -> bool {
        let mut input = String::new();
        let ret;
        println!("是否启用网络延迟校正(输入y或n)? 注意: 校正后可能会提前结算, 导致提单失败!");
        loop {
            print!("> ");
            Terminal::flush().unwrap();
            Terminal::read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if input == "y" {
                ret = true;
                break
            } else if input == "n" {
                ret = false;
                break
            } else {
                println!("输入错误, 重新输入");
                input = String::new();
            }
        }
    
        // 光标向上一行，将>改成实心圆字符
        println!("\x1B[1A\u{25CF} {}", input);
        ret
    }

    pub fn input_item_id() -> String {
        let mut input = String::new();
        println!("输入商品ID");
        loop {
            print!("> ");
            Terminal::flush().unwrap();
            Terminal::read_line(&mut input).unwrap();

            input = input.trim().to_string();

            if input.is_empty() {
                println!("商品ID不能为空, 重新输入");
                input = String::new();
            } else if !input.chars().all(char::is_numeric) {
                println!("商品ID只能包含数字, 重新输入");
                input = String::new();
            } else if input.len() != 12 {
                println!("商品ID长度应为12位, 重新输入");
                input = String::new();
            } else {
                break;
            }
        }
        println!("\x1B[1A\u{25CF} {}", input);
        input
    }

    pub fn input_start_time() -> String {
        let mut input = String::new();
        println!("输入提单时间(格式: %Y-%m-%d %H:%M:%S), 例如: 2024-10-08 20:30:00");
        print!("> 今日?(y/n)");
        Terminal::flush().unwrap();
        let mut today = false;

        let pressed_key = Terminal::read_key().unwrap();
        match pressed_key {
            KeyCode::Char('y') => {
                today = true;
                input = chrono::Local::now().format("%Y-%m-%d").to_string();
            }
            _ => (),
        };
        print!("\x1B[1A\x1B[2K\r"); 

        loop {
            print!("> ");
            Terminal::write(&input);
            Terminal::flush().unwrap();
            Terminal::read_line(&mut input).unwrap();
            input = input.trim().to_string();
            let target_time = NaiveDateTime::parse_from_str(&input, "%Y-%m-%d %H:%M:%S");
            match target_time {
                Ok(_) => break,
                Err(e) => {
                    println!("时间格式错误({:?}), 重新输入", e);
                    if today {
                        input = String::from(chrono::Local::now().format("%Y-%m-%d").to_string());
                    } else {
                        input = String::new();
                    }
                }
            }
        }
        println!("\x1B[1A\u{25CF} {}", input);
        input
    }
    

    pub fn run(&mut self) -> String {
        loop {
            if let Err(e) = self.refresh_screen() {
                die(e);
            }
            if self.should_quit {
                break;
            }
            if let Err(e) = self.process_keypress() {
                die(e);
            }
        }
        if self.is_interrupted {
            return "".to_string();
        }
        self.select_browsers.options[self.selected_browser_index as usize].clone()
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            println!("选择已登录taobao账号的浏览器:\r");
            self.draw_select_browsers('\u{25CF}'.to_string().as_str());
            Terminal::cursor_position(&Position { x: 0, y: 5 });
            Terminal::cursor_show();
        } else {
            println!("选择已登录taobao账号的浏览器:\r");
            self.draw_select_browsers(">");
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x,
                y: self.cursor_position.y,
            });
        }

        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            KeyCode::Char('z') => {
                self.should_quit = true;
                self.is_interrupted = true;
            }
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => self.move_cursor(pressed_key),
            KeyCode::Enter => {
                self.selected_browser_index = self.select_browsers.selected;
                self.should_quit = true;
            }
            _ => (),
        };
        Ok(())
    }

    fn draw_select_browsers(&self, symbol: &str) {
        let Select { options, selected } = &self.select_browsers;
        let mut y = 1;
        for (index, option) in options.iter().enumerate() {
            let mut text = "  ".to_string();
            if *selected == index as u8 {
                text = format!("{} ", symbol);
            }
            text.push_str(option);
            if option == "chrome" {
                text.push_str(" (仅macOS)");
            }
            Terminal::cursor_position(&Position { x: 0, y });
            print!("{}", text);
            y += 1;
        }
    }

    fn move_cursor(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                if self.select_browsers.selected == 0 {
                    self.select_browsers.selected = self.select_browsers.options.len() as u8 - 1;
                } else {
                    self.select_browsers.selected = self.select_browsers.selected.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                self.select_browsers.selected = self.select_browsers.selected.saturating_add(1) % self.select_browsers.options.len() as u8;
            }
            _ => (),
        }
    }

    pub fn default() -> Self {
        let browsers = vec!["chrome", "firefox", "edge"];

        Self {
            terminal: Terminal::default().expect("Failed to initialize terminal."),
            cursor_position: Position::default(),
            select_browsers: Select {
                options: browsers.iter().map(|&s| s.to_string()).collect(),
                selected: 0,
            },
            is_interrupted: false,
            selected_browser_index: 0,
            should_quit: false,
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
