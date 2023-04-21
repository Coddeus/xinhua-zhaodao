// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlite::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn derivatives(cmp: &str) -> String{
	let conn = sqlite::open("./data/hanyu.db").unwrap();
    let component = cmp.trim();
    let mut derivatives: String = "".to_owned();

	let query = format!("{}{}{}{}{}{}{}",
	"select word from 
	(
			select word from zidian 
		INTERSECT 
			select ch as word from decomp 
				where (ch like '%", component, "%' OR p1 like '%", component, "%' OR p2 like '%", component, "%')
	) result 
	
	join decomp on result.word = decomp.ch 

	order by s asc");
	let mut statement = conn.prepare(query).unwrap();

	while let Ok(sqlite::State::Row) = statement.next() {
	    derivatives = format!("{}{}", derivatives, statement.read::<String, _>("word").unwrap());
	}

    derivatives
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, derivatives])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
