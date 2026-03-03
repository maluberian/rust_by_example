
#[derive(Debug, Clone, Copy)] enum Project { CookieEating, BirthdayParty, HomeRenovation, BedTime }

fn have_diagram(p: Project) -> Option<Project> {
    match p {
        Project::HomeRenovation => Some(Project::HomeRenovation),
        Project::CookieEating => Some(Project::CookieEating),
        _ => None
    }
}

fn have_specification(p: Project) -> Option<Project> {
    match p {
        Project::CookieEating => Some(Project::CookieEating),
        Project::BirthdayParty => Some(Project::BirthdayParty),
        _ => None

    }
}

fn have_requirements(p: Project) -> Option<Project> {
    match p {
        Project::CookieEating => Some(Project::CookieEating),
        Project::BirthdayParty => Some(Project::BirthdayParty),
        Project::HomeRenovation => Some(Project::HomeRenovation),
        _ => None
    }
}

fn start_work_v2(p: Project) -> Option<Project> {
    have_diagram(p).map(have_diagram).flatten().map(have_specification).flatten().map(have_requirements).flatten()
}

fn start_work_v1(p: Project) -> Option<Project> {
    have_diagram(p).and_then(have_specification).and_then(have_requirements)
}

fn start_work(p: Project) {
    println!("=====================================================");
    println!("V1:");
    match start_work_v1(p) {
        Some(e) => println!("Work can begin on {:?}", e),
        None => println!("Work can NOT begin on {:?}", p),
    }

    println!();
    println!("----");
    println!();

    println!("V2:");
    match start_work_v2(p) {
        Some(e) => println!("Work can begin on {:?}", e),
        None => println!("Work can NOT begin on {:?}", p),
    }
    println!("=====================================================");
    println!();
}

fn main() {
    start_work(Project::CookieEating);
    start_work(Project::BirthdayParty);
    start_work(Project::HomeRenovation);
}