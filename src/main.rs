use sqlite;

fn main(){
	let conn = sqlite::open("./src/data/hanyu.db").unwrap();

	let query = "select * from cidian limit 10";
	let mut statement = conn.prepare(query).unwrap();

	while let Ok(sqlite::State::Row) = statement.next() {
	    println!("{}: ", statement.read::<String, _>("ci").unwrap());
	    println!("{}\n", statement.read::<String, _>("explanation").unwrap());
	}
}
