use std::ascii::StrAsciiExt;
use std::vec::Vec;
use std::collections::HashMap;


#[deriving(Show, Eq, PartialEq, Hash)]
enum Direction {
	East,
	West,
	North,
	South,
	Out,
	In,
	Up,
	Down
}

fn str_to_direction(maybe_direction: &str) -> Option<Direction> {
	match maybe_direction.to_ascii_lower().as_slice() {
		"e" | "east" => Some(East),
		"w" | "west" => Some(West),
		"n" | "north" => Some(North),
		"s" | "south" => Some(South),
		"out" => Some(Out),
		"in" => Some(In),
		"u" | "up" => Some(Up),
		"d" | "down" => Some(Down),
		_ => None
	}
}

fn reverse_direction(direction: Direction) -> Direction {
	match direction {
		West => East,
		East => West,
		North => South,
		South => North,
		Out => In,
		In => Out,
		Up => Down,
		Down => Up
	}
}

#[deriving(Show)]
struct Item<'a> {
	name: &'a str,
	description: &'a str,
}

impl<'a> Item<'a> {
	fn new(name: &'a str, description: &'a str) -> Item<'a> {
		Item {
			name: name,
			description: description
		}
	}
}

#[deriving(Show)]
struct Room<'a> {
	name: &'a str,
	description: &'a str,
	items: Vec<&'a Item<'a>>,
	directions: HashMap<Direction, &'a Room<'a>>,
}

impl<'a> Room<'a> {
	fn new(name: &'a str, description: &'a str) -> Room<'a> {
		Room {
			name: name,
			description: description,
			items: Vec::new(),
			directions: HashMap::new()
		}
	}

	fn link(first: &'a mut Room<'a>, direction_to_second: Direction, second: &'a mut Room<'a>) {
		first.directions.insert(direction_to_second, &*second);
		let opposite_direction = reverse_direction(direction_to_second);
		second.directions.insert(opposite_direction, &*first);
	}

	fn add_item(&mut self, item: &'a Item) {
		self.items.push(item);
	}

	fn can_move(&self, direction: Direction) -> bool {
		self.directions.contains_key(&direction)
	}

	fn room_for_direction(&self, direction: Direction) -> Option<&Room> {
		None
	}
}

fn init_rooms<'b>() -> HashMap<&'static str, Room<'b>> {
	let mut ret: HashMap<&str, Room<'b>> = HashMap::new();

	let start_name = "StartRoom";
	let east_name = "EastRoom";
	let mut start_room = Room::new(start_name,
		"A basic room");
	let mut east_room = Room::new(east_name,
		"A room to the east of the room you started in.");

	Room::link(&mut start_room, East, &mut east_room);

	ret.insert(start_name, start_room);
	ret.insert(east_name, east_room);

	ret
}

fn main() {
	let mut rooms = init_rooms();
	let mut current_room = "StartRoom";
	println!("rooms: {}", rooms);

	let mut input = std::io::stdio::stdin();
	loop {
		print!(">>>>  ");
		let line = input.read_line().ok().expect("Failed to read line!");
		let to_process: Vec<&str> = line.as_slice().trim().split(' ').collect();
		if to_process.len() > 0 && to_process[0].len() > 0 {
			match to_process[0] {
				"go" => {
					match str_to_direction(to_process[1]) {
						None => println!("I don't recognize that direction."),
						Some(direction) => {
							println!("You want to go: {}", direction);
							let room = rooms.find(&current_room).expect("Room does not exist.");
							// let new_room = room
						}
					}
				},
				"exit" | "quit" => break,
				_ => println!("I do not recognize that command.")
			}
		}
	}
}
