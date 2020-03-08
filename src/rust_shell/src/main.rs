use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guessing_game() {
    println!("Guess the number!");
    println!("Please guess the number.");    

    let secret_number = rand::thread_rng().gen_range(1, 10);

    loop {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to get the input!");

        // Shadowing the variable named number
        let number: u32 = match number.trim().parse() {
            Ok(integer) => integer,
            Err(_) => continue,
        };

        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed: {}", number);
    }
}

fn takes_ownership(spatial_reference_name: String) { // spatial_reference_name comes into scope
    println!("The spatial reference is {}.", spatial_reference_name);
} // Here, spatial_reference_name goes out of scope and drop is called. The backing memory is freed.

fn makes_copy(wkid: i32) { // wkid comes into scope
    println!("The wkid is {}.", wkid);
} // Here, wkid goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it.
    // spatial_reference_name comes into scope
    let spatial_reference_name = String::from("WGS84");

    // spatial_reference_name is returned and moves out to the calling function
    spatial_reference_name
}

fn takes_and_gives_back(spatial_reference_name: String) -> String { // spatial_reference_name comes into scope
    // spatial_reference_name is returned and moves out to the calling function
    spatial_reference_name
}

fn understanding_ownership() {
    // spatial_reference_name comes into scope
    let spatial_reference_name = String::from("WGS84");

    // spatial_reference_name value is moved into the function
    // and so spatial_reference_name is no longer valid here
    takes_ownership(spatial_reference_name);

    // compile error!
    //println!("{}", spatial_reference_name);

    // wkid comes into scope
    let wkid = 4326;

    // wkid would move into function,
    // but i32 is Copy, so it is okay to still use wkid afterward.
    makes_copy(wkid);

    // no compile error
    println!("The wkid is {} again.", wkid);

    // gives_ownership moves its return value into spatial_reference_name
    let spatial_reference_name = gives_ownership();

    // spatial_reference_name_new comes into scope
    let spatial_reference_name_new = String::from("WGS84");

    // spatial_reference_name is moved into takes_and_gives_back,
    // which also moves its return value into spatial_reference_name_moved
    let spatial_reference_name_moved = takes_and_gives_back(spatial_reference_name_new);
}   // Here, spatial_reference_name_moved goes out of scope and is dropped.
    // spatial_reference_name_new goes out of scope but was moved, so nothing happens.
    // spatial_reference_name goies out of scope and is dropped.

fn use_borrowed(spatial_reference_name: &String) -> i32 { // spatial_reference_name is a reference to a String
    match spatial_reference_name.as_ref() {
        "WGS84" => 4326,
        _ => 0,
    }
} // Here, spatial_reference_name goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change_borrowed(spatial_reference_name: &mut String) -> i32 {
    match spatial_reference_name.as_ref() {
        "WGS84" => 4326,
        "" => { spatial_reference_name.push_str("WGS84"); 4326 }
        _ => 0,
    }
}

fn references_and_borrowing() {
    // spatial_reference_name comes into scope
    let spatial_reference_name = String::from("WGS84");

    // pass a reference to spatial_reference_name into use_borrowed
    // a reference refers to the value of spatial_reference_name but does not own it.
    // because it does not own it, the value it points to will not be dropped when the reference goes out of scope.
    let wkid = use_borrowed(&spatial_reference_name);

    // wkid would move into function,
    // but i32 is Copy, so it is okay to still use wkid afterward.
    makes_copy(wkid);

    // mutable empty_name comes into scope
    // the mutable reference &mut empty_name is passed into change_borrowed
    let mut empty_name = String::from("");
    let wkid = change_borrowed(&mut empty_name);
    // here, empty_name was changed

    // wkid would move into function,
    // but i32 is Copy, so it is okay to still use wkid afterward.
    makes_copy(wkid);
}

fn before_the_hash(text: &str) -> &str {
    // returns the string slice before the hash character
    let bytes = text.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        if b'#' == byte {
            return &text[..index];
        }
    }

    ""
}

fn after_the_hash(text: &str) -> &str {
    // returns the string slice after the hash character
    let bytes = text.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        if b'#' == byte {
            return &text[index+1..];
        }
    }

    ""
}


fn the_slice_type() {
    let topic = "Stay away of:#hashtags!";
    let prefix = before_the_hash(&topic);
    let suffix = after_the_hash(&topic);
    println!("{} {}", prefix, suffix);
}



#[derive(Debug)]
struct Point2d {
    // defines a location in a two dimensional space
    x: f64,
    y: f64,
    wkid: i32
}

fn create_wgs84_point(latitude: f64, longitude: f64) -> Point2d {
    // creates a WGS84 point using two coordinates
    let location = Point2d {
        x: longitude,
        y: latitude,
        wkid: 4326
    };
    return location;
}

fn create_point(x: f64, y: f64, wkid: i32) -> Point2d {
    // create a point using field init shorthand
    // parameters have same name as struct fields
    Point2d {
        x,
        y,
        wkid
    }
}

