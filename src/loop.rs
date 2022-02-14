"loop" => {          //Sorry about this mess.... Just thought that a loop function maybe would be kinda fun :)
                
    let mut input = String::new();

    while !input.eq("exit"){

        //Get user input
        input = String::new();
        stdin().read_line(&mut input).expect("Invalid input");
        input = input[0..input.len() - 1].to_string();
        let input_vec = input.split_whitespace().map(str::to_string).collect::<Vec<String>>();

        //Match input with things
        match input_vec[0].as_str() {
            "insert" => {   //Should probably make it so you say for example "insert --name --shape"
                if input_vec.len() == 4{
                    let thing = Thing{
                        name: input_vec[1].to_string(),
                        shape: Some(input_vec[2].to_string()),
                        key: input_vec[3].to_string(),
                    };
                    insert(thing, &file ,&mut table);
                }else{
                    println!("Invalid input...");
                    println!("Input should be in format: insert --name --shape --key");
                }
            },
            "delete" => {
                if input_vec.len() == 2 {
                    let thing = Thing{
                        name: "".to_string(),
                        shape: None,
                        key: input_vec[1].to_string(),
                    };
                    delete(thing, &file_path,&mut table);
                }else{
                    println!("Invalid input...");
                    println!("Input should be in format: delete --key");
                }
        },
        "print" => {
            print_values(&table);
        },
    _ => (),
}

stdout().flush().unwrap();
}
},