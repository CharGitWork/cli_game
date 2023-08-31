

#[cfg(test)]
mod tests_panicking {
    use crate::functionalities::structs::{self, DEPLACEMENT, DataSet, LIMIT};
    
    #[test]
    #[should_panic]
    fn deplc_aways_from_min_limit() {
        // plyr.posy == 0
        let mut plyr = DataSet::new();
        *plyr.deplc.to_mut_y() = -1;
        let limit = LIMIT;
        // deplc return a Err(ErrorView)
        if let Ok(_) = structs::deplc(&mut plyr, &limit) {
            panic!("y out of index");
        }
        // init back to 0 or we'll have some bug
        *plyr.deplc.to_mut_y() = 0;

        // here it's the same, plyr.posx == 0
        *plyr.deplc.to_mut_x() = -1;
        // 0 - 1 in Y like X run to a Err(ErrorView)
        if let Err(err) = structs::deplc(&mut plyr, &limit) {
            panic!("{}", err);
        }
    }

    #[test]
    #[should_panic]
    fn deplc_aways_from_max_limit() {
        let mut plyr = DataSet::new();
        let limit = LIMIT;
        // plyr.posy == 4
        *plyr.posy_mut() = limit[1][1] - 1;
        *plyr.deplc.to_mut_y() = 1;
        // deplc return a Err(ErrorView)
        if let Ok(_) = structs::deplc(&mut plyr, &limit) {
            panic!("y out of index");
        }
        // init back to 0 or we'll have some bug
        *plyr.deplc.to_mut_y() = 0;

        // plyr.posx == 4
        *plyr.posx_mut() = limit[0][1] - 1;

        *plyr.deplc.to_mut_x() = 1;
        // 4 + 1 in Y like X run to a Err(ErrorView)
        if let Err(err) = structs::deplc(&mut plyr, &limit) {
            panic!("{}", err);
        }
    }

    #[test]
    #[should_panic]
    fn deplc_where_to_x_and_to_y_eq_zero() {
        let mut plyr = DataSet::new();
        let limit = LIMIT;

        // to_x and to_y == 0

        if let Err(_) = structs::deplc(&mut plyr, &limit) {
            panic!("y and x == 0");
        }
    }
    #[test]
    #[should_panic]
    fn calling_deplace_method_with_a_invalid_index() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 4;

        // panic for a wrond user entry
        p.deplace(&dep, index);
    }
}