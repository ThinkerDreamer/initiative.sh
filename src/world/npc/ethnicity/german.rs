use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Adelhayt", "Affra", "Agatha", "Allet", "Angnes", "Anna", "Apell", "Applonia", "Barbara",
        "Brida", "Brigita", "Cecilia", "Clara", "Cristi", "na", "Dorothea", "Duretta", "Ella",
        "Els", "Elsbeth", "Engel", "Enlein", "Enndlin", "Eva", "Fela", "Fronicka", "Genefe",
        "Geras", "Gerhauss", "Gertrudt", "Guttel", "Helena", "Irmel", "Jonata", "Kate", "rina",
        "Kuen", "Kungund", "Lucia", "Madalena", "Magdalen", "Margret", "Marlein", "Martha",
        "Otilia", "Ottilg", "Peternella", "Reusin", "Sibilla", "Ursel", "Vrsula", "Walpurg",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Albrecht", "Allexander", "Baltasar", "Benedick", "Berhart", "Caspar", "Clas", "Cristin",
        "Cristoff", "Dieterich", "Engelhart", "Erhart", "Felix", "Frantz", "Fritz", "Gerhart",
        "Gotleib", "Hans", "Hartmann", "Heintz", "Herman", "Jacob", "Jeremias", "Jorg", "Karil",
        "Kilian", "Linhart", "Lorentz", "Ludwig", "Marx", "Melchor", "Mertin", "Michel", "Moritz",
        "Osswald", "Ott", "Peter", "Rudolff", "Ruprecht", "Sewastian", "Sigmund", "Steffan",
        "Symon", "Thoman", "Ulrich", "Vallentin", "Wendel", "Wilhelm", "Wolff", "Wolfgang",
    ];
}

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        match gender {
            Gender::Masculine => {
                Self::MASCULINE_NAMES[rng.gen_range(0..Self::MASCULINE_NAMES.len())].to_string()
            }
            Gender::Feminine => {
                Self::FEMININE_NAMES[rng.gen_range(0..Self::FEMININE_NAMES.len())].to_string()
            }
            _ => {
                let dist =
                    WeightedIndex::new(&[Self::MASCULINE_NAMES.len(), Self::FEMININE_NAMES.len()])
                        .unwrap();
                if dist.sample(rng) == 0 {
                    Self::gen_name(rng, age, &Gender::Masculine)
                } else {
                    Self::gen_name(rng, age, &Gender::Feminine)
                }
            }
        }
    }
}

#[cfg(test)]
mod test_generate_for_ethnicity {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn gen_name_test() {
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let age = Age::Adult(0);
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::Trans;

        assert_eq!(
            ["Albrecht", "Thoman", "Lucia", "Helena", "Hans", "Berhart"],
            [
                Ethnicity::gen_name(&mut rng, &age, &m),
                Ethnicity::gen_name(&mut rng, &age, &m),
                Ethnicity::gen_name(&mut rng, &age, &f),
                Ethnicity::gen_name(&mut rng, &age, &f),
                Ethnicity::gen_name(&mut rng, &age, &t),
                Ethnicity::gen_name(&mut rng, &age, &t),
            ]
        );
    }
}
