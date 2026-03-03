
#[derive(Debug)] enum Meat { Hamburger, Steak, BeefTongue, Chicken  }
#[derive(Debug)] enum Cook { R, MR, M, MW, W }
#[derive(Debug)]
struct Protein  {
    meat: Option<Meat>,
    cook: Option<Cook>
}

#[derive(Debug)]
enum Potato { Fries, Baked, Hashed }


#[derive(Debug)]
struct Plate {
    protein: Option<Protein>,
    potato: Option<Potato>,
}

fn cook_r(p: Option<Protein>) -> Option<Protein> { match p { Some(p) => Some(Protein{ meat: p.meat, cook: Some(Cook::R) }), None => None } }
fn cook_mr(p: Option<Protein>) -> Option<Protein> { match p { Some(p) => Some(Protein{ meat: p.meat, cook: Some(Cook::MR) }), None => None } }
fn cook_m(p: Option<Protein>) -> Option<Protein> { match p { Some(p) => Some(Protein{ meat: p.meat, cook: Some(Cook::M) }), None => None } }
fn cook_mw(p: Option<Protein>) -> Option<Protein> { match p { Some(p) => Some(Protein{ meat: p.meat, cook: Some(Cook::MW) }), None => None } }
fn cook_w(p: Option<Protein>) -> Option<Protein> { match p { Some(p) => Some(Protein{ meat: p.meat, cook: Some(Cook::W) }), None => None } }

fn plate_protein(p: Option<Protein>) -> Option<Plate> {
    match p {
        Some(p) => Some(Plate {protein: Some(p), potato: None}),
        None => None
    }
}

fn plate_baked(p: Option<Plate>) -> Option<Plate> { match p { Some(p) => Some(Plate{ protein: p.protein, potato: Some(Potato::Baked) }), None => None } }
fn plate_fries(p: Option<Plate>) -> Option<Plate> { match p { Some(p) => Some(Plate{ protein: p.protein, potato: Some(Potato::Fries) }), None => None } }
fn plate_hash(p: Option<Plate>) -> Option<Plate> { match p { Some(p) => Some(Plate{ protein: p.protein, potato: Some(Potato::Hashed) }), None => None } }

fn main() {
    println!("{:?}", cook_r(Some(Protein{meat: Some(Meat::Hamburger), cook: None})).map(|p| cook_mr(Some(p))).map(|m| plate_protein(m)).map(|p| plate_fries(p)));
    println!("{:?}", cook_mr(Some(Protein{meat: Some(Meat::Steak), cook: None})).map(|p| cook_r(Some(p))).map(|m| plate_protein(m)).map(|p| plate_baked(p)));
}