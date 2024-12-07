mod todoru;
use todoru::Todoru; //add help later
use std::env;

fn main() {
    let todoru = Todoru::new().expect("Couldn't create the new To-do-ru instance");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1{
        let command = &args[1];
        match &command[..] {
            "list" => todoru.list(),
            "add" => todoru.add(&args[2..]),
            &_ => todo!(),
            //"rm" => todoru.remove(&args[2..]),
            //"done" => todoru.done(&args[2..]),
            //edit" => todoru.edit(&args[2..]),
            //"sort" => todoru.sort(),
            //"reset" => todoru.reset(),
            //"restore" => todoru.restore(),
            //"help" | "--help" | "-h" | _ => help(),
        }
    } else {
        todoru.list();
    }
}
