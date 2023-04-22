// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlite;

#[tauri::command]
fn derivative_zi(cmp: &str) -> String{
	let conn = sqlite::open("./data/hanyu.db").unwrap();
    let component = cmp.trim();
    let mut derivatives: String = "".to_owned();

	let query = format!("{}{}{}{}{}{}{}",
	"select distinct word from 
	(
			select word from zidian 
		INTERSECT 
			select ch as word from decomp 
				where (ch like '%", component, "%' OR p1 like '%", component, "%' OR p2 like '%", component, "%')
	) result 
	
	join decomp on result.word = decomp.ch 

	order by s asc");
	let querycopy = query.clone();

	match conn.prepare(query){
		Err(_) => {
			return "".to_owned();
		},
		_ => {
			let mut statement = conn.prepare(querycopy).unwrap();
			while let Ok(sqlite::State::Row) = statement.next() {
				derivatives = format!("{}{}", derivatives, statement.read::<String, _>("word").unwrap());
			}
		
			return derivatives
		},
	};
}

#[tauri::command]
fn derivative_ci(cmp: &str) -> String{
	let conn = sqlite::open("./data/hanyu.db").unwrap();
    let component = cmp.trim();
    let mut derivatives: String = "".to_owned();

	let query = format!("{}{}{}",
	"select distinct ci from cidian where ci like '%", component, "%' order by length(ci)");
	let querycopy = query.clone();

	match conn.prepare(query){
		Err(_) => {
			return "".to_owned();
		},
		_ => {
			let mut statement = conn.prepare(querycopy).unwrap();
			while let Ok(sqlite::State::Row) = statement.next() {
				derivatives = format!("{} {}", derivatives, statement.read::<String, _>("ci").unwrap());
			}
		
			return derivatives
		},
	};

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![derivative_zi, derivative_ci])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
