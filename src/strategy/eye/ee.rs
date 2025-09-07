


pub struct EletronicEye {
    params: String


}


impl EletronicEye {
    pub fn new(params: String) -> Self {
        EletronicEye {
            params
        }
    }

    pub fn run(&self) {
        println!("Running Eletronic Eye with params: {}", self.params);
        // Add your strategy logic here
    }
}