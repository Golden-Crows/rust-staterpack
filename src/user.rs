//membuat mod
pub mod user{
    //tambahkan pub kalau mau akses diluar mod
    pub struct User{
        pub name: String,
        pub age: u8,
    }

    impl User{
        pub fn cek_user(&self){
            println!("Halo {} umur {}", self.name, self.age);
        }
    }
}

