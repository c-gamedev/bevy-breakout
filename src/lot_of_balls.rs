//1 - run `cargo add rand`
//2 - Replace the ball setup for this code

    use rand::prelude::*;
    // Initialize the random number generator
    let mut rng = thread_rng();

    for _ in 0..100_000 {
        // Generate random initial direction
        let random_angle = rng.gen_range(0.0..std::f32::consts::TAU); // TAU is 2*PI
        let random_direction = Vec2::new(random_angle.cos(), random_angle.sin());
        let random_color = Color::rgba(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            1.0, // alpha value, you can randomize this too if you want
        );

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: BALL_STARTING_POSITION,
                    ..Default::default()
                },
                sprite: Sprite {
                    color: random_color,
                    custom_size: Some(BALL_SIZE),
                    ..Default::default()
                },
                texture: ball_tex.clone(),
                ..Default::default()
            },
            Ball { size: BALL_SIZE },
            Velocity(BALL_SPEED * random_direction),
        ));
    }