
  use serde::Deserialize;
  use std::fs::File;
  use std::io::Read;
  
  #[derive(Debug, Deserialize)]
  struct Address {
      street: String,
      city: String,
      country: String,
  }
  
  #[derive(Debug, Deserialize)]
  struct Person {
      first_name: String,
      last_name: String,
      age: u8,
      address: Vec<Address>,
      phone_numbers: Vec<String>,
  }
  
  fn main() {
  // read command line params
  // read env variables
  // fetch first object (file, db, git, etc.?)
  // fetch second object (local file)
  // compare 
  // return result

      // Read the JSON file (replace "text.json" with your actual file path)
      let mut file = File::open("sample/person.json").expect("Failed to open file");
      let mut json_string = String::new();
      file.read_to_string(&mut json_string).expect("Failed to read file");
  
      // Parse the JSON data into a Person object
      let person: Person = serde_json::from_str(&json_string).expect("Failed to parse JSON");
  
      // Now you can access the fields of the Person object
      println!("First Name: {}", person.first_name);
      println!("Last Name: {}", person.last_name);
      println!("Age: {}", person.age);
      for element in &person.address {
          println!("Street: {}", element.street);
          println!("City: {}", element.city);
        println!("Country: {}", element.country);
    }
    println!("Phone Numbers: {:?}", person.phone_numbers[0]);
      println!("Phone Numbers: {:?}", person.phone_numbers[1]);
  }