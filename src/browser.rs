use crate::ui::MyApp;

use webbrowser;

pub fn open_url(url: String) -> bool {
    let final_url = "https://papers.gceguide.com/A%20Levels/".to_string() + &url;

    if webbrowser::open(final_url.as_str()).is_ok() {
        true 
    } else {
        false
    }
}

pub fn contruct_url(data: &MyApp) -> Option<String> {
    let mut url = "".to_string();
    let syllabus_code; 

    url.push_str(&data.subject);
    url.push_str("%20");

    match data.subject.as_str() {
        "Economics" => {
            url.push_str("(9708)/");
            syllabus_code = "9708".to_string()
        }
        "Mathematics" => {
            url.push_str("(9709)/");
            syllabus_code = "9709".to_string();
        }
        "Computer" => {
            url.push_str("(9618)/");
            syllabus_code = "9618".to_string();
        }
        _ => {
            println!("Invalid");
            return None;
        }
    }

    url.push_str(format!("{}/", data.year).as_str());
    url.push_str(syllabus_code.as_str());

    match data.seating {
        crate::ui::PaperSeating::June => url.push_str("_s"),
        crate::ui::PaperSeating::November => url.push_str("_w"),
        crate::ui::PaperSeating::February => url.push_str("_s"),
        crate::ui::PaperSeating::Default => {
            println!("Error processing data.seating");
            return None;
        }
    }

    url.push_str(&data.year.to_string()[2..]);

    match data.mode {
        crate::ui::PaperMode::PastPaper => url.push_str("_qp_"),
        crate::ui::PaperMode::MarkingScheme => url.push_str("_ms_"),
        crate::ui::PaperMode::GradeThreshold => url.push_str("_gt_"),
        crate::ui::PaperMode::Default => {
            println!("Error processig data.mode");
            return None;
        }
    }

    match data.paper {
        1 => url.push_str("12"),
        2 => url.push_str("22"),
        3 => url.push_str("32"),
        4 => url.push_str("42"),
        _ => {
            println!("Error processing data.paper");
            return None;
        }
    }

    url.push_str(".pdf");

    println!("{}", url);

    return Some(url);
}
