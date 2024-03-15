use std::collections::HashSet;

pub fn test_hashset_type() {
    let mut planets_lst = HashSet::from(["Mercury", "Venus", "Earth"]);
    let planet_lst_more = HashSet::from(["Earth", "Mars", "Jupiter"]);

    let planet_diff = planets_lst.difference(&planet_lst_more);
    let planet_symdiff = planets_lst.symmetric_difference(&planet_lst_more);

    // for planet in planet_diff {
    //     println!("Thanks for adding {}", planet);
    // }

    // for planet in planet_symdiff {
    //     println!("Thanks for adding {}", planet);
    // }

    planets_lst.insert("Saturn");
    planets_lst.insert("Uranus");
    planets_lst.insert("Neptune");
    planets_lst.insert("Pluto");

    for planet in planets_lst {
        println!("Thanks for adding {}", planet);
    }
}