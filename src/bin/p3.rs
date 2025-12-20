
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

impl User {


    fn transfer_user(self, new_email: Option<String>, new_user: Option<String>) -> User {
        User {
            username: match new_user {
              Some(n) => n,
             None => self.username,
         },
         email: match new_email {
             Some(n) => n,
            None => self.email,
           },
        ..self
        }
    }             
    fn print_info(&self) {
        println!("{}",self.username);
        println!("{}",self.email);
        println!("{}", self.sign_in_count)
    }
    fn appear_offline(&mut self) {
        self.active = false
    }
}

#[allow(unused_mut)]    
#[allow(unused_variables)]
fn main() {
    let mut user1 = User {active:true, username: String::from("eexotm"), email: String::from("xve05780@gmail.com"), sign_in_count: 1};
    let mut count = &user1.sign_in_count;
    println!("{count}");
    let mut user2 = create_user(String::from("vxserce"), String::from("vsxersce@gmail.com"));
    let mut user3 = user2.transfer_user(Some(String::from("efeesadk@gmail.com")),None);
    dbg!(&user3);
    println!("{}", user3.active);
    user3.print_info();
    user3.appear_offline();
    println!("{}", user3.active);

}

    fn create_user(user: String, email: String) -> User {
    User {
    active: true,
    username: user,
    email,
    sign_in_count: 1,
    }
    
}