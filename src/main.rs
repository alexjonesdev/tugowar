use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::{error::Error, io};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode, self};
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use tugowar::frame::Drawable;
use tugowar::units::UnitType;
use tugowar::{frame, render, player::Player, interface::Interface};

fn main() -> Result <(), Box<dyn Error>> {
    println!("Hello, world!");

    //Terminal Setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player1 = Player::new();
    let mut player2 = Player::new();
    let mut instant = Instant::now();
    let mut interface = Interface::new();

    //Game Loop
    'gameloop: loop {
        //Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();

        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Char('f') => {
                        player1.spawn_unit(UnitType::Fighter);
                        player1.spend_energy(100);
                    }
                    KeyCode::Char('s') => {
                        player1.spawn_unit(UnitType::Shooter);
                        player1.spend_energy(100);
                    }
                    KeyCode::Char('c') => {
                        player1.spawn_unit(UnitType::Cannon);
                        player1.spend_energy(300);
                    }
                    _ => {}
                }
            }
        }

        //Updates
        player1.update(delta);
        player2.update(delta);
        interface.update(player1.hp, player1.en, player2.hp, player2.en);
        
        //Draw & Render
        let drawables: Vec<&dyn Drawable> = vec![&interface];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}