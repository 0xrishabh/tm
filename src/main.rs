use rusqlite::{Connection, params};
use home;
use std::env;
use chrono::{Timelike, DateTime, Local};
use chrono_tz::Tz;


/*
//////////////////////////////////////////////////////////////
                    Database Management
//////////////////////////////////////////////////////////////
*/
struct DB {
    conn: Connection
}
impl DB{
    fn new(conn: Connection) -> DB{
        DB {conn}
    }

    fn create_table(&self) {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS times (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                location        TEXT NOT NULL
            )",
            (),
        ).unwrap();
    }

    fn set(&self, name: &String, location: &String){
        self.conn.execute(
            "INSERT INTO times (name, location) VALUES (?1, ?2)",
            [name, location],
        ).unwrap();
    }


    fn get(&self, name: &str) -> String{
        let mut stmt = self.conn.prepare("SELECT location FROM times WHERE name = ?1").unwrap();
        let mut rows = stmt.query(params![name]).unwrap();
        let row = rows.next().unwrap().expect("DB: get operation failed");
        let location: String = row.get(0).unwrap();
        return location
    }
}

/*
//////////////////////////////////////////////////////////////
                    Helper Function
//////////////////////////////////////////////////////////////
*/
fn get_connection() -> Connection {
    let home = home::home_dir().unwrap();
    let db_path = home.to_str().unwrap().to_owned() + "/locations.db";
    let conn = Connection::open(db_path).unwrap();
    conn
}

fn format_time(zone: &String) -> String {
    let target_timezone = &zone.trim();
    let local_time = Local::now();
    let target_tz: Tz = target_timezone.parse().expect("Invalid timezone");
    let converted_time: DateTime<Tz> = local_time.with_timezone(&target_tz);
    
    let hour = converted_time.time().hour() as i32;
    let formatted_time = converted_time.format("%H:%M:%S");

    let message;
    if hour >= 5 && hour <= 12{
        message = format!("{} {}", formatted_time, "☀️");
    } else if hour >= 12 && hour <= 17 {
        message = format!("{} {}", formatted_time, "🌇");
    } else if hour >= 17 && hour <= 21 {
        message = format!("{} {}", formatted_time, "🌃");
    } else {
        message = format!("{} {}", formatted_time, "🌙");
    }
    message
}

/*
//////////////////////////////////////////////////////////////
                    Main
//////////////////////////////////////////////////////////////
*/

fn main(){
    let conn = get_connection();
    let db = DB::new(conn);
    let args: Vec<_> = env::args().collect();

    db.create_table();
    match args.len(){
        3 => {
            let name = &args[1];
            let zone = &args[2];
            db.set(name, zone);
        }
        2 => {
            let name = &args[1];
            let zone = db.get(name);
            let time = format_time(&zone);
            print!("{}", time);
        }
        _ => { eprintln!("Err: Wrong arguments passed!") }
    }
}



