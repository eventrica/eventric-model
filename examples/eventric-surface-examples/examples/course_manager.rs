use std::error::Error;

use derive_more::Debug;
use fancy_constructor::new;

#[derive(new, Debug)]
pub struct CourseRegistered {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("course manager...");

    Ok(())
}
