use std::io::{self};
use std::process::{Command, Stdio};
use crossterm::style::Stylize;

struct Proyect {
    name: String,
    command: String,
    args: String,
}

fn main() -> std::io::Result<()> {
    println!(); 
    println!("{}", "***************".bold().yellow());
    println!("{}", "Hello from react-proyect-gen !".bold().yellow());
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
    
    let projects: Vec<Proyect> = vec![
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
    
    let mut proyect_selected = String::new();

    loop {
        println!();
        println!("Select the proyect type:");

        for (index, project) in projects.iter().enumerate() {
            println!("{}: {}", index + 1, project.name);
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
        } else if proyect_selected == "7" || proyect_selected == "turborepo" {
            println!();
            println!("You selected: Turborepo react-native starter !");
            let confirm = confirm_action();

            if confirm {
                println!();
                println!("Starting process...");
                println!();

                let mut cmd = Command::new("npx")
                    .arg("create-turbo@latest")
                    .arg("-e")
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
        } else {
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

fn create_proyect (command: &String, args: &String, proyect_name: &String) -> bool {
    let mut cmd = Command::new(command)
        .arg(args)
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
}