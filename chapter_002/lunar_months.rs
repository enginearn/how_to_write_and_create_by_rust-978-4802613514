use std::collections::HashMap;

fn main() {
    let lunar_months = [
        "睦月", "如月", "弥生", "卯月", "皐月", "水無月", "文月", "葉月", "長月", "神無月", "霜月", "師走"
    ];

    let mut m_map: HashMap<&str, usize> = HashMap::new();

    for (i, m) in lunar_months.iter().enumerate() {
        m_map.insert(m, i + 1);
    }

    println!("睦月 => {}月", m_map["睦月"]);
    println!("如月 => {}月", m_map["如月"]);
    println!("弥生 => {}月", m_map["弥生"]);
    println!("卯月 => {}月", m_map["卯月"]);
    println!("皐月 => {}月", m_map["皐月"]);
    println!("水無月 => {}月", m_map["水無月"]);
    println!("文月 => {}月", m_map["文月"]);
    println!("葉月 => {}月", m_map["葉月"]);
    println!("長月 => {}月", m_map["長月"]);
    println!("神無月 => {}月", m_map["神無月"]);
    println!("霜月 => {}月", m_map["霜月"]);
    println!("師走 => {}月", m_map["師走"]);

}
