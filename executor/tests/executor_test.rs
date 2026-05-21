use executor::{Executor, Pose};

#[test]
fn test_bm_e() {
    let mut e = Executor::with_pose(Pose::new(0,0,'E'));
    e.execute("BM");
    assert_eq!(e.query(), Pose::new(-1,0,'E'));
}

#[test]
fn test_bl_e() {
    let mut e = Executor::with_pose(Pose::new(0,0,'E'));
    e.execute("BL");
    assert_eq!(e.query(), Pose::new(0,0,'S'));
}

#[test]
fn test_br_e() {
    let mut e = Executor::with_pose(Pose::new(0,0,'E'));
    e.execute("BR");
    assert_eq!(e.query(), Pose::new(0,0,'N'));
}

#[test]
fn test_bbm_n() {
    let mut e = Executor::with_pose(Pose::new(0,0,'N'));
    e.execute("BBM");
    assert_eq!(e.query(), Pose::new(0,1,'N'));
}

#[test]
fn test_fm_e() {
    let mut e = Executor::with_pose(Pose::new(0,0,'E'));
    e.execute("FM");
    assert_eq!(e.query(), Pose::new(2,0,'E'));
}

#[test]
fn test_bfm_e() {
    let mut e = Executor::with_pose(Pose::new(0,0,'E'));
    e.execute("BFM");
    assert_eq!(e.query(), Pose::new(-2,0,'E'));
}

#[test]
fn test_fl_e() {
    let mut e = Executor::with_pose(Pose::new(0,0,'E'));
    e.execute("FL");
    assert_eq!(e.query(), Pose::new(1,0,'N'));
}

#[test]
fn test_bfl_e() {
    let mut e = Executor::with_pose(Pose::new(0,0,'E'));
    e.execute("BFL");
    assert_eq!(e.query(), Pose::new(-1,0,'S'));
}