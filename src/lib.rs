mod fflib {
    // --- FIGURY GEOMETRYCZNE ---

    /// Pole prostokąta
    pub fn rectangle_area(length: f64, width: f64) -> f64 {
        length * width
    }

    /// Pole trójkąta
    pub fn triangle_area(base: f64, height: f64) -> f64 {
        (base * height) / 2.0
    }

    /// Pole trapezu
    pub fn trapezoid_area(base1: f64, base2: f64, height: f64) -> f64 {
        ((base1 + base2) * height) / 2.0
    }

    /// Pole rombu
    pub fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
        (diagonal1 * diagonal2) / 2.0
    }

    /// Obwód koła
    pub fn circle_circumference(radius: f64) -> f64 {
        2.0 * std::f64::consts::PI * radius
    }

    /// Pole koła
    pub fn circle_area(radius: f64) -> f64 {
        std::f64::consts::PI * radius.powi(2)
    }

    // --- PODSTAWOWE OPERACJE MATEMATYCZNE ---

    /// Silnia (rekurencyjna)
    pub fn factorial(n: u64) -> u64 {
        if n == 0 { 1 } else { n * factorial(n - 1) }
    }

    /// Potęgowanie (x^y)
    pub fn power(base: f64, exponent: f64) -> f64 {
        base.powf(exponent)
    }

    /// Pierwiastek kwadratowy
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    /// Logarytm naturalny
    pub fn ln(x: f64) -> f64 {
        x.ln()
    }

    /// Sinus (radiany)
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    /// Cosinus (radiany)
    pub fn cos(x: f64) -> f64 {
        x.cos()
    }

    /// Tangens (radiany)
    pub fn tan(x: f64) -> f64 {
        x.tan()
    }

    /// Maksimum dwóch liczb
    pub fn max(a: f64, b: f64) -> f64 {
        a.max(b)
    }

    /// Minimum dwóch liczb
    pub fn min(a: f64, b: f64) -> f64 {
        a.min(b)
    }

    /// Suma elementów tablicy
    pub fn sum(arr: &[f64]) -> f64 {
        arr.iter().sum()
    }

    /// Średnia elementów tablicy
    pub fn average(arr: &[f64]) -> f64 {
        let sum: f64 = arr.iter().sum();
        sum / arr.len() as f64
    }
}
