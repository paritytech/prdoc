use crate::schema::Schema;

pub struct SchemaCmd;

impl SchemaCmd {
	pub fn run() {
		let schema = Schema::get(true);
		println!("{schema}");
	}
}
