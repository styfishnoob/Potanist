use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

type BootTimeSumMap = HashMap<u16, Vec<(u8, u8, u8, u8)>>;

pub fn build_boot_time_sum_map() -> BootTimeSumMap {
    let mut map: BootTimeSumMap = HashMap::new();

    for month in 1..=12 {
        let max_day = match month {
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };

        for day in 1..=max_day {
            for min in 0..=59 {
                for sec in 15..=59 {
                    let value = (month as u16) * (day as u16) + (min as u16) + (sec as u16);
                    let candidates = map.entry(value).or_insert_with(Vec::new);

                    // すでに同じ month/day のものがあるか確認
                    if let Some(existing) =
                        candidates.iter_mut().find(|e| e.0 == month && e.1 == day)
                    {
                        // sec が 15 に近い方を優先
                        if (sec as i16 - 15).abs() < (existing.3 as i16 - 15).abs() {
                            *existing = (month, day, min, sec);
                        }
                    } else {
                        // 初めての (month, day) 組み合わせなら追加
                        candidates.push((month, day, min, sec));
                    }
                }
            }
        }
    }

    map
}

pub fn save_json(map: &BootTimeSumMap, path: &str) {
    let file = File::create(path).expect("Failed to create JSON file");
    serde_json::to_writer_pretty(file, map).expect("Failed to write JSON");
}

pub fn save_bincode(map: &BootTimeSumMap, path: &str) {
    let encoded = bincode::serialize(map).expect("Failed to encode");
    std::fs::write(path, encoded).expect("Failed to write bincode");
}

pub fn load_json(path: &str) -> BootTimeSumMap {
    let file = File::open(path).expect("Failed to open JSON file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse JSON")
}

pub fn load_bincode(path: &str) -> BootTimeSumMap {
    let encoded = std::fs::read(path).expect("Failed to read bincode");
    bincode::deserialize(&encoded).expect("Failed to decode")
}
