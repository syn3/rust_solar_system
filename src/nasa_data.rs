//! Реальные параметры тел Солнечной системы (данные NASA).
//! Источник: https://nssdc.gsfc.nasa.gov/planetary/factsheet/

/// Реальные физические параметры небесного тела
#[derive(Clone, Debug)]
pub struct NasaData {
    /// Масса в кг
    pub mass_kg: f64,
    /// Радиус в км
    pub radius_km: f64,
    /// Большая полуось орбиты в а.е. (астрономических единицах)
    pub semimajor_axis_au: Option<f64>,
    /// Орбитальный период в земных годах
    pub orbital_period_years: Option<f64>,
    /// Орбитальная скорость в км/с (средняя)
    pub orbital_velocity_km_s: Option<f64>,
}

impl NasaData {
    pub fn sun() -> Self {
        Self {
            mass_kg: 1.989e30,
            radius_km: 696_340.0,
            semimajor_axis_au: None,
            orbital_period_years: None,
            orbital_velocity_km_s: None,
        }
    }

    pub fn mercury() -> Self {
        Self {
            mass_kg: 3.285e23,
            radius_km: 2_439.7,
            semimajor_axis_au: Some(0.387),
            orbital_period_years: Some(0.241),
            orbital_velocity_km_s: Some(47.4),
        }
    }

    pub fn venus() -> Self {
        Self {
            mass_kg: 4.867e24,
            radius_km: 6_051.8,
            semimajor_axis_au: Some(0.723),
            orbital_period_years: Some(0.615),
            orbital_velocity_km_s: Some(35.0),
        }
    }

    pub fn earth() -> Self {
        Self {
            mass_kg: 5.972e24,
            radius_km: 6_371.0,
            semimajor_axis_au: Some(1.0),
            orbital_period_years: Some(1.0),
            orbital_velocity_km_s: Some(29.8),
        }
    }

    pub fn mars() -> Self {
        Self {
            mass_kg: 6.39e23,
            radius_km: 3_389.5,
            semimajor_axis_au: Some(1.524),
            orbital_period_years: Some(1.881),
            orbital_velocity_km_s: Some(24.1),
        }
    }

    pub fn jupiter() -> Self {
        Self {
            mass_kg: 1.898e27,
            radius_km: 69_911.0,
            semimajor_axis_au: Some(5.203),
            orbital_period_years: Some(11.86),
            orbital_velocity_km_s: Some(13.1),
        }
    }

    pub fn saturn() -> Self {
        Self {
            mass_kg: 5.683e26,
            radius_km: 58_232.0,
            semimajor_axis_au: Some(9.537),
            orbital_period_years: Some(29.45),
            orbital_velocity_km_s: Some(9.7),
        }
    }

    pub fn uranus() -> Self {
        Self {
            mass_kg: 8.681e25,
            radius_km: 25_362.0,
            semimajor_axis_au: Some(19.19),
            orbital_period_years: Some(84.02),
            orbital_velocity_km_s: Some(6.8),
        }
    }

    pub fn neptune() -> Self {
        Self {
            mass_kg: 1.024e26,
            radius_km: 24_622.0,
            semimajor_axis_au: Some(30.07),
            orbital_period_years: Some(164.8),
            orbital_velocity_km_s: Some(5.4),
        }
    }

    /// Форматирование массы для отображения
    pub fn mass_display(&self) -> String {
        if self.mass_kg >= 1e27 {
            format!("{:.2}×10²⁷ kg", self.mass_kg / 1e27)
        } else if self.mass_kg >= 1e24 {
            format!("{:.2}×10²⁴ kg", self.mass_kg / 1e24)
        } else if self.mass_kg >= 1e21 {
            format!("{:.2}×10²¹ kg", self.mass_kg / 1e21)
        } else {
            format!("{:.2e} kg", self.mass_kg)
        }
    }
}
