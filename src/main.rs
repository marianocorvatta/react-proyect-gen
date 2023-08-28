use std::io::{self, stdout};
use std::process::{Command, Stdio};

use crossterm::{
    ExecutableCommand,
    cursor, style::Stylize
};

struct Proyect {
    name: String,
    command: String,
    args: String,
}

fn main() -> std::io::Result<()> {
    println!(); 
    println!("{}", "***************".bold().yellow());
    println!("{}", "Hello from React Proyect Generator !".bold().yellow());
    println!("{}", "***************".bold().yellow());
    println!();
    println!("This program will create a new react proyect.");
    println!();

    let mut proyect_name = String::new();

    println!("{}", "Please input your proyect name:".bold());
    println!();

    let mut stdout = stdout();
    stdout.execute(cursor::MoveUp(2)).unwrap();
    stdout.execute(cursor::MoveRight(32)).unwrap();

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
    
    let mut template_selected = String::new();

    loop {
        println!();
        println!("{}", "Select the proyect type:".bold());

        for (index, project) in templates.iter().enumerate() {
            println!("{}: {}", index + 1, project.name);
        }    
        println!();

        io::stdin()
            .read_line(&mut template_selected)
            .expect("Failed to read line");

        template_selected = template_selected.trim().to_string();

        if template_selected == "exit" {
            println!();
            println!("Bye!");
            break;
        }
 
        if template_selected == "1" || template_selected == "nextjs" {
            create_proyect(&templates[0].name, &templates[0].command, &templates[0].args, &proyect_name);
            break;
        } else if template_selected == "2" || template_selected == "vitejs" {
            create_proyect(&templates[1].name, &templates[1].command, &templates[1].args, &proyect_name);
            break;
        } else if template_selected == "3" || template_selected == "t3-app" {    
            create_proyect(&templates[2].name, &templates[2].command, &templates[2].args, &proyect_name);
            break;
        } else if template_selected == "4" || template_selected == "cra" || template_selected == "create-react-app" {
            create_proyect(&templates[3].name, &templates[3].command, &templates[3].args, &proyect_name);
            break;
        } else if template_selected == "5" || template_selected == "react-native" {
            create_proyect(&templates[4].name, &templates[4].command, &templates[4].args, &proyect_name);
            break;
        } else if template_selected == "6" || template_selected == "create-expo-app" {
            create_proyect(&templates[5].name, &templates[5].command, &templates[5].args, &proyect_name);
            break;
        } else if template_selected == "7" || template_selected == "turborepo" {
            create_proyect(&templates[6].name, &templates[6].command, &templates[6].args, &proyect_name);
            break;
        } else {
            println!("{}", "Please select a valid option.".red().bold());
        }
    }
    Ok(())
}

fn create_proyect(name: &String, command: &String, args: &String, proyect_name: &String) -> bool {
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
            return true;
        } else {
            return false;
        }
    } else {
       return false;
    }
}

fn confirm_action() -> bool {
    let mut confirm = String::new();

    println!("{}", "Are you sure? (y/n): ".bold());

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