/// Verilen bir fonksiyonun türevini yaklaşık olarak hesaplar.
///
/// # Argümanlar
/// * `f` - Türevini almak istediğimiz fonksiyon.
/// * `x` - Türevini hesaplamak istediğimiz nokta.
/// * `h` - Küçük bir değer, türev hesaplamasında kullanılır (varsayılan: 1e-7).
/// # Dönüş Değeri
/// * `f` fonksiyonunun `x` noktasındaki yaklaşık türevi.
pub fn derivative<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// Verilen bir fonksiyonun belirli bir aralıktaki integralini yaklaşık olarak hesaplar.
///
/// # Argümanlar
/// * `f` - İntegralini almak istediğimiz fonksiyon.
/// * `a` - İntegral başlangıç noktası.
/// * `b` - İntegral bitiş noktası.
/// * `n` - İntegral hesaplamasında kullanılacak dikdörtgen sayısı (varsayılan: 1000).
/// # Dönüş Değeri
/// * `f` fonksiyonunun `[a, b]` aralığındaki yaklaşık integrali.
pub fn integral<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let width = (b - a) / (n as f64);
    let mut total_area = 0.0;

    for i in 0..n {
        let x = a + (i as f64 + 0.5) * width;
        total_area += f(x) * width;
    }

    total_area
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_derivative() {
        let f = |x: f64| x.powi(2);
        let deriv_at_3 = derivative(f, 3.0, 1e-7);
        assert!((deriv_at_3 - 6.0).abs() < 1e-5);
    }

    #[test]
    fn test_integral() {
        let f = |x: f64| x;
        let integral_result = integral(f, 0.0, 1.0, 1000);
        assert!((integral_result - 0.5).abs() < 1e-5);
    }
}
