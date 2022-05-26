mod race;
use crate::race::Race;
mod horse;
use crate::horse::Horse;



fn main() {
    let mut race = Race{
        num_horses: 0,
        roster: Vec::new()
    };
    let horse1 = Horse{
        name: "Sandy".to_string()
    };
    let horse2 = Horse{
        name: "Sebastian".to_string()
    };
    race.add_horse(horse1);
    race.add_horse(horse2);
    race.print_roster();

}
