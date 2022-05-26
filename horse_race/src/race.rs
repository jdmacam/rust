use crate::horse::Horse;
pub struct Race{
    pub num_horses: u8,
    pub roster: Vec<Horse>
}
impl Race{
    pub fn add_horse(&mut self, h: Horse){
        self.roster.push(h);
    }
    pub fn print_roster(&self){
        for i in &self.roster{
            println!("{}",i.get_name());
        }
    }
}