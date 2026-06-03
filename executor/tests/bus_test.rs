use executor::{BusExecutor, Pose};

// M 指令
#[test]
fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("M");
    let expected_pose = Pose::new(1, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_1_given_command_is_bm_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("BM");
    let expected_pose = Pose::new(-1, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_2_given_command_is_fm_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("FM");
    let expected_pose = Pose::new(2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_2_given_command_is_fbm_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("FBM");
    let expected_pose = Pose::new(-2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

// L 指令
#[test]
fn should_return_x_plus_1_and_facing_n_given_command_is_l_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("L");
    // 前进1格(x+1), 左转(朝N)
    let expected_pose = Pose::new(1, 0, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_1_and_facing_s_given_command_is_bl_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("BL");
    // 后退1格(x-1), 右转(朝S)
    let expected_pose = Pose::new(-1, 0, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_2_and_facing_n_given_command_is_fl_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("FL");
    // 前进1格+前进1格(x+2), 左转(朝N)
    let expected_pose = Pose::new(2, 0, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_2_and_facing_s_given_command_is_fbl_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("FBL");
    // 后退1格+后退1格(x-2), 右转(朝S)
    let expected_pose = Pose::new(-2, 0, 'S');
    assert_eq!(expected_pose, executor.query());
}

// R 指令
#[test]
fn should_return_x_plus_1_and_facing_s_given_command_is_r_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("R");
    // 前进1格(x+1), 右转(朝S)
    let expected_pose = Pose::new(1, 0, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_1_and_facing_n_given_command_is_br_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("BR");
    // 后退1格(x-1), 左转(朝N)
    let expected_pose = Pose::new(-1, 0, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_2_and_facing_s_given_command_is_fr_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("FR");
    // 前进1格+前进1格(x+2), 右转(朝S)
    let expected_pose = Pose::new(2, 0, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_2_and_facing_n_given_command_is_fbr_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = BusExecutor::with_pose(original_pose);
    executor.execute("FBR");
    // 后退1格+后退1格(x-2), 左转(朝N)
    let expected_pose = Pose::new(-2, 0, 'N');
    assert_eq!(expected_pose, executor.query());
}