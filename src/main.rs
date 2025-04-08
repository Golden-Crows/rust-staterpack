use crate::user::user::User;

mod user;

fn main(){
    println!("hello manx");
}

#[test]
fn test(){


    let a: i8 = 10;
    let b: i16 = a as i16;

    println!("pengubahan {}",b);


}

#[test]
fn numeric(){

    let a = 10;
    let b= 30;

    println!("tambah a + b {}",a + b);
    println!("kali a * b {}",a * b);
    println!("bagi b % a {}",b % a);

}

#[test]
fn argumented_assigment(){
    let mut c = 10;
    c += 50;
    println! ("{}", c);
}

#[test]
fn comparison_operator(){
    let nilai = 100;

    if nilai <=75 {
        println!("Dibawah kkm");
    }
    else{
        println!("Bagus");
    }
}

#[test]
fn boolean_operator(){
    let nilai_kkm = 50;
    let nilai_lulus = 75;

    // harus true semua agar lolos
    if nilai_kkm >= 60 && nilai_lulus>= 75 {
        println!("lulus");
    }else{
        println!("Gagal");
    }

    //  salah satu lulus , bisa true
    if nilai_kkm >= 60 || nilai_lulus>= 75 {
        println!("lulus");
    }else{
        println!("Gagal");
    }

}

#[test]
fn contoh_tupple(){
    let data: (i32, f64,  bool)=(10 , 10.5, true);
    println!("{:?}",data);

    let a = data.0;
    let b = data.1;
    let c= data.2;

    println!("{} {} {}", a,b, c);

    //destructuring  tupple
    //let (a,b,c)=data;

    //kalau ada data yg g mau dimbil gunakan _
    //let (a,b,_)=data;

    //mengubah tupple harus pakai variable mut
    let mut hewan: (&str,&str,&str)=("PSHT","PSPHP","PN");
    hewan.2 = "ikspi";
    println!("{:?}",hewan);

}

#[test]
fn array(){
    let g = [1,2,3];

    let a = g[0];
    print!("{}",a);

    let lenght = g.len();
    println!("{}", lenght);

    //array dua dimensi
    let matrix:[[i32;3];2]   =[
        [10,2,5],
        [50,25,7],
    ];

    println!("{}",matrix[1][0]);
}

#[test]
fn constan_contoh(){
    const MAXIMUM:i16=  100;
    println!("{}",MAXIMUM);
}

#[test]
fn if_test(){
    let b: bool = true;

    if b == false{
        println!("auuuu");
    }else{
        println!("gg");
    }

    let nilai = 10;

    if nilai <=9{
        println!("cacat");
    }else if nilai== 10{
        println!("sempurna");
    }else{
        println!("noob");
    }

    //if langsung
    let gg=100;
    let kamu = if gg== 100{
        "kamu gg"
    }else{
        "kamu noob"
    };

    println!("{}",kamu);
}

#[test]
fn test_loop(){
    let mut counter = 0;

    loop{
        counter+=1;
        if counter>=10{
            break;
        }else if counter % 2 == 0{
            continue;
        }
    }
    println!("{}", counter);
}

#[test]
fn test_loop_label(){
    //kasih nama loop label dngan petik '

    let mut number = 1;

    'outer: loop{
        let mut i  = 1;
        loop {
            if number > 11 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);

            i += 1;
            if i > 11 {
                break;
            }
        }
        number+=1;
    }
}

#[test]
fn whil_loop(){
    let mut counter = 0;
    while counter<= 10{
        if counter % 2 == 0 {
            println!("{}", counter);
        }
        counter +=1;
    }
}

#[test]
fn array_iterasi(){
    let pacar: [&str;2]=["Zahra","Siapa"];

    let mut index = 0;
    while index < pacar.len() {
        println!("{}", pacar[index]);
        index+=1;
        println!();
    }

    let sefure: [[&str;2];3]=[
        ["Zahra","Semok"],
        ["Erin", "Hyper"],
        ["Kiki", "Montok"]
    ];

    for i in &sefure{
        for sx in i{
            println!("{}",sx);
        }
        println!();
    }

}

#[test]
fn array_iterasi_for(){
    let pacar: [&str;2]=["Zahra","Body"];

    for tmpk in pacar{
        println!("{}",tmpk);
    }

    let sefure: [[&str;2];3]=[
        ["Zahra","Semok"],
        ["Erin", "Hyper"],
        ["Kiki", "Montok"]
    ];

    for i in &sefure{
        for sx in i{
            println!("{}",sx);
        }
        println!();
    }

}

#[test]
fn range(){
    let kasus: [&str;5]=["Crotin","Zahra","Di","Perut","Nya"];

    let range=0..5;
    println!("Start: {}",range.start);
    println!("End: {}",range.end);

    for i in range{
        println!("{}",kasus[i]);
    }

    /*
        cara lain

        for i in 0..5{
            println!("{}",i);
        }
    */
}

#[test]
fn range_inclusive(){
    let kasus: [&str;5]=["Crotin","Zahra","Di","Perut","Nya"];

    let range=0..=4;
    println!("Start: {}",range.start());
    println!("End: {}",range.end());

    for i in range{
        print!("{}",kasus[i]);
    }
}


