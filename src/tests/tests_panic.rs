

#[cfg(test)]
mod tests_panicking {
    use crate::functionalities::structs::DEPLACEMENT;
    use crate::functionalities::structs::DataSet;
    
    #[test]
    #[should_panic]
    fn new_move_aways_from_zero() {
        let mut plyr = DataSet::new();
        *plyr.posy_mut() = 0;
        *plyr.deplc.to_mut_y() = -1;
        let max_y = 3;
        let max_x = 6;
        if let Ok(_) = plyr.new_move(&(max_y, max_x)) {
            panic!("y out of index");
        }
        *plyr.deplc.to_mut_y() = 0;

        *plyr.posx_mut() = 0;
        *plyr.deplc.to_mut_x() = -1;
        if let Err(err) = plyr.new_move(&(max_y, max_x)) {
            panic!("{}", err);
        }
    }
    #[test]
    #[should_panic]
    fn new_move_aways_from_max_limit() {
        let mut plyr = DataSet::new();
        let max_y = 3;
        let max_x = 6;

        *plyr.posy_mut() = max_y;
        *plyr.deplc.to_mut_y() = 1;

        if let Ok(_) = plyr.new_move(&(max_y, max_x)) {
            panic!("y out of index");
        }
        *plyr.deplc.to_mut_y() = 0;

        *plyr.posx_mut() = max_x;
        *plyr.deplc.to_mut_x() = 1;
        if let Err(err) = plyr.new_move(&(max_y, max_x)) {
            panic!("{}", err);
        }
    }

    #[test]
    #[should_panic]
    fn call_new_move_where_to_x_and_to_y_eq_zero() {
        let mut plyr = DataSet::new();
        let max_y = 3;
        let max_x = 6;

        // to_x and to_y == 0

        if let Err(_) = plyr.new_move(&(max_y, max_x)) {
            panic!("y and x == 0");
        }
    }
    #[test]
    #[should_panic]
    fn deplace_index_to_high() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 4;

        p.deplace(&dep, index);
        assert_eq!(p.deplc.to_x(), -1);
    }
}