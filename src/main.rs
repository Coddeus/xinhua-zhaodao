use sqlite;
use std::io::stdin;

fn main() -> Result<(), ()>{
	let conn = sqlite::open("./src/data/hanyu.db").unwrap();

    let mut component = String::new();
    stdin().read_line(&mut component).unwrap();
    let component = component.trim();

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
	    println!("- {}", statement.read::<String, _>("word").unwrap());
	}

	Ok(())
}
