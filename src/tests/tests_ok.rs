#[cfg(test)]
mod tests_are_ok {
    use crate::functionalities::structs::{self, DataSet, DEPLACEMENT, LIMIT};

    #[test]
    fn move_to_the_right() {
        let mut plyr = DataSet::new();
        *plyr.posx_mut() = 3;
        *plyr.deplc.to_mut_x() = 1;
        let limit = LIMIT;
        structs::deplc(&mut plyr, &limit).unwrap();
        assert_eq!(plyr.posx(), 4);
    } 
    #[test]
    fn move_to_the_bottom() {
        let mut plyr = DataSet::new();
        *plyr.posy_mut() = 2;
        *plyr.deplc.to_mut_y() = 1;
        let limit = LIMIT;
        structs::deplc(&mut plyr, &limit).unwrap();
        assert_eq!(plyr.posy(), 3);
    }
    #[test]
    fn move_to_the_left() {
        let mut plyr = DataSet::new();
        *plyr.posx_mut() = 3;
        *plyr.deplc.to_mut_x() = -1;
        let limit = LIMIT;
        structs::deplc(&mut plyr, &limit).unwrap();
        assert_eq!(plyr.posx(), 2);
    }
    #[test]
    fn move_to_the_top() {
        let mut plyr = DataSet::new();
        *plyr.posy_mut() = 2;
        *plyr.deplc.to_mut_y() = -1;
        let limit = LIMIT;
        structs::deplc(&mut plyr, &limit).unwrap();
        assert_eq!(plyr.posy(), 1);
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