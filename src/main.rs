use std::io::{self, stdout};
use std::process::{Command, Stdio};

use crossterm::event::KeyEventKind;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEvent},
    cursor, style::Stylize, style,
};

struct Proyect {
    name: String,
    command: String,
    args: String,
}

const MENU: &str = r#"
***************

 Select the proyect type:

1. nextjs
2. vitejs
3. t3-app
4. create-react-app
5. react-native
6. create-expo-app
7. turborepo

Select type to create a new proyect or press 'q' to quit.

***************
"#;

fn main() -> std::io::Result<()> {
    println!(); 
    println!("{}", "***************".bold().yellow());
    println!("{}", "Hello from React Proyect Generator !".bold().yellow());
    println!("{}", "***************".bold().yellow());
    println!();
    println!("This program will create a new react proyect.");
    println!();

    let mut proyect_name = String::new();

    println!("{}", "Please input your proyect name ->".bold());
    println!();

    let mut stdout = stdout();
    stdout.execute(cursor::MoveUp(2)).unwrap();
    stdout.execute(cursor::MoveRight(34)).unwrap();

    io::stdin()
        .read_line(&mut proyect_name)
        .expect("Failed to read line");

    proyect_name = proyect_name.trim().to_string();
    
    let templates: Vec<Proyect> = vec![
        Proyect {
            name: String::from("nextjs"),
            command: String::from("npx"),
            args: String::from("create-next-app@latest"),
        },
        Proyect {
            name: String::from("vitejs"),
            command: String::from("npm"),
            args: String::from("create vite@latest"),
        },
        Proyect {
            name: String::from("t3-app"),
            command: String::from("npm"),
            args: String::from("create t3-app@latest"),
        },
        Proyect {
            name: String::from("create-react-app"),
            command: String::from("npx"),
            args: String::from("create-react-app"),
        },
        Proyect {
            name: String::from("react-native"),
            command: String::from("npx"),
            args: String::from("react-native@latest init"),
        },
        Proyect {
            name: String::from("create-expo-app"),
            command: String::from("npx"),
            args: String::from("create-expo-app"),
        },
        Proyect {
            name: String::from("turborepo"),
            command: String::from("npx"),
            args: String::from("create-turbo@latest -e"),
        },
    ];
    
    loop {
        println!();
        println!("{}", "Select the proyect type:".bold());

        // ToDo: Hide cursor and make options selectable
        // ToDo: if the user no confim the action the program should quit
        // maybe remove the loop
        
        // for (index, project) in templates.iter().enumerate() {
        //     println!("{}: {}", index + 1, project.name);
        // }


        for (index, line) in MENU.split('\n').enumerate() {
            match index {
                1 | 2 | 3 | 4 => stdout.execute(style::Print(line)).unwrap(),
                5 => stdout.execute(style::Print(line.red())).unwrap(),
                6 => stdout.execute(style::Print(line.yellow())).unwrap(),
                7 => stdout.execute(style::Print(line.green())).unwrap(),
                8 => stdout.execute(style::Print(line.blue())).unwrap(),
                9 => stdout.execute(style::Print(line.magenta())).unwrap(),
                10 => stdout.execute(style::Print(line.cyan())).unwrap(),
                11 | 12 | 13 | 14 | 15 => stdout.execute(style::Print(line)).unwrap(),
                _ => stdout.execute(style::Print(line)).unwrap(),
            };
            stdout.execute(cursor::MoveToNextLine(1)).unwrap();
        }
        println!();

        match read_char()? {
            '1' => create_proyect(&templates[0].name, &templates[0].command, &templates[0].args, &proyect_name),
            '2' => create_proyect(&templates[1].name, &templates[1].command, &templates[1].args, &proyect_name),
            '3' => create_proyect(&templates[2].name, &templates[2].command, &templates[2].args, &proyect_name),
            '4' => create_proyect(&templates[3].name, &templates[3].command, &templates[3].args, &proyect_name),
            '5' => create_proyect(&templates[4].name, &templates[4].command, &templates[4].args, &proyect_name),
            '6' => create_proyect(&templates[5].name, &templates[5].command, &templates[5].args, &proyect_name),
            '7' => create_proyect(&templates[6].name, &templates[6].command, &templates[6].args, &proyect_name),
            'q' => {
                println!();
                println!("Bye!");
                break;
            },
            _ => println!("Please select a valid option."),
        }
    }
    Ok(())
}

fn create_proyect(name: &String, command: &String, args: &String, proyect_name: &String) {
    println!();
    println!("You selected: {} !", name);

    let confirm = confirm_action();

    println!();
    println!("Starting process...");
    println!();
    
    if confirm {
        let mut cmd = Command::new(command)
            .args(args.split_whitespace())
            .arg(proyect_name)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to execute process");

        let status = cmd.wait().expect("failed to wait for child process");
        
        if status.success() {
            return;
        } else {
            return;
        }
    } else {
       return;
    }
}

fn confirm_action() -> bool {
    let mut confirm = String::new();
    let mut stdout = stdout();

    println!("{}", "Are you sure? (y/n): ".bold());
    stdout.execute(cursor::MoveUp(1)).unwrap();
    stdout.execute(cursor::MoveRight(21)).unwrap();

    io::stdin()
        .read_line(&mut confirm)
        .expect("Failed to read line");

    confirm = confirm.trim().to_string();

    if confirm == "y" || confirm == "yes" {
        return true;
    } else {
        return false;
    }
}

pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}