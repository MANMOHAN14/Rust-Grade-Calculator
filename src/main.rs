use std::fs::File;
use std::io::{self, BufWriter};
use printpdf::*;

/// Calculate average marks
fn calculate_average(total_marks: f64, subjects: u32) -> f64 {
    total_marks / subjects as f64
}

/// Assign grade based on average
fn assign_grade(avg: f64) -> &'static str {
    if avg >= 90.0 {
        "A"
    } else if avg >= 75.0 {
        "B"
    } else if avg >= 60.0 {
        "C"
    } else {
        "D"
    }
}

/// Generate PDF Report Card
fn generate_pdf(name: &str, total: f64, subjects: u32, average: f64, grade: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let content = format!(
        "📄 Student Report Card\n
         Name      : {}\n
         Total     : {}\n
         Subjects  : {}\n
         Average   : {:.2}\n
         Grade     : {}",
        name, total, subjects, average, grade
    );

    layer.use_text(content, 14.0, Mm(20.0), Mm(270.0), &font);

    let file = File::create("report_card.pdf").unwrap();
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).unwrap();
}

fn main() {
    let mut name = String::new();
    let mut total_input = String::new();
    let mut subjects_input = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter total marks:");
    io::stdin().read_line(&mut total_input).unwrap();

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut subjects_input).unwrap();

    let name = name.trim();
    let total: f64 = total_input.trim().parse().expect("Enter a valid number");
    let subjects: u32 = subjects_input.trim().parse().expect("Enter a valid number");

    let average = calculate_average(total, subjects);
    let grade = assign_grade(average);

    println!("\n--- Report Card ---");
    println!("Name    : {}", name);
    println!("Total   : {}", total);
    println!("Subjects: {}", subjects);
    println!("Average : {:.2}", average);
    println!("Grade   : {}", grade);

    generate_pdf(name, total, subjects, average, grade);
    println!("\n✅ PDF saved as 'report_card.pdf'");
}
