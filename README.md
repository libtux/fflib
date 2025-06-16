# fflib

`fflib` to prosta biblioteka w Rust zawierająca podstawowe funkcje do obliczeń geometrycznych oraz matematycznych.

---

## Funkcje

### Geometryczne
- `rectangle_area(length: f64, width: f64) -> f64` – pole prostokąta
- `triangle_area(base: f64, height: f64) -> f64` – pole trójkąta
- `trapezoid_area(base1: f64, base2: f64, height: f64) -> f64` – pole trapezu
- `rhombus_area(diagonal1: f64, diagonal2: f64) -> f64` – pole rombu
- `circle_area(radius: f64) -> f64` – pole koła
- `circle_circumference(radius: f64) -> f64` – obwód koła

### Matematyczne
- `factorial(n: u64) -> u64` – silnia
- `power(base: f64, exponent: f64) -> f64` – potęgowanie
- `sqrt(x: f64) -> f64` – pierwiastek kwadratowy
- `ln(x: f64) -> f64` – logarytm naturalny
- `sin(x: f64) -> f64` – sinus (radiany)
- `cos(x: f64) -> f64` – cosinus (radiany)
- `tan(x: f64) -> f64` – tangens (radiany)
- `max(a: f64, b: f64) -> f64` – maksimum
- `min(a: f64, b: f64) -> f64` – minimum
- `sum(arr: &[f64]) -> f64` – suma elementów tablicy
- `average(arr: &[f64]) -> f64` – średnia elementów tablicy

---

## Przykład użycia

```rust
fn main() {
    let pole_kwadratu = fflib::rectangle_area(4.0, 4.0);
    println!("Pole kwadratu: {}", pole_kwadratu);

    let sin_45 = fflib::sin(std::f64::consts::FRAC_PI_4);
    println!("Sin(45°): {}", sin_45);

    let silnia_5 = fflib::factorial(5);
    println!("5! = {}", silnia_5);
}
