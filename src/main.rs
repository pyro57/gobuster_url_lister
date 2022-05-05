use std::fs;
use std::env;
use std::io:: Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = "
USAGE
gobuster_url_lister /path/to/gobuster/output/file /path/to/save/urls/
    ";
    let mut urls: Vec<String>  = Vec::new();
    let mut accessible_urls: Vec<String> = Vec::new();
    if args.len() != 3{
        print!("{}", usage);
    }
    else{
        let inputfile = &args[1];
        let mut outfile = fs::File::create(&args[2]).expect("Failed to open output file");
        let file_content = fs::read_to_string(inputfile).expect("Error creating reading file");
        let chunks: Vec<&str> = file_content.split("===============================================================\n").collect();
        let mut base = String::new();
        let mut url = String::new();
        let mut path = String::new();
        let mut status = String::new();
        for chunk in chunks{
            if chunk.len() > 0{
                let lines: Vec<&str> = chunk.split("\n").collect();
                for line in lines{
                    if line.contains("[+] Url:                     "){
                        let base_vec: Vec<&str> = line.split("[+] Url:                     ").collect();
                        base = base_vec[1].to_string();
                    }
                    if line.contains("(Status:"){
                        let path_vec: Vec<&str> = line.split("(Status:").collect();
                        path = path_vec[0].trim().to_string();
                        let status_vec: Vec<&str> = path_vec[1].split(")").collect();
                        status = status_vec[0].to_string();
                        println!("{}", status);
                    }
                    if path.len() > 1{
                        if status == " 200".to_string(){
                            url = format!("{}{}", base, path);
                            if accessible_urls.contains(&url.to_string()) == false{
                                accessible_urls.push(url);
                            }
                        }
                        else{
                            url = format!("{}{}", base, path);
                            if urls.contains(&url.to_string()) == false{
                                urls.push(url);
                            }  
                        }
                    }
                    status = String::new();
                    path = String::new();
                }
            }
        }
        
        for url in urls{
            println!("URL = {}", url);
            writeln!(&mut outfile, "{}", url).expect("failed to write line to output file");
        }
        println!("These are accessible check them first!");
        writeln!(&mut outfile, "These are accessible check them first!");
        for url in accessible_urls{
            println!("{}", url);
            writeln!(&mut outfile, "{}", url);
        }
    }
}
