use std::fs::read_to_string;
use xlsxwriter::Workbook;

#[derive(Debug, Clone)]
pub struct DayPlan {
    pub date: String,
    pub week: String,
    pub pen: String,
    pub hanzi: String,
    pub math: String,
    pub english: String,
    pub poem: String,
    pub sport: String,
}

impl DayPlan {
    fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 8 {
            return None;
        }
        Some(Self {
            date: parts[0].trim().to_string(),
            week: parts[1].trim().to_string(),
            pen: parts[2].trim().to_string(),
            hanzi: parts[3].trim().to_string(),
            math: parts[4].trim().to_string(),
            english: parts[5].trim().to_string(),
            poem: parts[6].trim().to_string(),
            sport: parts[7].trim().to_string(),
        })
    }
}

fn read_plan() -> Vec<DayPlan> {
    let text = read_to_string("./assets/data/xiaoyu/plan.txt").expect("读取plan.txt失败");
    text.lines()
        .filter(|l| !l.is_empty())
        .filter_map(DayPlan::from_line)
        .collect()
}

fn split_by_month(plans: &[DayPlan]) -> (Vec<DayPlan>, Vec<DayPlan>, Vec<DayPlan>, Vec<DayPlan>) {
    let mut may = vec![];
    let mut june = vec![];
    let mut july = vec![];
    let mut aug = vec![];
    for p in plans {
        if p.date.starts_with("5.") {
            may.push(p.clone());
        } else if p.date.starts_with("6.") {
            june.push(p.clone());
        } else if p.date.starts_with("7.") {
            july.push(p.clone());
        } else if p.date.starts_with("8.") {
            aug.push(p.clone());
        }
    }
    (may, june, july, aug)
}

fn write_sheet(wb: &Workbook, name: &str, data: &[DayPlan]) {
    let mut ws = wb.add_worksheet(Some(name)).unwrap();
    let headers = [
        "日期", "星期", "控笔", "汉字", "数学", "英语", "古诗", "体能",
    ];
    for (col, h) in headers.iter().enumerate() {
        ws.write_string(0, col as u16, h, None).unwrap();
    }
    for (row, p) in data.iter().enumerate() {
        let r = (row + 1) as u32;
        ws.write_string(r, 0, &p.date, None).unwrap();
        ws.write_string(r, 1, &p.week, None).unwrap();
        ws.write_string(r, 2, &p.pen, None).unwrap();
        ws.write_string(r, 3, &p.hanzi, None).unwrap();
        ws.write_string(r, 4, &p.math, None).unwrap();
        ws.write_string(r, 5, &p.english, None).unwrap();
        ws.write_string(r, 6, &p.poem, None).unwrap();
        ws.write_string(r, 7, &p.sport, None).unwrap();
    }
    // ws.set_column_width(0, 12).unwrap();
    // ws.set_column_width(1, 10).unwrap();
    // ws.set_column_width(2, 26).unwrap();
    // ws.set_column_width(3, 20).unwrap();
    // ws.set_column_width(4, 28).unwrap();
    // ws.set_column_width(5, 40).unwrap();
    // ws.set_column_width(6, 24).unwrap();
    // ws.set_column_width(7, 32).unwrap();
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let plans = read_plan();
    let (may, june, july, aug) = split_by_month(&plans);
    let wb = Workbook::new("学习计划_5-8月.xlsx").unwrap();
    write_sheet(&wb, "5月", &may);
    write_sheet(&wb, "6月", &june);
    write_sheet(&wb, "7月", &july);
    write_sheet(&wb, "8月", &aug);
    wb.close()?;
    println!("✅ Excel 生成成功：学习计划_5-8月.xlsx");
    Ok(())
}