//parameter function
fn parameter(pelaku: &str,korban: &str){
    println!("{} Ngentod Brutal Sama {}",pelaku , korban);
}

#[test]
fn cek_param(){
    parameter("Alfa","Zahra");
    println!();
    parameter("Alfa","Kiki");
}

//return value
fn contoh_return(n: i32)->i32{
    if n < 1{
        return 0;
    }

    let mut result = 1;
    for i in 1..=n{
        result *= i;
    }

    result
}

#[test]
fn test_contoh_return(){
  let result = contoh_return(5);

    println!("{}",result);
}

//recursive function
 fn print_text(value: String, times: u32){
    if times == 0{
        return;
    }else{
        println!("{}",value);
    }
    //kita panggil di sini
    print_text(value, times - 1);
}

#[test]
fn test_print_text(){
    print_text(String::from("Alfa"),5);
}

fn factorial_recursive(n: u32)->u32{
    if n<= 1{
        return 1;
    }

    n  * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive(){
    let result = factorial_recursive(5);
    println!("{}",result);
}

//ownership function
fn full_name(namadepan: String, namatengah: String, namabelakang: String)->String{
    format!("{} {} {}", namadepan, namatengah,namabelakang)
}

#[test]
fn test_full_name(){
    let namadepan = String::from("Perkosa");
    let namatengah = String::from("Memek");
    let namabelakang = String::from("Zahra");

    let nama = full_name(namadepan,namatengah,namabelakang);

    println!("{}", nama);

    /*
        println!("{}", namadepan);
        println!("{}", namatengah);
        println!("{}", namabelakang);

        ini semua sudah tidak bisa karena owner dipindahkan
    */
}

//mengembalikan ownership dengan tupple
fn full_name_2(namadepan: String, namatengah: String, namabelakang: String)
    ->(String, String, String,String){

    let fullname= format!("{} {} {}", namadepan, namatengah,namabelakang);

    (namadepan,namabelakang,namatengah,fullname)
}

#[test]
fn test_full_name_2() {
    let namadepan = String::from("Perkosa");
    let namatengah = String::from("Memek");
    let namabelakang = String::from("Zahra");

    let (namadepan,namabelakang,namatengah,fullname) = full_name_2(namadepan, namatengah, namabelakang);

    //kalau gini bisa
    println!("{}", namadepan);
    println!("{}", namatengah);
    println!("{}", namabelakang);
    println!("{}", fullname);
}

//mengambil value tanpa return value ownership
//kalau pakai tupple ribet
fn memek_zahra(warna: &str, bentuk: &str)->String{
    format!("warna mmk nya {} {}", warna,bentuk)

}

#[test]
fn desk_memek(){

    //gunakan & untuk mengambil reference
    let warna = String::from("Pink");
    let bentuk = String::from("Tembem");

    let mmk= memek_zahra(&warna,&bentuk);
    println!("{}",mmk);

    //masih bisa
    println!("{}",warna);
    println!("{}",bentuk);
}

//slice
#[test]
fn slice_array(){
    let array:[i32;10] = [1,2,3,4,5,6,7,8,9,10];
    let slice_1:&[i32] =&array[..] ;
    println!("{:?}",slice_1);

    let slice_2: &[i32]= &array[1..6];
    println!("{:?}",slice_2);

    //ini tidak mengambil kepemilikan karena slice 2 itu reference bukan owner
    let slice_3 = slice_2;
    println!("{:?}",slice_3);

}

#[test]
fn slice_string(){
    let nama: String = String::from("Babayo Yayo Yaya");

    let ambil_sebagian: &str = &nama[1..8];
    print!("{}",ambil_sebagian);
}

//struct <------------------------------------------------->

struct Member{
    nama: String,
    umur: u32,
    alamat: String
}

//menggunakan struct pada function
fn struct_in_function(person: &Member) {
    println!("{}",person.nama);
    println!("{}",person.umur);
    println!("{}",person.alamat);
}

//akses stuct manual
#[test]
fn test_struct() {

    let person: Member = Member{
        nama: String::from("Zahra"),
        umur:20,
        alamat:String::from("jl entah berantah")
    };

    println!("{}",person.nama);
    println!("{}",person.umur);
    println!("{}",person.alamat);

    //akses struct di funtion
    struct_in_function(&person);

}

//init shorthand<----------------------------------------->
#[test]
fn init_shorthand() {
    let firstname=String::from("ZAHRA");

    let zahra: Member = Member{
        nama: firstname,//owner dipindah
        umur: 20,
        alamat:String::from("jeje")
    };


    struct_in_function(&zahra);
}

//struct update

#[test]
fn update_syntax() {
    let firstname=String::from("ZAHRA");

    let zahra: Member = Member{
        nama: firstname,//owner dipindah
        umur: 20,
        alamat:String::from("jeje")
    };

    let zahra_2: Member = Member{..zahra};
    //zahra tidak bisa diakses diganti ownership ke zahra_2

    /*
        cara supaya tetap bisa akses zahra

        let zahra_2: Member = Member{
            nama: zahra.nama.clone(),
            umur: zahra.umur.clone(),
            alamat: zahra.alamat.clone()
        }

        struct_in_function(&zahra);
    */

    struct_in_function(&zahra_2);
}

//tuple struct<------------------------------------------------->
struct GeoPoint(f64, f64);

#[test]
fn tupple_struct() {
    let lokasi = GeoPoint(0000.077,44567.0);
    println!("{}",lokasi.0);
    println!("{}",lokasi.1);
}

//struct tanpa field
struct Nothing;

#[test]
fn nothing() {
    let _:Nothing = Nothing{};

    // _ DI SINI UNTUK VARIABLE YG TDK DIGUNAKAN
}

//method <-------------------------------->
struct Jiko{
    name:String
}

impl Jiko{
    fn jiko_pr(&self){
        println!("Hallo {} ", self.name);
    }
}

#[test]
fn test_method(){
    let wibu: Jiko = Jiko{
        name: String::from("Jancok")
    };

    wibu.jiko_pr();
}

//associated function<---------------------------------->

//struct nya di atas
impl GeoPoint{
    fn new(long: f64 , lat:f64)->GeoPoint{
        GeoPoint(long,lat)
    }
}

#[test]
fn test_associated_function() {
    let geo: GeoPoint = GeoPoint::new(0090808.88,4545.00);
    println!("{} {}" , geo.0, geo.1);
}

//enum <------------------------------------->
enum Role{
    Admin,
    Member
}

#[test]
fn test_enum() {
    let _alfa = Role::Admin;
    let _ = Role::Member;
}

//enum data
enum Payment{
    //cc
    CreditCard(String),
    BankTf(String, String),
    EWallet(String, String)
}
//enum  juga bisa dikasih method
impl Payment{

    //destructuring enum
    fn bayar(&self, amount: u32){

        match self {

            Payment::CreditCard(number)=>{
                println!("Pay number {} jumlah{}" , number , amount);
            }
            Payment::BankTf(bank , number)=>{
                println!("pay with {} number {} jumlah {}", bank , number, amount);
            }
            Payment::EWallet(ewallet, number)=>{
                println!("pay with {} number {} jumlah{}", ewallet , number , amount);
            }

        }
    }
}
#[test]
fn test_pay() {
    let pay = Payment::BankTf(String::from("BCA"),String::from("097773344"));
    let pay_cc = Payment::CreditCard(String::from("99322445"));
    let pay_wallet = Payment::EWallet(String::from("ovo"), String::from("00399392"));

    pay.bayar(5000000);
    pay_cc.bayar(5000000);
    pay_wallet.bayar(5000000);
}

//mengakses data enum <----------------------------------------------------------------------->
#[test]
fn mactch_enum() {
    let admin = Role::Admin;

    match admin{
        Role::Admin=>{
            println!("hello admin");
        }
        Role::Member=>{
            println!("hello member");
        }
    }

}

//pattern matching untuk value
#[test]
fn matching_value() {
    let name = "Alfa";

    match name{
        "Alfa"=>{
            println!("Halo boss");
        }

        "Zahra"|"Lena" => {
            println!("Halo sefure ku ");
        }

        _other=>{
            println!("Elu siapa brok")
        }
    }
}

//range pattern
#[test]
fn range_pattern() {
    let value = 100000;

    match value{
        1..=5 =>{
            println!("Bosok");
        }
        _other=>{
            println!("GG");
        }

    }
}

//destructuring struct pattern
#[test]
fn struct_pattern() {
    let point  = GeoPoint(66.878,456.998);
    match point {
        GeoPoint(long,0.0)=>{
            println!("long {}", long);
        }
        GeoPoint(0.0, lat)=>{
            println!("lat {}", lat);
        }
        GeoPoint(long, lat)=>{
            println!("Long: {} Lat: {}" , long , lat);
        }
    }

    //cara ke 2 untuk struct yg bukan bentuk tupple
    let zahra: Member = Member{
        nama:String::from("Zahra"),
        umur: 20,
        alamat:String::from("JL entah berantah")
    };

    match zahra {
        Member{nama, umur, ..}=>{
            /*
            .. ini adalah ignoring untuk struct biasa
            _ ini adalah ignoring untuk struct tupple
            */
            println!("{} {} ", nama , umur);
        }
    }
}

#[test]
fn test_match_expression() {
    let nilai = 100;
    let result = match nilai {
        0 => {"no nilai"},
        75 => {"kkm"},
        _other =>{"Bagus"}
    };

    println!("{}",result);
}

//type alias<------------------------------------------------>
type Age = u8;
type IdentityNumber = String;

struct Penduduk{
    nik: IdentityNumber,
    umur: Age
}

#[test]
fn test_type_alias() {
    let dadang = Penduduk{
        nik:String::from("994883772"),
        umur:20
    };

    println!("{} {}", dadang.nik, dadang.umur);
}

//akses mod
#[test]
fn akses_mod() {
    //struct User dari file user.rs
    let dadang = User{
        name:String::from("Dadang"),
        age:30
    };

    dadang.cek_user();
}
