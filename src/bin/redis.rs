extern crate redis;
use redis::{Client, Commands, Connection, RedisResult};
extern crate time;
use time::PreciseTime;

static query_inactive: &'static str= "entry_inactive";
static query_trim_done: &'static str= "entry_trim_done";
static query_was_fuzzed: &'static str= "entry_was_fuzzed";
static query_passed_det: &'static str= "entry_passed_det";
static query_var_behavior: &'static str= "entry_var_behavior";
static query_favored: &'static str= "entry_favored";

// static aquery_len: &'static str= "_len";
// static aquery_cal_failed: &'static str= "_cal_failed";
// static aquery_bitmap_size: &'static str= "_bitmap_size";
// static aquery_path_stat: &'static str= "_path_stat";
// static aquery_children: &'static str= "_children";
// static aquery_src: &'static str= "_src";
// static aquery_exec_cksum: &'static str= "_exec_cksum";
// static aquery_exec_us: &'static str= "_exec_us";
// static aquery_handicap: &'static str= "_handicap";
// static aquery_depth: &'static str= "_depth";
// static aquery_touch_count: &'static str= "_touch_count";
// static aquery_mut_opt: &'static str= "_mut_opt";


#[derive(Default)]
pub struct QEntry {
    pub inactive: bool, // fake remove
    pub len: u32, // byte size
    ///
    pub cal_failed: u16, // calibration failed times
    pub trim_done: bool,
    pub was_fuzzed: bool,
    pub passed_det: bool,
    pub var_behavior: bool,
    pub favored: bool,
    pub bitmap_size: u32, //
    pub path_stat: u8,
    ///
    pub id: usize,
    pub src: usize,
    pub exec_cksum: u64,
    ///
    pub exec_us: u32, // avg value among different runs
    pub handicap: u16, // used for calculating the score (it is how many cycles this queue item has been skipped for), to prevent starving
    pub depth: u16,
    pub touch_count: u16,
    pub mut_opt: u8,
}

fn store_queue_item(conn: &Connection, entry: &QEntry) {
    let id = entry.id;

    let query_len = format!("{}_len", id);
    let query_cal_failed = format!("{}_cal_failed", id);
    let query_bitmap_size = format!("{}_bitmap_size", id);
    let query_path_stat = format!("{}_path_stat", id);
    let query_children = format!("{}_children", entry.src);
    let query_src = format!("{}_src", id);
    let query_exec_cksum = format!("{}_exec_cksum", id);
    let query_exec_us = format!("{}_exec_us", id);
    let query_handicap = format!("{}_handicap", id);
    let query_depth = format!("{}_depth", id);
    let query_touch_count = format!("{}_touch_count", id);
    let query_mut_opt = format!("{}_mut_opt", id);

    let _: () = conn.set(query_len, entry.len).unwrap();
    let _: () = conn.set(query_cal_failed, entry.cal_failed).unwrap();
    let _: () = conn.set(query_bitmap_size, entry.bitmap_size).unwrap();
    let _: () = conn.set(query_path_stat, entry.path_stat).unwrap();
    let _: () = conn.set(query_src, entry.src).unwrap();
    let _: () = conn.set(query_exec_cksum, entry.exec_cksum).unwrap();
    let _: () = conn.set(query_exec_us, entry.exec_us).unwrap();
    let _: () = conn.set(query_handicap, entry.handicap).unwrap();
    let _: () = conn.set(query_depth, entry.depth).unwrap();
    let _: () = conn.set(query_touch_count, entry.touch_count).unwrap();
    let _: () = conn.set(query_mut_opt, entry.mut_opt).unwrap();

    let _: () = redis::cmd("setbit").arg("query_inactive").arg(id).arg(entry.inactive as u8).query(conn).unwrap();
    let _: () = redis::cmd("setbit").arg("query_trim_done").arg(id).arg(entry.trim_done as u8).query(conn).unwrap();
    let _: () = redis::cmd("setbit").arg("query_was_fuzzed").arg(id).arg(entry.was_fuzzed as u8).query(conn).unwrap();
    let _: () = redis::cmd("setbit").arg("query_passed_det").arg(id).arg(entry.passed_det as u8).query(conn).unwrap();
    let _: () = redis::cmd("setbit").arg("query_var_behavior").arg(id).arg(entry.var_behavior as u8).query(conn).unwrap();
    let _: () = redis::cmd("setbit").arg("query_favored").arg(id).arg(entry.favored as u8).query(conn).unwrap();
    let _: () = conn.sadd(query_children, id).unwrap();

}

