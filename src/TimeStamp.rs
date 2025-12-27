use chrono::prelude::Local;

fn time_get() -> (i64, i64, i64, i64, String, String, String, String, String, String, String, String, String) {
    let t = Local::now();
    let st = t.timestamp();
    let mt = st / 60;
    let ht = mt / 60;
    let dt = ht / 24;
    let st_bin = format!("{:b}", st);
    let mt_bin = format!("{:b}", mt);
    let ht_bin = format!("{:b}", ht);
    let dt_bin = format!("{:b}", dt);
    let st_hex = format!("{:X}", st);
    let mt_hex = format!("{:X}", mt);
    let ht_hex = format!("{:X}", ht);
    let dt_hex = format!("{:X}", dt);
    let time_str = t.format("%Y-%m-%d %H:%M:%S +08:00").to_string();

    (st, mt, ht, dt, st_bin, mt_bin, ht_bin, dt_bin, st_hex, mt_hex, ht_hex, dt_hex, time_str)
}

fn main() {
    let (st, mt, ht, dt, st_bin, mt_bin, ht_bin, dt_bin, st_hex, mt_hex, ht_hex, dt_hex, time_str) = time_get();
    println!();
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║  当前时间: {:<39}║", time_str);
    println!("╠═══════════════════════════════════════════════════╣");
    println!("║  时间戳(秒): {:<12} | 十六进制: {:<12}║", st, st_hex);
    println!("║  二进制: {:<41}║", st_bin);
    println!("╠═══════════════════════════════════════════════════╣");
    println!("║  时间戳(分): {:<12} | 十六进制: {:<12}║", mt, mt_hex);
    println!("║  二进制: {:<41}║", mt_bin);
    println!("╠═══════════════════════════════════════════════════╣");
    println!("║  时间戳(时): {:<12} | 十六进制: {:<12}║", ht, ht_hex);
    println!("║  二进制: {:<41}║", ht_bin);
    println!("╠═══════════════════════════════════════════════════╣");
    println!("║  时间戳(天): {:<12} | 十六进制: {:<12}║", dt, dt_hex);
    println!("║  二进制: {:<41}║", dt_bin);
    println!("╚═══════════════════════════════════════════════════╝");
    println!();
}
