
pub struct Horse{
    pub name: String
}

impl Horse{
    pub fn say_hello(&self){
        println!("neigh 🐴");
    }

    pub fn get_name(&self) -> String{
        return self.name.clone();
    }
}