fn load_queue_item(conn: &Connection, id: usize) -> Option<QEntry> {
    let query_len = format!("{}_len", id);
    let query_cal_failed = format!("{}_cal_failed", id);
    let query_bitmap_size = format!("{}_bitmap_size", id);
    let query_path_stat = format!("{}_path_stat", id);
    let query_src = format!("{}_src", id);
    let query_exec_cksum = format!("{}_exec_cksum", id);
    let query_exec_us = format!("{}_exec_us", id);
    let query_handicap = format!("{}_handicap", id);
    let query_depth = format!("{}_depth", id);
    let query_touch_count = format!("{}_touch_count", id);
    let query_mut_opt = format!("{}_mut_opt", id);

    let len = conn.get(query_len).unwrap_or(0u32);
    if len == 0 {
        return None;
    }
    let cal_failed = conn.get(query_cal_failed).unwrap_or(0);
    let bitmap_size = conn.get(query_bitmap_size).unwrap_or(0);
    if bitmap_size == 0{
        return None;
    }
    let path_stat = conn.get(query_path_stat).unwrap_or(0);
    let src = conn.get(query_src).unwrap_or(0);
    let exec_cksum = conn.get(query_exec_cksum).unwrap_or(0);
    let exec_us = conn.get(query_exec_us).unwrap_or(0);
    let handicap = conn.get(query_handicap).unwrap_or(0);
    let depth = conn.get(query_depth).unwrap_or(0);
    let touch_count = conn.get(query_touch_count).unwrap_or(0);
    let mut_opt = conn.get(query_mut_opt).unwrap_or(0);

    let inactive = conn.getbit(query_inactive, id).unwrap_or(0) != 0;
    let trim_done = conn.getbit(query_trim_done, id).unwrap_or(0) != 0;
    let was_fuzzed = conn.getbit(query_was_fuzzed, id).unwrap_or(0) != 0;
    let passed_det = conn.getbit(query_passed_det, id).unwrap_or(0) != 0;
    let var_behavior = conn.getbit(query_var_behavior, id).unwrap_or(0) != 0;
    let favored = conn.getbit(query_favored, id).unwrap_or(0) != 0;

    Some(QEntry {
        inactive: inactive,
        len: len,
        ///
        cal_failed: cal_failed,
        trim_done: trim_done,
        was_fuzzed: was_fuzzed,
        passed_det: passed_det,
        var_behavior: var_behavior,
        favored: favored,
        bitmap_size: bitmap_size, //
        path_stat: path_stat,
        ///
        id: id,
        src: src,
        exec_cksum: exec_cksum,
        ///
        exec_us: exec_us, // avg value among different runs
        handicap: handicap, // used for calculating the score (it is how many cycles this queue item has been skipped for), to prevent starving
        depth: depth,
        touch_count: touch_count,
        mut_opt: mut_opt,
    })

}



fn main() {

    let g_start = PreciseTime::now();
    let con_start = PreciseTime::now();
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
    let con_end = PreciseTime::now();
    println!("{} seconds for connecting to server", con_start.to(con_end));

    let entry1 = QEntry {
        inactive: false,
        len: 20,
        cal_failed: 0,
        trim_done: true,
        was_fuzzed: true,
        passed_det: true,
        var_behavior: false,
        favored: true,
        bitmap_size: 256,
        path_stat: 2,
        id: 1,
        src: 0,
        exec_cksum: 67890,
        exec_us: 566,
        handicap: 0,
        depth: 0,
        touch_count: 1,
        mut_opt: 2,
    };
    
    let store_start = PreciseTime::now();
    store_queue_item(&conn, &entry1);
    let store_end = PreciseTime::now();
    println!("{} seconds for storing to server", store_start.to(store_end));

    let load_start = PreciseTime::now();
    let entry2 = load_queue_item(&conn, entry1.id);
    let load_end = PreciseTime::now();
    println!("{} seconds for loading from server", load_start.to(load_end));

    match entry2 {
        Some(e) => {
            println!("\tentry2: id is {}, len is {}", e.id, e.len);
        },
        _ => {
            println!("\tDidn't get entry2");
        }
    }
    
    let g_end = PreciseTime::now();
    println!("{} seconds for the whole run", g_start.to(g_end));
}