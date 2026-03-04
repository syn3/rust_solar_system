//! Интеграционные тесты физического движка.
//! Запускаются через: cargo test

use macroquad::prelude::*;

mod test_utils {
    use macroquad::prelude::*;

    /// Создаёт тело для тестов
    pub fn create_test_body(
        name: &str,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        mass: f32,
        radius: f32,
    ) -> super::Body {
        Body::new(name, Vec2::new(x, y), Vec2::new(vx, vy), mass, radius, WHITE)
    }
}

use test_utils::*;

mod physics {
    use super::super::physics::body::Body;
    use super::super::physics::world::{World, G};
    use macroquad::prelude::*;

    #[test]
    fn test_world_new() {
        let world = World::new();
        assert!(world.bodies.is_empty());
    }

    #[test]
    fn test_world_add_body() {
        let mut world = World::new();
        let body = Body::new("Test", Vec2::ZERO, Vec2::ZERO, 1.0, 1.0, WHITE);
        world.add_body(body);
        assert_eq!(world.bodies.len(), 1);
        assert_eq!(world.bodies[0].name, "Test");
    }

    #[test]
    fn test_body_speed() {
        let body = Body::new(
            "Test",
            Vec2::ZERO,
            Vec2::new(3.0, 4.0),
            1.0,
            1.0,
            WHITE,
        );
        // 3-4-5 треугольник
        assert!((body.speed() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_body_with_nasa() {
        use super::super::nasa_data::NasaData;
        
        let body = Body::new("Earth", Vec2::ZERO, Vec2::ZERO, 1.0, 1.0, BLUE)
            .with_nasa(NasaData::earth());
        
        assert!(body.nasa_data.is_some());
        let nasa = body.nasa_data.unwrap();
        assert!((nasa.mass_kg - 5.972e24).abs() < 1e20);
    }

    #[test]
    fn test_compute_accelerations_single_body() {
        let mut world = World::new();
        
        // Солнце в центре
        world.add_body(Body::new("Sun", Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, YELLOW));
        
        // Планета на расстоянии 100 единиц
        let mut planet = Body::new("Planet", Vec2::new(100.0, 0.0), Vec2::ZERO, 1.0, 5.0, WHITE);
        planet.velocity = Vec2::new(0.0, (G * 1000.0 / 100.0).sqrt()); // круговая орбита
        world.add_body(planet);
        
        world.compute_accelerations();
        
        // Ускорение направлено к Солнцу (отрицательное по X)
        let planet_accel = world.bodies[1].acceleration;
        assert!(planet_accel.x < 0.0);
        assert!(planet_accel.y.abs() < 1e-6);
    }

    #[test]
    fn test_step_preserves_momentum() {
        let mut world = World::new();
        
        // Два тела с противоположными импульсами
        world.add_body(Body::new("Sun", Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, YELLOW));
        world.add_body(Body::new("A", Vec2::new(50.0, 0.0), Vec2::new(0.0, 1.0), 1.0, 1.0, WHITE));
        world.add_body(Body::new("B", Vec2::new(-50.0, 0.0), Vec2::new(0.0, -1.0), 1.0, 1.0, WHITE));
        
        world.compute_accelerations();
        
        let initial_momentum = world.bodies[1].velocity * world.bodies[1].mass 
            + world.bodies[2].velocity * world.bodies[2].mass;
        
        world.step(0.001);
        
        let final_momentum = world.bodies[1].velocity * world.bodies[1].mass 
            + world.bodies[2].velocity * world.bodies[2].mass;
        
        // Импульс должен сохраняться (с точностью до численных ошибок)
        assert!((initial_momentum.x - final_momentum.x).abs() < 0.01);
        assert!((initial_momentum.y - final_momentum.y).abs() < 0.01);
    }

    #[test]
    fn test_step_updates_position() {
        let mut world = World::new();
        world.add_body(Body::new("Sun", Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, YELLOW));
        world.add_body(Body::new("Planet", Vec2::new(100.0, 0.0), Vec2::new(0.0, 1.0), 1.0, 1.0, WHITE));
        
        let initial_pos = world.bodies[1].position;
        world.compute_accelerations();
        world.step(0.01);
        
        // Позиция должна измениться
        assert!(world.bodies[1].position != initial_pos);
    }

    #[test]
    fn test_trail_records_positions() {
        let mut world = World::new();
        world.add_body(Body::new("Sun", Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, YELLOW));
        world.add_body(Body::new("Planet", Vec2::new(100.0, 0.0), Vec2::new(0.0, 1.0), 1.0, 1.0, WHITE));
        
        world.compute_accelerations();
        
        // Делаем несколько шагов
        for _ in 0..10 {
            world.step(0.01);
        }
        
        // Trail должен содержать записи
        assert!(world.bodies[1].trail.len() > 0);
    }

    #[test]
    fn test_trail_max_length() {
        let mut world = World::new();
        world.add_body(Body::new("Sun", Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, YELLOW));
        world.add_body(Body::new("Planet", Vec2::new(100.0, 0.0), Vec2::new(0.0, 1.0), 1.0, 1.0, WHITE));
        
        world.compute_accelerations();
        
        // Делаем много шагов
        for _ in 0..1000 {
            world.step(0.01);
        }
        
        // Trail должен быть ограничен
        assert!(world.bodies[1].trail.len() <= 500);
    }

    #[test]
    fn test_total_energy_calculation() {
        let mut world = World::new();
        
        // Солнце
        world.add_body(Body::new("Sun", Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, YELLOW));
        
        // Планета на круговой орбите
        let r = 100.0;
        let v = (G * 1000.0 / r).sqrt();
        let planet = Body::new("Planet", Vec2::new(r, 0.0), Vec2::new(0.0, v), 1.0, 1.0, WHITE);
        world.add_body(planet);
        
        let energy = world.total_energy();
        
        // Энергия должна быть отрицательной (связанная система)
        assert!(energy < 0.0);
    }

    #[test]
    fn test_energy_conservation_approximate() {
        let mut world = World::new();
        
        // Простая система: Солнце + планета
        world.add_body(Body::new("Sun", Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, YELLOW));
        
        let r = 100.0;
        let v = (G * 1000.0 / r).sqrt();
        world.add_body(Body::new("Planet", Vec2::new(r, 0.0), Vec2::new(0.0, v), 1.0, 1.0, WHITE));
        
        world.compute_accelerations();
        let initial_energy = world.total_energy();
        
        // Несколько шагов симуляции
        for _ in 0..100 {
            world.step(0.01);
        }
        
        let final_energy = world.total_energy();
        
        // Энергия должна сохраняться (с точностью ~1% для leapfrog)
        let relative_diff = (initial_energy - final_energy).abs() / initial_energy.abs();
        assert!(relative_diff < 0.01, "Energy drift: {}%", relative_diff * 100.0);
    }

    #[test]
    fn test_circular_orbit_stability() {
        let mut world = World::new();
        
        // Солнце
        world.add_body(Body::new("Sun", Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, YELLOW));
        
        // Планета на круговой орбите
        let r = 100.0;
        let v = (G * 1000.0 / r).sqrt();
        world.add_body(Body::new("Planet", Vec2::new(r, 0.0), Vec2::new(0.0, v), 1.0, 1.0, WHITE));
        
        world.compute_accelerations();
        
        // Запоминаем начальную позицию
        let initial_pos = world.bodies[1].position;
        
        // Симулируем один полный оборот (T = 2*pi*r/v)
        let period = 2.0 * std::f32::consts::PI * r / v;
        let steps = (period / 0.01) as usize;
        
        for _ in 0..steps {
            world.step(0.01);
        }
        
        // Планета должна вернуться примерно в начальную позицию
        let final_pos = world.bodies[1].position;
        let dist = (final_pos - initial_pos).length();
        
        assert!(dist < 1.0, "Orbit not closed: distance = {}", dist);
    }

    #[test]
    fn test_earth_moon_system() {
        let mut world = World::new();
        
        // Солнце
        world.add_body(Body::new("Sun", Vec2::ZERO, Vec2::ZERO, 100000.0, 25.0, YELLOW));
        
        // Земля
        let earth = Body::new("Earth", Vec2::new(85.0, 0.0), Vec2::new(0.0, 10.8), 1.5, 5.5, BLUE);
        world.add_body(earth);
        
        // Луна (орбита вокруг Земли)
        let moon = Body::new("Moon", Vec2::new(97.0, 0.0), Vec2::new(0.0, 12.5), 0.1, 2.0, GRAY);
        world.add_body(moon);
        
        world.compute_accelerations();
        
        // Луна должна испытывать притяжение и от Солнца, и от Земли
        let moon_accel = world.bodies[2].acceleration;
        assert!(moon_accel.length() > 0.0);
    }
}

mod nasa_data {
    use super::super::nasa_data::NasaData;

    #[test]
    fn test_sun_data() {
        let sun = NasaData::sun();
        assert!(sun.mass_kg > 1e30);
        assert!(sun.radius_km > 600_000.0);
        assert!(sun.semimajor_axis_au.is_none());
    }

    #[test]
    fn test_earth_data() {
        let earth = NasaData::earth();
        assert!((earth.mass_kg - 5.972e24).abs() < 1e22);
        assert!((earth.radius_km - 6_371.0).abs() < 1.0);
        assert_eq!(earth.semimajor_axis_au, Some(1.0));
        assert_eq!(earth.orbital_period_years, Some(1.0));
    }

    #[test]
    fn test_planet_order() {
        let planets = [
            NasaData::mercury(),
            NasaData::venus(),
            NasaData::earth(),
            NasaData::mars(),
            NasaData::jupiter(),
            NasaData::saturn(),
            NasaData::uranus(),
            NasaData::neptune(),
        ];
        
        // Проверяем, что орбитальный период растёт
        for i in 1..planets.len() {
            let prev_period = planets[i - 1].orbital_period_years.unwrap();
            let curr_period = planets[i].orbital_period_years.unwrap();
            assert!(curr_period > prev_period, "Planet {} has smaller period than previous", i);
        }
    }

    #[test]
    fn test_mass_display_sun() {
        let sun = NasaData::sun();
        let display = sun.mass_display();
        assert!(display.contains("10²⁷"));
    }

    #[test]
    fn test_mass_display_earth() {
        let earth = NasaData::earth();
        let display = earth.mass_display();
        assert!(display.contains("10²⁴"));
    }

    #[test]
    fn test_mass_display_moon() {
        let moon = NasaData {
            mass_kg: 7.342e22,
            radius_km: 1_737.4,
            semimajor_axis_au: None,
            orbital_period_years: None,
            orbital_velocity_km_s: None,
        };
        let display = moon.mass_display();
        assert!(display.contains("10²²"));
    }
}
