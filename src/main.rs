
use std::fs::{File, remove_file, read_to_string};
use std::io::Write;
use std::process::Command;
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	let args_arr: [Option<String>;4] = check_args(args);
	let file_stem: &str;
	let input: Vec<char>;
	let mut output: String = String::new();
	match &args_arr[3] {
		Some(e) => {
		file_stem = "bfcode";
		input = e.chars().collect();
		},
		None => {match &args_arr[0] {
			Some(e) => {
				let path = Path::new(e);
				file_stem = path.file_stem().and_then(|stem| stem.to_str()).unwrap_or(&e);
    			input = read_input_file(&e.to_string());
			},
			None => {panic!("give me a file to run!");},
		}},
	}
	match &args_arr[1] {
		Some(_e) => output.push_str("use std::io::{stdout, Write};fn main(){let input:Vec<char>;"),
		None => {
				match &args_arr[2] {
					Some(e) => output.push_str(format!("use std::io::{{stdout, Write}};fn main(){{let s = \"{}\";let input:Vec<char> = s.chars().collect();", e).as_str()),
					None => output.push_str("use std::io::{stdin, stdout, Write};fn main(){let mut s=String::new();print!(\"input: \");let _=stdout().flush();stdin().read_line(&mut s).expect(\"Did not enter a correct string\");if let Some('\\n')=s.chars().next_back(){s.pop();}if let Some('\\r')=s.chars().next_back(){s.pop();}let input:Vec<char>=s.chars().collect();"),
				}
			}
		}
	output.push_str("let mut input_pointer:usize=0;let mut lock=stdout().lock();let mut memory:[u8;30_000]=[0;30_000];let mut pointer_pos:u16=0;");
 
 
	// will panic if incorrect braces
	ensure_correct_braces(&input);
 
	let mut i: usize = 0;
	while i < input.len() {
    	match input[i] {
        	'[' => output.push_str("while memory[pointer_pos as usize] != 0 {"),
        	']' => output.push_str("}"),
        	'.' => output.push_str("write!(lock, \"{}\", memory[pointer_pos as usize] as char).unwrap();"),
        	',' => output.push_str("match input.get(input_pointer){Some(value)=>{memory[pointer_pos as usize]=input[input_pointer] as u8;input_pointer += 1;}, None => (),}"),
        	'+' => {let mut num_concurrent_additions: usize = 1;
            	for j in i+1..input.len() {if input[j]!='+'{i=j-1;break;} num_concurrent_additions += 1;} output.push_str(format!("memory[pointer_pos as usize]=memory[pointer_pos as usize].wrapping_add({});", num_concurrent_additions).as_str())},
        	'-' => {let mut num_concurrent_subtractions: usize = 1;
            	for j in i+1..input.len() {if input[j]!='-'{i=j-1;break;} num_concurrent_subtractions += 1;} output.push_str(format!("memory[pointer_pos as usize]=memory[pointer_pos as usize].wrapping_sub({});", num_concurrent_subtractions).as_str())},
        	'>' => {let mut num_concurrent_move_rights: usize = 1;
            	for j in i+1..input.len() {if input[j]!='>'{i=j-1;break;} num_concurrent_move_rights += 1;} output.push_str(format!("pointer_pos=pointer_pos.wrapping_add({});", num_concurrent_move_rights).as_str())},
        	'<' => {let mut num_concurrent_move_lefts: usize = 1;
            	for j in i+1..input.len() {if input[j]!='<'{i=j-1;break;} num_concurrent_move_lefts += 1;} output.push_str(format!("pointer_pos=pointer_pos.wrapping_sub({});", num_concurrent_move_lefts).as_str())},
        	_ => (),
    	}
    	i += 1;
	}

 	// end of main
 	//output.push_str("write!(lock, \"{}\", memory[pointer_pos as usize]).unwrap();}");
 	output.push('}');

 	let mut output_file = File::create(format!("{}.rs", file_stem))?;
 	output_file.write_all(output.as_bytes())?;
	 
 	let output = Command::new("rustc")
 	.args(&[ "-O", format!("{}.rs", file_stem).as_str(), "-o", file_stem])
 	.output()?;
	remove_file(format!("{}.rs", file_stem))?;
	/*
	if output.status.success() {

    	let execution_output = Command::new(format!("./{}", file_stem))
        	.output()?;
    	remove_file(format!("{}.rs", file_stem))?;
    	if execution_output.status.success() {
        	println!("{}", String::from_utf8_lossy(&execution_output.stdout));
    	} else {
        	println!("execution failed!");
        	println!("error: {}", String::from_utf8_lossy(&execution_output.stderr));
    	}
	}
 	*/
	if !output.status.success() {
    	panic!("compliation failed! error: {}", String::from_utf8_lossy(&output.stderr));
	}
	println!("compliation successful! brainfuck executable saved as {}", file_stem);
	Ok(())
 
 }
 
 fn read_input_file(file_name: &String) -> Vec<char> {
	if !file_name.ends_with(".b") && !file_name.ends_with(".bf") {
    	panic!("give me a brainfuck (.b .bf) file to run!");
	}
    
	let file_contents = read_to_string(file_name)
    	.unwrap_or_else(|err| panic!("could not read file: {}", err));
    
	file_contents.chars().collect()
 }
 
 fn ensure_correct_braces(input: &Vec<char>) {
	let mut counter:usize = 0;
	for i in 0..input.len() {
    	match input[i] {
        	'[' => counter += 1,
        	']' => {if counter == 0 {panic!("unexpected closing bracket at the {}th character!", i+1);}counter -= 1},
        	_ => (),
    	}
	}
	if counter > 0 {
    	panic!("incorrect combination of braces! missing {} closing brackets", counter);
	}
 }

fn check_args(args: Vec<String>) -> [Option<String>;4] {
    let mut arg_index = 1;
    let mut returnArg: [Option<String>;4] = [None, None, None, None];
    if args.len() < 2 {
    	panic!("give me a file to run!");
    } else if args.len() == 2 {
    	if !isValidPath(&args[1]) {
    		panic!("could not find given file!");
    	}
    	returnArg[0] = Some(args[1].clone());
    	arg_index += 1;
    }
    while arg_index < args.len() {
        match &args[arg_index].as_str() {
            &"-file" => {if isValidPath(&args[arg_index+1]) {
            				returnArg[0] = Some(args[arg_index+1].clone());
            				arg_index += 1;
            			} else {
            				panic!("could not find given file!");
            			}},
            &"-noinput" => returnArg[1] = Some("s".to_string()),
            &"-preinput" => {returnArg[2] = Some(args[arg_index+1].clone());
            				arg_index += 1;
            				},
            &"-raw" => { let mut tempStr = String::new();
            			for i in arg_index+1..args.len() {
            				tempStr.push_str(&args[i]);
            			}
            			returnArg[3] = Some(tempStr);
            			arg_index = args.len();
            },
            _ => panic!("unknown argument: {}", args[arg_index]),
        }
        arg_index += 1;
    }
    returnArg
}

fn isValidPath(file_name: &str) -> bool {
    Path::new(file_name).is_file()
}
