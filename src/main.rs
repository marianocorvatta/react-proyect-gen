use std::io::{self};
use std::process::{Command, Stdio};
use crossterm::style::Stylize;

fn main() -> std::io::Result<()> {
    println!(); 
    println!("{}", "***************".bold().yellow());
    println!("{}", "Hello, world from create code proyect!".bold().yellow());
    println!("{}", "***************".bold().yellow());
    println!();
    println!("This program will create a new react proyect.");
    println!(); 

    let mut proyect_name = String::new();

    println!("Please input your proyect name...");
    println!();

    io::stdin()
        .read_line(&mut proyect_name)
        .expect("Failed to read line");
        
    let proyect_types = ["nextjs", "vitejs", "t3-app", "create-react-app", "react-native", "create-expo-app"];
    let mut proyect_selected = String::new();

    loop {
        println!();
        println!("Select the proyect type:");

        for (index, proyect) in proyect_types.iter().enumerate() {
            println!("{}. {}", index + 1, proyect);
        }
        println!();

        io::stdin()
            .read_line(&mut proyect_selected)
            .expect("Failed to read line");

        proyect_selected = proyect_selected.trim().to_string();

        if proyect_selected == "exit" {
            println!();
            println!("Bye!");
            break;
        }
 
        if proyect_selected == "1" || proyect_selected == "nextjs" {
            println!();
            println!("You selected: nextjs !");
            let confirm = confirm_action();

            if confirm {
                println!();
                println!("Starting process...");
                println!();

                let mut cmd = Command::new("npx")
                    .arg("create-next-app@latest")
                    .arg(&proyect_name)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("failed to execute process");

                let status = cmd.wait().expect("failed to wait for child process");
                
                if status.success() {
                    println!("{}", "Proyect created !".green().bold());
                    println!();
                    println!("To run the proyect:");
                    println!("cd {}", proyect_name);
                    println!("npm run dev");
                } else {
                    println!("{}", "Command failed".red().bold());
                }
            }
            
            break;
        } else if proyect_selected == "2" || proyect_selected == "vitejs" {
            println!();
            println!("You selected: vitejs !");
            let confirm = confirm_action();

            if confirm {
                println!();
                println!("Starting process...");
                println!();

                let mut cmd = Command::new("npm")
                    .arg("create")
                    .arg("vite@latest")
                    .arg(&proyect_name)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("failed to execute process");

                let status = cmd.wait().expect("failed to wait for child process");
                
                if status.success() {
                    println!("{}", "Proyect created !".green().bold());
                    println!();
                    println!("To run the proyect:");
                    println!("cd {}", proyect_name);
                    println!("npm install");
                    println!("npm run dev");
                } else {
                    println!("{}", "Command failed".red().bold());
                }
            }
           
            break;
        } else if proyect_selected == "3" || proyect_selected == "t3-app" {    
            println!();
            println!("You selected: t3-app !");
            let confirm = confirm_action();

            if confirm {
                println!();
                println!("Starting process...");
                println!();

                let mut cmd = Command::new("npm")
                    .arg("create")
                    .arg("t3-app@latest")
                    .arg(&proyect_name)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("failed to execute process");

                let status = cmd.wait().expect("failed to wait for child process");
                
                if status.success() {
                    println!("{}", "Proyect created !".green().bold());
                    println!();
                    println!("To run the proyect:");
                    println!("cd {}", proyect_name);
                    println!("npm start");
                } else {
                    println!("{}", "Command failed".red().bold());
                }
            }

            break;
        } else if proyect_selected == "4" || proyect_selected == "cra" || proyect_selected == "create-react-app" {
            println!();
            println!("You selected: create-react-app !");
            let confirm = confirm_action();

            if confirm {
                println!();
                println!("Starting process...");
                println!();

                let mut cmd = Command::new("npx")
                    .arg("create-react-app")
                    .arg(&proyect_name)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("failed to execute process");

                let status = cmd.wait().expect("failed to wait for child process");
                
                if status.success() {
                    println!("{}", "Proyect created !".green().bold());
                    println!();
                    println!("To run the proyect:");
                    println!("cd {}", proyect_name);
                    println!("npm start");
                } else {
                    println!("{}", "Command failed".red().bold());
                }
            }

            break;
        } else if proyect_selected == "5" || proyect_selected == "react-native" {
            println!();
            println!("You selected: react-native !");
            let confirm = confirm_action();

            if confirm {
                println!();
                println!("Starting process...");
                println!();

                let mut cmd = Command::new("npx")
                    .arg("react-native@latest")
                    .arg("init")
                    .arg(&proyect_name)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("failed to execute process");

                let status = cmd.wait().expect("failed to wait for child process");
                
                if status.success() {
                    println!("{}", "Proyect created !".green().bold());
                    println!();
                    println!("To run the proyect:");
                    println!("cd {}", proyect_name);
                    println!("npx expo start");
                    println!("and follow the instructions in this link https://reactnative.dev/docs/environment-setup?guide=quickstart&os=windows&platform=android");
                } else {
                    println!("{}", "Command failed".red().bold());
                }
            }

            break;
        } else if proyect_selected == "6" || proyect_selected == "create-expo-app" {
            println!();
            println!("You selected: create-expo-app !");
            let confirm = confirm_action();

            if confirm {
                println!();
                println!("Starting process...");
                println!();

                let mut cmd = Command::new("npx")
                    .arg("create-expo-app")
                    .arg(&proyect_name)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("failed to execute process");

                let status = cmd.wait().expect("failed to wait for child process");
                
                if status.success() {
                    println!("{}", "Proyect created !".green().bold());
                    println!();
                    println!("To run the proyect:");
                    println!("Follow the instructions in this link https://reactnative.dev/docs/environment-setup?guide=native");
                } else {
                    println!("{}", "Command failed".red().bold());
                }
            }

            break;
        }else {
            println!("{}", "Please select a valid option.".red().bold());
        }
    }
    Ok(())
}

fn confirm_action() -> bool {
    let mut confirm = String::new();

    println!("Are you sure? (y/n): ");

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