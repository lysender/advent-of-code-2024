use glam::IVec2;

pub fn coord_greater(v1: &IVec2, v2: &IVec2) -> bool {
    if v1.x > v2.x {
        // X-axis hass high priority
        return true;
    }
    if v1.x == v2.x {
        return v1.y > v2.y;
    }
    false
}

pub fn create_visited_grid(rows: i32, cols: i32) -> Vec<Vec<bool>> {
    let row: Vec<bool> = vec![false; cols as usize];
    vec![row; rows as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_greater() {
        assert!(coord_greater(&IVec2::new(1, 2), &IVec2::new(1, 1)));
        assert!(coord_greater(&IVec2::new(2, 2), &IVec2::new(1, 1)));
        assert!(coord_greater(&IVec2::new(2, 1), &IVec2::new(1, 1)));
        assert!(coord_greater(&IVec2::new(2, 0), &IVec2::new(1, 2)));
        assert!(!coord_greater(&IVec2::new(1, 1), &IVec2::new(1, 2)));
        assert!(!coord_greater(&IVec2::new(1, 1), &IVec2::new(2, 2)));
        assert!(!coord_greater(&IVec2::new(1, 1), &IVec2::new(2, 1)));
        assert!(!coord_greater(&IVec2::new(1, 2), &IVec2::new(2, 0)));
    }

    #[test]
    fn test_create_visited_grid() {
        assert_eq!(
            create_visited_grid(2, 3),
            vec![vec![false, false, false], vec![false, false, false],]
        );
    }
}
