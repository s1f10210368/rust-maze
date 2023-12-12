use super::*;

//キー入力によって極座標カメラの位置を更新する
pub fn from_keyboard
(   mut q_camera: Query<&mut OrbitCamera>,
    time: Res<Time>,
    inkey: Res<Input<KeyCode>>,
)
{   let Ok ( mut camera ) = q_camera.get_single_mut() else { return };
    let orbit = &mut camera.orbit;

    let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間

    for keycode in inkey.get_pressed()
    {   match keycode
        {   KeyCode::Z =>
                orbit.r = ( orbit.r + time_delta ).min( ORBIT_CAMERA_MAX_R ),
            KeyCode::X =>
                orbit.r = ( orbit.r - time_delta ).max( ORBIT_CAMERA_MIN_R ),
            KeyCode::Up =>
                orbit.theta = ( orbit.theta + time_delta ).min( ORBIT_CAMERA_MAX_THETA ),
            KeyCode::Down =>
                orbit.theta = ( orbit.theta - time_delta ).max( ORBIT_CAMERA_MIN_THETA ),
            KeyCode::Right =>
            {   orbit.phi -= time_delta;
                orbit.phi += if orbit.phi < 0.0 { TAU } else { 0.0 };
            }
            KeyCode::Left =>
            {   orbit.phi += time_delta;
                orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
            }
            _ => (),
        }
    }
}