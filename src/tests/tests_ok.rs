#[cfg(test)]
mod tests_are_ok {
    use crate::functionalities::structs::DataSet;
    use crate::functionalities::structs::DEPLACEMENT;

    #[test]
    fn move_to_the_right() {
        let mut plyr = DataSet::new();
        *plyr.posx_mut() = 5;
        *plyr.deplc.to_mut_x() = 1;
        let max_x = 6;
        plyr.new_move(&(3, max_x)).unwrap();
        assert_eq!(plyr.posx(), max_x);
    }
    #[test]
    fn move_to_the_left() {
        let mut plyr = DataSet::new();
        *plyr.posx_mut() = 1;
        *plyr.deplc.to_mut_x() = -1;
        plyr.new_move(&(3, 6)).unwrap();
        assert_eq!(plyr.posx(), 0);
    }
    #[test]
    fn move_to_the_bottom() {
        let mut plyr = DataSet::new();
        *plyr.posy_mut() = 2;
        *plyr.deplc.to_mut_y() = 1;
        let max_y = 3;
        plyr.new_move(&(max_y, 6)).unwrap();
        assert_eq!(plyr.posy(), max_y);
    }
    #[test]
    fn move_to_the_up()  {
        let mut plyr = DataSet::new();
        *plyr.posy_mut() = 1;
        *plyr.deplc.to_mut_y() = -1;
        let max_y = 3;
        plyr.new_move(&(max_y, 6)).unwrap();
        assert_eq!(plyr.posy(), 0);
    }
    #[test]
    fn new_move_to_x_non_eq_zero() {
        let mut plyr = DataSet::new();
        // to_y == 0
        *plyr.deplc.to_mut_x() = 1;
        let max_y = 3;
        let max_x = 6;
        if let Err(e) = plyr.new_move(&(max_y, max_x)) {
            panic!("{}", e);
        }
    }
    #[test]
    fn new_move_to_y_non_eq_zero() {
        let mut plyr = DataSet::new();
        // to_x == 0
        *plyr.deplc.to_mut_y() = 1;
        let max_y = 3;
        let max_x = 6;
        if let Err(e) = plyr.new_move(&(max_y, max_x)) {
            panic!("{}", e);
        }
    }
    #[test]
    fn deplace_up() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 0;

        p.deplace(&dep, index);
        assert_eq!(p.deplc.to_y(), -1);
    }
    #[test]
    fn deplace_right() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 1;

        p.deplace(&dep, index);
        assert_eq!(p.deplc.to_x(), 1);
    }
    #[test]
    fn deplace_down() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 2;

        p.deplace(&dep, index);
        assert_eq!(p.deplc.to_y(), 1);
    }
    #[test]
    fn deplace_left() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 3;

        p.deplace(&dep, index);
        assert_eq!(p.deplc.to_x(), -1);
    }
}