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

//referenc field di struct
