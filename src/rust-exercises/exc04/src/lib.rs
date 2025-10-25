//! # Calculus Modülü
//!
//! Bu modül, temel matematiksel işlemleri gerçekleştiren fonksiyonlar içerir.
//! Örnek olarak, türev ve integral hesaplamaları için fonksiyonlar sağlar.
//!
//! # Functions
//! 
//! - `derivative`: Verilen bir fonksiyonun türevini yaklaşık olarak hesaplar.
//! - `integral`: Verilen bir fonksiyonun belirli bir aralıktaki integralini yaklaşık olarak hesaplar.
//! 
//! # Examples
//! 
//! Aşağıda, bu modüldeki fonksiyonların nasıl kullanılacağına dair örnekler verilmiştir.
//! 
//! ```rust
//! use exc04::calculus::{derivative, integral};
//! 
//! // Türev hesaplama örneği
//! let f = |x: f64| x.powi(2);
//! let deriv_at_3 = derivative(f, 3.0, 1e-7);
//! assert!((deriv_at_3 - 6.0).abs() < 1e-5);
//! 
//! // Integral hesaplama örneği
//! let f = |x: f64| x;
//! let integral_result = integral(f, 0.0, 1.0, 1000);
//! assert!((integral_result - 0.5).abs() < 1e-5);
//! ```

pub mod calculus;