fn copy_point(source_point: Point2d) -> Point2d {
    // create a deep copy using all the struct fields with .. shorthand
    Point2d {
        ..source_point
    }
}



#[derive(Debug)]
struct NamedLocation {
    // defines a simple named location
    name: String,
    location: Point2d
}

impl NamedLocation {
    // defines the methods which can be used with a NamedLocation
    // you should put all the things you can do with an instance of a type in one impl block
    
    fn is_upper_hemisphere(&self) -> bool {
        match self.location.wkid {
            4326 => 0.0 < self.location.y,
            _ => false
        }
    }

    fn has_same_name(&self, other: &NamedLocation) -> bool {
        return self.name == other.name;
    }

    // associated functions are often used for construction that will create a new instance of the struct
    fn create_dessau() -> NamedLocation {
        // creates a new named location using the coordinates of Dessau
        NamedLocation {
            name: String::from("Dessau"),
            location: create_wgs84_point(51.83604, 12.24283)
        }
    }

    fn create_bonn() -> NamedLocation {
        // creates a new named location using the coordinates of Bonn
        NamedLocation {
            name: String::from("Bonn"),
            location: create_wgs84_point(50.7343800, 7.0954900)
        }
    }
}



fn defining_and_instantiating_structs() {
    let location = create_wgs84_point(51.83604, 12.24283);
    println!("{} {} {}", location.x, location.y, location.wkid);

    let location = create_point(12.24283, 51.83604, location.wkid);
    println!("{} {} {}", location.x, location.y, location.wkid);

    let location = copy_point(location);
    println!("{} {} {}", location.x, location.y, location.wkid);

    // create a simple tuple struct
    #[derive(Debug)]
    struct Location(f64, f64, i32);
    let dessau = Location(location.x, location.y, location.wkid);
    println!("{} {} {}", dessau.0, dessau.1, dessau.2);

    // print the point using the output format "Debug" by using ":?"
    // this only works if you annotate the struct with #[derive(Debug)]
    println!("Location is: {:?}", dessau);

    // Use the associated function for creating a new instance
    let dessau_location = NamedLocation::create_dessau();
    println!("{} is on upper hemisphere? {}", dessau_location.name, dessau_location.is_upper_hemisphere());

    let bonn_location = NamedLocation::create_bonn();
    println!("{:?} has the same name like {:?}? {}", dessau_location, bonn_location, dessau_location.has_same_name(&bonn_location));
}



#[derive(Debug)]
struct SpatialReference {
    // create a simple struct
    name: String,
    wkid: i32
}

impl SpatialReference {
    // create implementations for the SpatialReference struct

    fn create_wgs84() -> SpatialReference {
        SpatialReference {
            name: String::from("WGS84"),
            wkid: 4326
        }
    }
}

#[derive(Debug)]
enum WellKnownSpatialReference {
    // define an WellKnonwSpatialReference enum having different types like None, SpatialReference and String

    Unknown,
    WGS84(SpatialReference),
    WebMercator(String)
}

impl WellKnownSpatialReference {
    // create implementations for the WellKnownSpatialReference enum
    
    fn wkid(&self) -> Option<i32> {
        // returns the wkid only for WellKnownSpatialReference enums having a SpatialReference instance
        // all other enums return None
        match &self {
            WellKnownSpatialReference::WGS84(spatial_reference) => Some(spatial_reference.wkid),
            _ => None
        }
    }
    
    fn has_wkid(&self) -> bool {
        // returns true only if the enum is WellKnownSpatialReference::WGS84
        match &self {
            WellKnownSpatialReference::WGS84(_) => true,
            _ => false
        }
    }
}

fn print_wkid(wellknown_spatial_reference : &WellKnownSpatialReference) {
    // prints whether or not the WellKnownSpatialReference has a wkid
    println!("{:?} has wkid? {}", wellknown_spatial_reference, wellknown_spatial_reference.has_wkid());
    
    // gets the wkid option and only prints the wkid if some wkid was returned
    let wkid_option = wellknown_spatial_reference.wkid();
    match wkid_option {
        Some(wkid) => println!("WKID is: {0}", wkid),
        _ => ()
    }

    // using "if let" when matching only one pattern
    // "if let" is not that verbose like "match"
    if let Some(4326) = wkid_option {
        println!("The world geodetic system strikes back again!");
    }
}

fn defining_an_enum() {
    // create the WGS84 spatial reference
    let wgs84 = SpatialReference::create_wgs84();

    // define the world geodetic system as well known by using the WGS84 spatial reference
    let world_geodetic_system = WellKnownSpatialReference::WGS84(wgs84);

    // define the web mercator system as well known by only using a String
    let web_mercator_system = WellKnownSpatialReference::WebMercator(String::from("WebMercator"));

    // define nonsense
    let pumuckl = WellKnownSpatialReference::Unknown;

    // use the has_wkid method on all defined enums
    print_wkid(&world_geodetic_system);
    print_wkid(&web_mercator_system);
    print_wkid(&pumuckl);
}



fn main() {
    // Seems to work
    defining_an_enum();
}

