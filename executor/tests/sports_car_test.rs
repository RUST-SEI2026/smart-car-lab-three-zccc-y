use executor::{Pose, SportsCarExecutor};

// M 指令
#[test]
fn should_return_x_plus_2_given_command_is_m_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("M");
    let expected_pose = Pose::new(2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_mius_2_given_command_is_bm_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("BM");
    let expected_pose = Pose::new(-2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_4_given_command_is_fm_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("FM");
    let expected_pose = Pose::new(4, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_mius_4_given_command_is_fbm_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("FBM");
    let expected_pose = Pose::new(-4, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

// L 指令
#[test]
fn should_return_y_plus_1_and_facing_n_given_command_is_l_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("L");
    let expected_pose = Pose::new(0, 1, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_y_plus_1_and_facing_s_given_command_is_bl_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("BL");
    // B: 右转90° → 朝S, 后退1格 → 朝S后退是y+1
    let expected_pose = Pose::new(0, 1, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_1_y_plus_1_and_facing_n_given_command_is_fl_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("FL");
    let expected_pose = Pose::new(1, 1, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_mius_1_y_plus_1_and_facing_s_given_command_is_fbl_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("FBL");
    // B&F: 后退1格(x-1) + 右转(朝S) + 后退1格(y+1)
    let expected_pose = Pose::new(-1, 1, 'S');
    assert_eq!(expected_pose, executor.query());
}

// R 指令
#[test]
fn should_return_y_mius_1_and_facing_s_given_command_is_r_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("R");
    let expected_pose = Pose::new(0, -1, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_y_mius_1_and_facing_n_given_command_is_br_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("BR");
    // B: 左转90° → 朝N, 后退1格 → 朝N后退是y-1
    let expected_pose = Pose::new(0, -1, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_1_y_mius_1_and_facing_s_given_command_is_fr_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("FR");
    let expected_pose = Pose::new(1, -1, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_mius_1_y_mius_1_and_facing_n_given_command_is_fbr_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = SportsCarExecutor::with_pose(original_pose);
    executor.execute("FBR");
    // B&F: 后退1格(x-1) + 左转(朝N) + 后退1格(y-1)
    let expected_pose = Pose::new(-1, -1, 'N');
    assert_eq!(expected_pose, executor.query());
}