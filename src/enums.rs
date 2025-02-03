
#[derive(Debug)]
pub enum India {
    Delhi,
    Mumbai,
    Rajisthan
}

#[derive(Debug)]
pub enum Metrics {
    Gaj(India),
    Sqfeet,
}

#[derive(Debug)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn enumTes() {
    let  home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let guj = Metrics::Gaj(India::Delhi);

    println!("first home = {home:#?} second is = {loopback:#?} and last is = {guj:#?} ");

    let res = match home {
        IpAddr::V4(a, b,c ,d ) => {
            println!("Ipv4 address : {a}");
            a},
        IpAddr::V6(address) => {
            println!("{address}");
            0
        }

    };

    let res2 = match guj {
        Metrics::Gaj(a) => {
            (match a {
                India::Delhi => 110059,
                India::Mumbai => 400001,
                India::Rajisthan => 301001,
            })*2
        }
        Metrics::Sqfeet => 100
    };
    println!("simple match {res}");
    println!("nested match = {res2}");

    let tmep = Some(10);
    println!("{}",tmep.unwrap())
}
